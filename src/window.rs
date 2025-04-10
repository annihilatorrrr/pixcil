use crate::{app::App, event::Event};
use pagurus::image::Canvas;
use pagurus::{Result, spatial::Region};

pub mod color_selector;
pub mod config;
pub mod main;
pub mod widget;

pub trait Window: 'static + std::fmt::Debug {
    fn region(&self) -> Region;
    fn render(&self, app: &App, canvas: &mut Canvas);
    fn is_terminated(&self) -> bool;
    fn handle_screen_resized(&mut self, app: &mut App) -> Result<()>;
    fn handle_event(&mut self, app: &mut App, event: &mut Event) -> Result<()>;
}
