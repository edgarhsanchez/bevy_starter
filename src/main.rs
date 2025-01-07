mod widgets {
    pub mod prelude;
    pub mod key_bindings;
    pub mod options;
}

mod app_state;

mod mini_salsa {
    pub mod theme;
}

use std::time::Duration;

use app_state::{AppPlugin, AppState, HomeEvent};
use widgets::{options::OptionsEvent, prelude::*};

use crossterm::event::{KeyCode, KeyEventKind, MouseEventKind};
use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*, 
    state::app::StatesPlugin,
};
use bevy_ratatui::{
    error::exit_on_error, event::{KeyEvent, MouseEvent}, terminal::RatatuiContext, RatatuiPlugins,
};

fn main() {    
    let frame_rate = Duration::from_secs_f64(1.0/60.0);
    App::new()
        .add_plugins(RatatuiPlugins::default())
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_rate)))
        .add_plugins(StatesPlugin)        
        .add_plugins(AppPlugin)    
        .add_plugins(OptionsPlugin)    
        .add_systems(PreUpdate, keyboard_events_handler)
        .add_systems(PreUpdate, mouse_events_handler)
        .run();
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