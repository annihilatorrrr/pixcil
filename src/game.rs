use crate::{
    app::App,
    event::{Event, InputId},
    model::Models,
    window::{main::MainWindow, Window},
};
use pagurus::{
    event::Event as PagurusEvent, event::WindowEvent as PagurusWindowEvent, failure::OrFail,
    video::VideoFrame, Game, Result, System,
};
use pagurus_game_std::{image::Canvas, logger::Logger};

pagurus_game_std::export_wasm_functions!(PixcilGame);

#[derive(Default)]
pub struct PixcilGame {
    logger: Logger,
    video_frame: VideoFrame,
    windows: Vec<Box<dyn Window>>,
    app: Option<App>,
}

impl PixcilGame {
    fn handle_event_without_log_flush<S: System>(
        &mut self,
        system: &mut S,
        event: PagurusEvent,
    ) -> Result<bool> {
        let app = self.app.as_mut().or_fail()?;

        match event {
            PagurusEvent::Terminating => return Ok(false),
            PagurusEvent::Window(PagurusWindowEvent::RedrawNeeded { size }) => {
                app.request_redraw(size.to_region());
                if size != app.screen_size() {
                    self.video_frame = VideoFrame::new(system.video_frame_spec(size));
                    app.set_screen_size(size);
                    for window in &mut self.windows {
                        window.handle_screen_resized(app).or_fail()?;
                    }
                }
            }
            _ => {}
        };

        let event = Event::from_pagurus_event(app, event);
        self.handle_pixcil_event(system, event).or_fail()?;

        Ok(true)
    }

    fn handle_pixcil_event<S: System>(
        &mut self,
        system: &mut S,
        event: Option<Event>,
    ) -> Result<()> {
        let app = self.app.as_mut().or_fail()?;
        if let Some(mut event) = event {
            let mut terminated = false;
            for window in self.windows.iter_mut().rev() {
                window.handle_event(app, &mut event).or_fail()?;
                if window.is_terminated() {
                    terminated = true;
                    app.request_redraw(window.region());
                }
            }

            if terminated {
                self.windows = std::mem::take(&mut self.windows)
                    .into_iter()
                    .filter(|w| !w.is_terminated())
                    .collect();
            }
        }
        self.windows.extend(app.take_spawned_windows());
        app.set_pending_timeouts(system);

        // TODO: Handle FPS (avoid too many renderings during a short term)
        self.render(system).or_fail()?;
        Ok(())
    }

    fn render<S: System>(&mut self, system: &mut S) -> Result<()> {
        let mut canvas = Canvas::new(&mut self.video_frame);
        let app = self.app.as_mut().or_fail()?;

        // TODO: Reduce redundant redraws
        for region in app.take_redraw_requests() {
            let mut canvas = canvas.mask_region(region);
            for window in &mut self.windows {
                window.render(app, &mut canvas);
            }
        }

        system.video_draw(self.video_frame.as_ref());

        Ok(())
    }
}

impl<S: System> Game<S> for PixcilGame {
    fn initialize(&mut self, system: &mut S) -> Result<()> {
        self.logger = Logger::init(log::Level::Debug).or_fail()?;
        self.windows.push(Box::new(MainWindow::new()));
        self.app = Some(App::new().or_fail()?);
        self.logger.flush(system);
        Ok(())
    }

    fn handle_event(&mut self, system: &mut S, event: PagurusEvent) -> Result<bool> {
        let result = self.handle_event_without_log_flush(system, event).or_fail();
        self.logger.flush(system);
        result
    }

    fn query(&mut self, _system: &mut S, name: &str) -> Result<Vec<u8>> {
        match name {
            "nextIoRequest" => {
                if let Some(req) = self.app.as_mut().or_fail()?.dequeue_io_request() {
                    Ok(serde_json::to_vec(&req).or_fail()?)
                } else {
                    Ok(Vec::new())
                }
            }
            "workspacePng" => {
                let app = self.app.as_ref().or_fail()?;
                let data = app.models().to_png(app).or_fail()?;
                Ok(data)
            }
            _ => Err(pagurus::failure::Failure::new(format!(
                "unknown query: {name:?}"
            ))),
        }
    }

    fn command(&mut self, system: &mut S, name: &str, data: &[u8]) -> Result<()> {
        match name {
            "notifyInputNumber" => {
                #[derive(serde::Deserialize)]
                struct Data {
                    id: InputId,
                    number: String,
                }
                let data: Data = serde_json::from_slice(data).or_fail()?;
                let event = Event::Input {
                    id: data.id,
                    text: data.number,
                };
                self.handle_pixcil_event(system, Some(event)).or_fail()?;
                Ok(())
            }
            "loadWorkspace" => {
                let app = self.app.as_mut().or_fail()?;
                *app.models_mut() = Models::from_png(data).or_fail()?;
                app.request_redraw(app.screen_size().to_region());
                self.handle_pixcil_event(system, None).or_fail()?;

                Ok(())
            }
            _ => Err(pagurus::failure::Failure::new(format!(
                "unknown command: {name:?}"
            ))),
        }
    }
}
