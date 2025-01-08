use bevy::prelude::*;
use bevy_ratatui::{
    error::exit_on_error,
    event::{KeyEvent, MouseEvent},
    terminal::RatatuiContext,
};
use crossterm::event::KeyEventKind;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Flex, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, WidgetRef},
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

impl WidgetRef for OptionsState {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
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
        self.render_new_game_button(vertical_chunks[1], buf);
        self.render_key_bindings_button(vertical_chunks[3], buf);
        self.render_video_button(vertical_chunks[5], buf);
        self.render_audio_button(vertical_chunks[7], buf);
        self.render_back_button(vertical_chunks[9], buf);
    }
}

fn options_event_handler(
    mut app_state: ResMut<NextState<AppState>>,
    mut send_options_state: ResMut<NextState<OptionsState>>,
    options_state: Res<State<OptionsState>>,
    mut options_events: EventReader<OptionsEvent>,
) {
    let options_event = options_events.read();
    let state = options_state.get();
    for event in options_event {
        match event {
            OptionsEvent::MouseEvent(event) => match state {
                OptionsState::None => {}
                _ => {}
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

fn render_options(
    options_state: Res<State<OptionsState>>,
    app_state: Res<State<AppState>>,
    mut context: ResMut<RatatuiContext>,
) -> color_eyre::Result<()> {
    let app_state = app_state.get();
    if app_state != &AppState::Options {
        return Ok(());
    }
    context.draw(|frame| {
        let area = frame.area();
        frame.render_widget(options_state.get(), area);
    })?;

    Ok(())
}

impl OptionsState {
    fn render_new_game_button(&self, area: Rect, buf: &mut Buffer) {
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
            .block(Block::default().borders(Borders::ALL))
            .style(style)
            .alignment(Alignment::Center)
            .render_ref(area, buf);

    }

    fn render_key_bindings_button(&self, area: Rect, buf: &mut Buffer) {
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
    }

    fn render_video_button(&self, area: Rect, buf: &mut Buffer) {
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


    }

    fn render_audio_button(&self, area: Rect, buf: &mut Buffer) {
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
    }

    fn render_back_button(&self, area: Rect, buf: &mut Buffer) {
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
    }
}
