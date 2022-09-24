use super::{
    block::BlockWidget, button::ButtonWidget, select_box::SelectBoxWidget, FixedSizeWidget,
    VariableSizeWidget, Widget,
};
use crate::{
    app::App,
    asset::{ButtonKind, IconId, Text},
    event::Event,
};
use pagurus::{
    failure::OrFail,
    spatial::{Position, Region, Size},
    Result,
};
use pagurus_game_std::image::Canvas;

#[derive(Debug)]
pub struct EraseToolWidget {
    region: Region,
    tools: BlockWidget<SelectBoxWidget>,
}

impl EraseToolWidget {
    pub fn new(_app: &App) -> Result<Self> {
        let mut buttons = vec![
            ButtonWidget::new(ButtonKind::Basic, IconId::Erase),
            ButtonWidget::new(ButtonKind::Basic, IconId::ScissorLasso),
            ButtonWidget::new(ButtonKind::Basic, IconId::ScissorRectangle),
        ];
        buttons[0].set_kind(ButtonKind::BasicPressed);
        Ok(Self {
            region: Region::default(),
            tools: BlockWidget::new(
                "ERASING TOOL".parse::<Text>().or_fail()?,
                SelectBoxWidget::new(buttons, 0).or_fail()?,
            ),
        })
    }
}

impl Widget for EraseToolWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.tools.render(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.tools.handle_event(app, event).or_fail()?;
        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![&mut self.tools]
    }
}

impl FixedSizeWidget for EraseToolWidget {
    fn requiring_size(&self, app: &App) -> Size {
        self.tools.requiring_size(app)
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));
        self.tools.set_region(app, self.region);
    }
}
