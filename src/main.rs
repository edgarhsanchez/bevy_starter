mod widgets {
    pub mod home;
    pub mod options;
    pub mod key_bindings;
}


mod mini_salsa {
    pub mod theme;
}

mod states {
    pub mod home_state;
    pub mod app_state;
    pub mod options_state;
}
use std::{io::stdout, time::Duration, error::Error};

use states::app_state::AppState;
use widgets::{home::{HomeEvent, HomePlugin}, options::{OptionsEvent, OptionsPlugin}};

use crossterm::{cursor::{DisableBlinking, EnableBlinking, SetCursorStyle}, event::{DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture, KeyCode, KeyEventKind, MouseEventKind}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*, 
    state::app::StatesPlugin,
};
use bevy_ratatui::{
    error::exit_on_error, event::{KeyEvent, MouseEvent}, terminal::RatatuiContext, RatatuiPlugins,
};

fn main() -> Result<(), Box<dyn Error>> {    
    let frame_rate = Duration::from_secs_f64(1.0/60.0);
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    stdout().execute(EnableBlinking)?;
    stdout().execute(SetCursorStyle::BlinkingBar)?;
    stdout().execute(EnableBracketedPaste)?;
    enable_raw_mode()?;
    App::new()
        .add_plugins(bevy::log::LogPlugin::default())
        .add_plugins(RatatuiPlugins{
            enable_mouse_capture: true,
            ..default()
        })        
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_rate)))
        .add_plugins(StatesPlugin)        
        .init_state::<AppState>()
        .add_plugins(HomePlugin)    
        .add_plugins(OptionsPlugin)    
        .add_systems(PreUpdate, keyboard_events_handler)
        .add_systems(PreUpdate, mouse_events_handler)
        .run();

        disable_raw_mode()?;
        stdout().execute(DisableBracketedPaste)?;
        stdout().execute(SetCursorStyle::DefaultUserShape)?;
        stdout().execute(DisableBlinking)?;
        stdout().execute(DisableMouseCapture)?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
}

fn mouse_events_handler(
    app_state: ResMut<State<AppState>>,
    mut mouse_events: EventReader<MouseEvent>,
    mut home_events: EventWriter<HomeEvent>,
    mut options_events: EventWriter<OptionsEvent>,
) {
    let app_state = app_state.get();
    for event in mouse_events.read() {
        match app_state {
            AppState::Home => {
                home_events.send(HomeEvent::MouseEvent(event.clone()));
            }
            AppState::Options => {
                options_events.send(OptionsEvent::MouseEvent(event.clone()));
            }
        }
    }
}

fn keyboard_events_handler(
    app_state: Res<State<AppState>>,
    mut keyboard_events: EventReader<KeyEvent>,   
    mut home_events: EventWriter<HomeEvent>,  
    mut options_events: EventWriter<OptionsEvent>  
) {
    let app_state = app_state.get();
    // get event
    for event in keyboard_events.read() {
        match app_state {
            AppState::Home => {
                home_events.send(HomeEvent::KeyEvent(event.clone()));
            }
            AppState::Options => {
                options_events.send(OptionsEvent::KeyEvent(event.clone()));
            }
        }
    }
}