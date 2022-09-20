use super::{
    block::BlockWidget, number_box::NumberBoxWidget, toggle::ToggleWidget, FixedSizeWidget,
    VariableSizeWidget, Widget,
};
use crate::{app::App, event::Event};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

// - frame
//   - [o] frame preview on/off (switch)
//   - frame size (width / height sliders)
// - layer count (slider)
// - animation
//   - frame count (slider)
//   - fps (slider)
// - General
//   - unit size (slider)
//   - max undo history (select box)

#[derive(Debug)]
pub struct ConfigWidget {
    region: Region,
    frame_preview: BlockWidget<ToggleWidget>,
    frame_size: BlockWidget<NumberBoxWidget>,
}

impl Default for ConfigWidget {
    fn default() -> Self {
        Self {
            region: Region::default(),
            frame_preview: BlockWidget::new(
                "PREVIEW".parse().expect("unreachable"),
                ToggleWidget::default(),
            ),
            frame_size: BlockWidget::new(
                "SIZE".parse().expect("unreachable"),
                NumberBoxWidget::new(1, 64, 9999),
            ),
        }
    }
}

impl Widget for ConfigWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.frame_preview.render_if_need(app, canvas);
        self.frame_size.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.frame_preview.handle_event(app, event).or_fail()?;
        app.models_mut()
            .config
            .frame_preview
            .set(self.frame_preview.body().is_on());

        self.frame_size.handle_event(app, event).or_fail()?;
        // TODO: set model

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.frame_preview, &mut self.frame_size]
    }
}

impl FixedSizeWidget for ConfigWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let mut size = self.frame_preview.requiring_size(app);
        size.width += self.frame_size.requiring_size(app).width + 8;
        size
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut frame_preview_region = self.region;
        frame_preview_region.size.width = self.frame_preview.requiring_size(app).width;
        self.frame_preview.set_region(app, frame_preview_region);

        let mut frame_size_region = self.region;
        frame_size_region.position.x = frame_preview_region.end().x + 8;
        frame_size_region.size.width = self.frame_size.requiring_size(app).width;
        self.frame_size.set_region(app, frame_size_region);
    }
}
