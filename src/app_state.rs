use bevy::prelude::*;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{WidgetRef, Block, Borders},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Home,
}

impl WidgetRef for AppState {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        match self {
            AppState::Home => {
                Block::default()
                    .title("Home")
                    .borders(Borders::ALL)
                    .render_ref(area, buf);
            }
        }
    }
}