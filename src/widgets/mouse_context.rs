use ratatui::layout::{Position, Rect};

pub trait MouseContext {
    fn is_over(&self, position: Position) -> bool;

    fn set_context(&mut self, rect: Rect);
}