use std::collections::HashMap;

use bevy::{log, prelude::*};
use bevy_ratatui::{
    error::exit_on_error,
    event::{KeyEvent, MouseEvent},
    terminal::RatatuiContext,
};
use crossterm::event::{KeyEventKind, MouseEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Position},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, StatefulWidgetRef, WidgetRef},
};

use crate::states::{
    app_state::{self, AppState},
    options_state::OptionsState,
};

type Rect = ratatui::layout::Rect;

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OptionsEvent>()
            .init_resource::<RegisteredComponents>()
            .add_systems(PreUpdate, options_event_handler)
            .add_systems(Update, render_options.pipe(exit_on_error))
            .init_state::<OptionsState>();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Event)]
pub enum OptionsEvent {
    MouseEvent(MouseEvent),
    KeyEvent(KeyEvent),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum OptionComponents {
    NewGame,
    KeyBindings,
    Video,
    Audio,
    Back,
}

impl StatefulWidgetRef for OptionsState {
    type State = RegisteredComponents;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let sub_area = Layout::default()
            .direction(Direction::Horizontal)
            .flex(Flex::Center)
            .constraints([
                // Area for Options
                Constraint::Length(25),
            ])
            .margin(4)
            .split(area);
        Block::default()
            .title("Options")
            .borders(Borders::ALL)
            .render_ref(area, buf);

        // Add margin at top and bottom for vertical centering
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20), // Top margin
                Constraint::Length(3),      // New Game
                Constraint::Length(1),      // Spacing
                Constraint::Length(3),      // Key Bindings
                Constraint::Length(1),      // Spacing
                Constraint::Length(3),      // Video
                Constraint::Length(1),      // Spacing
                Constraint::Length(3),      // Audio
                Constraint::Length(1),      // Spacing
                Constraint::Length(3),      // Back
                Constraint::Percentage(20), // Bottom margin
            ])
            .split(sub_area[0]);

        // Render buttons with centered alignment
        self.render_new_game_button(vertical_chunks[1], buf, state);
        self.render_key_bindings_button(vertical_chunks[3], buf, state);
        self.render_video_button(vertical_chunks[5], buf, state);
        self.render_audio_button(vertical_chunks[7], buf, state);
        self.render_back_button(vertical_chunks[9], buf, state);
    }
}

