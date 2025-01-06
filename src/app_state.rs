use bevy::prelude::*;

use bevy_ratatui::event::{self, KeyEvent, MouseEvent};
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

#[derive(Debug, Clone, Event, PartialEq, Eq)]
pub enum HomeEvent {
    MouseEvent(MouseEvent),
    KeyEvent(KeyEvent),
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

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HomeEvent>()
            .add_systems(PreUpdate, home_events_handler);
    }
}

fn home_events_handler(
    mut app_state: ResMut<State<AppState>>,
    mut home_events: EventReader<HomeEvent>,
) {
    let home_event = home_events.read();
    for event in home_event {
        match  event {
            HomeEvent::MouseEvent(event) =>{
                
            }
            HomeEvent::KeyEvent(event) =>{
    
            }
        }
    }
}
