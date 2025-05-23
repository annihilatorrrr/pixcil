use super::{FixedSizeWidget, Widget, button::ButtonWidget, size_box::SizeBoxWidget};
use crate::{
    app::App,
    asset::{ButtonKind, IconId},
    event::Event,
    pixel::PixelSize,
};
use orfail::{OrFail, Result};
use pagurus::image::Canvas;
use pagurus::spatial::{Position, Region, Size};

const MARGIN: u32 = 8;
const HALF_MARGIN: u32 = MARGIN / 2;

#[derive(Debug)]
pub struct PixelSizeWidget {
    region: Region,
    pixel_size: SizeBoxWidget,
    halve: ButtonWidget,
    double: ButtonWidget,
    frame: ButtonWidget,
}

impl PixelSizeWidget {
    pub fn new(app: &App) -> Self {
        let pixel_size = app.models().config.minimum_pixel_size.get();
        Self {
            region: Region::default(),
            pixel_size: SizeBoxWidget::new(pixel_size),
            halve: ButtonWidget::new(ButtonKind::Middle, IconId::Halve),
            double: ButtonWidget::new(ButtonKind::Middle, IconId::Double),
            frame: ButtonWidget::new(ButtonKind::Middle, IconId::UnitFrame),
        }
    }

    pub fn value(&self) -> PixelSize {
        self.pixel_size.value()
    }
}

impl Widget for PixelSizeWidget {
    fn region(&self) -> Region {
        self.region
    }

    fn render(&self, app: &App, canvas: &mut Canvas) {
        self.pixel_size.render_if_need(app, canvas);
        self.halve.render_if_need(app, canvas);
        self.double.render_if_need(app, canvas);
        self.frame.render_if_need(app, canvas);
    }

    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()> {
        self.pixel_size.handle_event(app, event).or_fail()?;

        self.halve.handle_event(app, event).or_fail()?;
        if self.halve.take_clicked(app) {
            self.pixel_size.set_value(app, self.pixel_size.value() / 2);
        }

        self.double.handle_event(app, event).or_fail()?;
        if self.double.take_clicked(app) {
            self.pixel_size.set_value(app, self.pixel_size.value() * 2);
        }

        self.frame.handle_event(app, event).or_fail()?;
        if self.frame.take_clicked(app) {
            if self.frame.icon() == IconId::UnitFrame {
                self.pixel_size
                    .set_value(app, app.models().config.frame.get_base_region().size());
            } else {
                self.pixel_size.set_value(app, PixelSize::square(1));
            }
        }
        if app.models().config.minimum_pixel_size.get()
            == app.models().config.frame.get_base_region().size()
        {
            self.frame.set_icon(app, IconId::UnitPixel);
        } else {
            self.frame.set_icon(app, IconId::UnitFrame);
        }

        Ok(())
    }

    fn children(&mut self) -> Vec<&mut dyn Widget> {
        vec![
            &mut self.pixel_size,
            &mut self.halve,
            &mut self.double,
            &mut self.frame,
        ]
    }
}

impl FixedSizeWidget for PixelSizeWidget {
    fn requiring_size(&self, app: &App) -> Size {
        let mut size = self.pixel_size.requiring_size(app);
        size.width += MARGIN + self.halve.requiring_size(app).width;
        size.width += HALF_MARGIN + self.double.requiring_size(app).width;
        size.width += MARGIN + self.frame.requiring_size(app).width;
        size
    }

    fn set_position(&mut self, app: &App, mut position: Position) {
        self.region = Region::new(position, self.requiring_size(app));

        self.pixel_size.set_position(app, position);
        position.x = self.pixel_size.region().end().x + MARGIN as i32;
        position.y += 4;

        self.halve.set_position(app, position);
        position.x = self.halve.region().end().x + HALF_MARGIN as i32;

        self.double.set_position(app, position);
        position.x = self.double.region().end().x + MARGIN as i32;

        self.frame.set_position(app, position);
    }
}