fn options_event_handler(
    mut app_state: ResMut<NextState<AppState>>,
    mut send_options_state: ResMut<NextState<OptionsState>>,
    options_state: Res<State<OptionsState>>,
    mut options_events: EventReader<OptionsEvent>,
    registered_components: Res<RegisteredComponents>,
    context: Res<RatatuiContext>,
) {
    let state = options_state.get();
    let size = context.size().unwrap();
    for event in options_events.read() {
        match event {
            OptionsEvent::MouseEvent(m_evt) => match m_evt.kind {
                MouseEventKind::Moved => {
                    // find the button that is hovered
                    let x = m_evt.column;
                    let y = m_evt.row;
                    if registered_components.is_over(OptionComponents::KeyBindings, x, y) {
                        send_options_state.set(OptionsState::KeyBindingsOver);
                    } else if registered_components.is_over(OptionComponents::NewGame, x, y) {
                        send_options_state.set(OptionsState::NewGameOver);                        
                    } else if registered_components.is_over(OptionComponents::Video, x, y) {
                        send_options_state.set(OptionsState::VideoOver);                        
                    } else if registered_components.is_over(OptionComponents::Audio, x, y) {
                        send_options_state.set(OptionsState::AudioOver);                        
                    } else if registered_components.is_over(OptionComponents::Back, x, y) {
                        send_options_state.set(OptionsState::BackOver);                       
                    } else {
                        send_options_state.set(OptionsState::None);                        
                    }
                    
                }
                MouseEventKind::Down(btn) => {
                    // find the button that is hovered
                    let x = m_evt.column;
                    let y = m_evt.row;
                    if registered_components.is_over(OptionComponents::KeyBindings, x, y) {
                        send_options_state.set(OptionsState::KeyBindingsDown);
                    } else if registered_components.is_over(OptionComponents::NewGame, x, y) {
                        send_options_state.set(OptionsState::NewGameDown);                        
                    } else if registered_components.is_over(OptionComponents::Video, x, y) {
                        send_options_state.set(OptionsState::VideoDown);                        
                    } else if registered_components.is_over(OptionComponents::Audio, x, y) {
                        send_options_state.set(OptionsState::AudioDown);                        
                    } else if registered_components.is_over(OptionComponents::Back, x, y) {
                        send_options_state.set(OptionsState::BackDown);                       
                    } else {
                        send_options_state.set(OptionsState::None);                        
                    }
                }
                MouseEventKind::Up(btn) => {

                },
                _ => {
                    info!("Some other mouse event")
                }
            },
            OptionsEvent::KeyEvent(event) => {
                match event.kind {
                    KeyEventKind::Release => match event.code {
                        crossterm::event::KeyCode::Esc => {
                            app_state.set(AppState::Home);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Resource, Default)]
pub struct RegisteredComponents(HashMap<OptionComponents, Rect>);


impl RegisteredComponents {
    pub fn is_over(&self, component: OptionComponents, x: u16, y: u16) -> bool {
        if let Some(rect) = self.0.get(&component) {
            rect.contains(Position { x, y })
        } else {
            false
        }
    }
}

fn render_options(
    options_state: ResMut<State<OptionsState>>,
    app_state: Res<State<AppState>>,
    mut context: ResMut<RatatuiContext>,
    mut registered_components: ResMut<RegisteredComponents>,
) -> color_eyre::Result<()> {
    let app_state = app_state.get();
    if app_state != &AppState::Options {
        return Ok(());
    }
    context.draw(|frame| {
        let area = frame.area();
        frame.render_stateful_widget_ref(*options_state.get(), area, &mut registered_components );
    })?;

    Ok(())
}

impl OptionsState {
    fn render_new_game_button(&self, area: Rect, buf: &mut Buffer, state: &mut RegisteredComponents) {
        let (title, style) = match self {
            OptionsState::NewGameOver => (
                "New Game",
                Style::default().fg(Color::Black).bg(Color::White),
            ),
            OptionsState::NewGameDown => (
                "New Game",
                Style::default().fg(Color::White).bg(Color::Black),
            ),
            _ => (
                "New Game",
                Style::default().fg(Color::White).bg(Color::Black),
            ),
        };
        Paragraph::new(title)
            .block(
                Block::default()
                .borders(Borders::ALL)
                .border_style(style))
            .style(style)
            .alignment(Alignment::Center)
            .render_ref(area, buf);

        state.0.insert(OptionComponents::NewGame, area);
    }

    fn render_key_bindings_button(&self, area: Rect, buf: &mut Buffer, state: &mut RegisteredComponents) {
        let (title, style) = match self {
            OptionsState::KeyBindingsOver => {
                ("Key Bindings", Style::default().fg(Color::Black).bg(Color::White))
            }
            OptionsState::KeyBindingsDown => {
                ("Key Bindings", Style::default().fg(Color::White).bg(Color::Black))
            }
            _ => ("Key Bindings", Style::default().fg(Color::White).bg(Color::Black)),
        };
        Paragraph::new(title).block(Block::default().borders(Borders::ALL))
            .style(style)
            .alignment(Alignment::Center)
            .render_ref(area, buf);

        state.0.insert(OptionComponents::KeyBindings, area);
    }

    fn render_video_button(&self, area: Rect, buf: &mut Buffer, state: &mut RegisteredComponents) {
        let (title, style) = match self {
            OptionsState::VideoOver => {
                ("Video", Style::default().fg(Color::Black).bg(Color::White))
            }
            OptionsState::VideoDown => {
                ("Video", Style::default().fg(Color::White).bg(Color::Black))
            }
            _ => ("Video", Style::default().fg(Color::White).bg(Color::Black)),
        };
        Paragraph::new(title).block(Block::default().borders(Borders::ALL))
            .style(style)
            .alignment(Alignment::Center)
            .render_ref(area, buf);


        state.0.insert(OptionComponents::Video, area);
    }

    fn render_audio_button(&self, area: Rect, buf: &mut Buffer, state:  &mut RegisteredComponents) {
        let (title, style) = match self {
            OptionsState::AudioOver => {
                ("Audio", Style::default().fg(Color::Black).bg(Color::White))
            }
            OptionsState::AudioDown => {
                ("Audio", Style::default().fg(Color::White).bg(Color::Black))
            }
            _ => ("Audio", Style::default().fg(Color::White).bg(Color::Black)),
        };
        Paragraph::new(title).block(Block::default().borders(Borders::ALL))
            .style(style)
            .alignment(Alignment::Center)
            .render_ref(area, buf);

        state.0.insert(OptionComponents::Audio, area);
    }

    fn render_back_button(&self, area: Rect, buf: &mut Buffer, state:  &mut RegisteredComponents) {
        let (title, style) = match self {
            OptionsState::BackOver => {
                ("Back", Style::default().fg(Color::Black).bg(Color::White))
            }
            OptionsState::BackDown => {
                ("Back", Style::default().fg(Color::White).bg(Color::Black))
            }
            _ => ("Back", Style::default().fg(Color::White).bg(Color::Black)),
        };
        Paragraph::new(title).block(Block::default().borders(Borders::ALL))
            .style(style)
            .alignment(Alignment::Center)
            .render_ref(area, buf);

        state.0.insert(OptionComponents::Back, area);
    }
}
