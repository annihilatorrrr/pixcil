use super::{widget::WidgetWindow, Window};
use crate::{app::App, event::Event, widget::color_selector::ColorSelectorWidget};
use pagurus::image::Canvas;
use pagurus::{failure::OrFail, spatial::Region, Result};

#[derive(Debug)]
pub struct ColorSelectorWindow(WidgetWindow<ColorSelectorWidget>);

impl ColorSelectorWindow {
    pub fn new(app: &App) -> Self {
        Self(WidgetWindow::with_margin(ColorSelectorWidget::new(app), 8))
    }
}

impl Window for ColorSelectorWindow {
    fn region(&self) -> Region {
        self.0.region()
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.0.render(app, canvas);
    }

    fn is_terminated(&self) -> bool {
        self.0.is_terminated()
    }

    fn handle_screen_resized(&mut self, app: &mut App) -> Result<()> {
        self.0.handle_screen_resized(app).or_fail()
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.0.handle_event(app, event).or_fail()
    }
}
