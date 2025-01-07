use bevy::prelude::*;

use bevy_ratatui::{error::exit_on_error, event::{self, KeyEvent, MouseEvent}, terminal::RatatuiContext};
use crossterm::event::KeyEventKind;
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
    Options,
}

#[derive(Debug, Clone, Event, PartialEq, Eq)]
pub enum HomeEvent {
    MouseEvent(MouseEvent),
    KeyEvent(KeyEvent),
}

impl WidgetRef for AppState {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {        
        match self {
            AppState::Home | AppState::Options => {
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
            .init_state::<AppState>()
            .add_systems(PreUpdate, home_events_handler)
            .add_systems(Update, render_home.pipe(exit_on_error));
    }
}

fn render_home(
    app_state: Res<State<AppState>>,
    mut context: ResMut<RatatuiContext>,
) -> color_eyre::Result<()> {
    context.draw(|frame| {
        let area = frame.area();
        frame.render_widget(app_state.get(), area);
    })?;
    Ok(())
}

fn home_events_handler(    
    mut home_events: EventReader<HomeEvent>,
    mut app_exit: EventWriter<AppExit>,
) {
    for event in home_events.read() {
        match event {
            HomeEvent::MouseEvent(mouse_event) => {
                // do nothing
            }
            HomeEvent::KeyEvent(key_event) => {
                match key_event.kind {
                    KeyEventKind::Release => {
                        match key_event.code {
                            crossterm::event::KeyCode::Char('q') | crossterm::event::KeyCode::Esc => {
                                app_exit.send_default();
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
