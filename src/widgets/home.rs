use bevy::prelude::*;

use bevy_ratatui::{error::exit_on_error, event::{self, KeyEvent, MouseEvent}, terminal::RatatuiContext};
use crossterm::event::KeyEventKind;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{WidgetRef, Block, Borders},
};

use crate::states::app_state::{self, AppState};
use crate::states::home_state::HomeState;

pub struct HomeWidget;

#[derive(Debug, Clone, Event, PartialEq, Eq)]
pub enum HomeEvent {
    MouseEvent(MouseEvent),
    KeyEvent(KeyEvent),
}

impl WidgetRef for HomeWidget {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {        
        Block::default()
            .title("Home")
            .borders(Borders::ALL)
            .render_ref(area, buf);
    }
}

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HomeEvent>()            
            .add_systems(PreUpdate, home_events_handler)
            .add_systems(Update, render_home.pipe(exit_on_error));
    }
}

fn render_home(
    app_state: Res<State<AppState>>,
    mut context: ResMut<RatatuiContext>,
) -> color_eyre::Result<()> {
    let app_state = app_state.get();
    if app_state != &AppState::Home {
        return Ok(());
    }
    context.draw(|frame| {
        let area = frame.area();
        frame.render_widget_ref(HomeWidget, area);
    })?;
    Ok(())
}

fn home_events_handler(    
    mut app_state: ResMut<NextState<AppState>>,
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
                            crossterm::event::KeyCode::Char('q') => {
                                app_exit.send_default();
                            }
                            crossterm::event::KeyCode::Esc => {
                                app_state.set(AppState::Options);
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
