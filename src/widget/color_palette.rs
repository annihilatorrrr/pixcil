use super::{FixedSizeWidget, Widget, button::ButtonWidget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    canvas_ext::CanvasExt,
    event::Event,
    region_ext::RegionExt,
};
use orfail::{OrFail, Result};
use pagurus::{
    image::{Canvas, Rgba},
    spatial::{Contains, Position, Region, Size},
};
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct ColorPaletteWidget {
    region: Region,
    colors: Vec<Rgba>,
    buttons: Vec<ButtonWidget>,
}

impl ColorPaletteWidget {
    pub fn new(app: &App, width: u32) -> Self {
        let colors = Self::get_colors(app);
        let buttons = colors
            .iter()
            .map(|_| ButtonWidget::new(ButtonKind::Middle, IconId::Null))
            .collect();
        let height = ButtonWidget::new(ButtonKind::Middle, IconId::Null)
            .requiring_size(app)
            .height;
        Self {
            region: Region::new(Position::default(), Size::from_wh(width, height)),
            colors,
            buttons,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    fn get_colors(app: &App) -> Vec<Rgba> {
        let models = app.models();

        let mut colors = Vec::new();
        let mut color_set = HashSet::new();
        for (_, color) in models.pixel_canvas.raw_pixels() {
            if color_set.contains(&color) {
                continue;
            }
            color_set.insert(color);
            colors.push(color);
        }

        colors
    }

    fn render_color_label(&self, button: &ButtonWidget, color: Rgba, canvas: &mut Canvas) {
        let offset = button.state().offset(button.kind()).y;
        let mut label_region = button.region();
        label_region.position.x += 2;
        label_region.size.width -= 4;

        label_region.position.y += 2 + offset;
        label_region.size.height -= 4 + 4;

        canvas.fill_rectangle(label_region.without_margin(2), color.into());
    }
}

impl Widget for ColorPaletteWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        let mut canvas = canvas.mask_region(self.region);
        for (button, &color) in self.buttons.iter().zip(self.colors.iter()) {
            button.render(app, &mut canvas);
            self.render_color_label(button, color, &mut canvas);
        }
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        if let Some(position) = event.position() {
            if !self.region.contains(&position) {
                return Ok(());
            }
        }

        for (button, &color) in self.buttons.iter_mut().zip(self.colors.iter()) {
            button.handle_event(app, event).or_fail()?;
            if button.take_clicked(app) {
                app.models_mut().config.color.set(color);
                break;
            }
        }

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        let mut children = Vec::new();
        children.extend(self.buttons.iter_mut().map(|b| b as &mut dyn Widget));
        children
    }
}

impl FixedSizeWidget for ColorPaletteWidget {
    fn requiring_size(&self, _app: &App) -> Size {
        self.region.size
    }

    fn set_position(&mut self, app: &App, position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        let mut position = position;
        for color in &mut self.buttons {
            color.set_position(app, position);
            position.x += color.requiring_size(app).width as i32;
        }
    }
}
