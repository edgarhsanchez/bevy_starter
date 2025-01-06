mod widgets {
    pub mod prelude;
    pub mod key_bindings;
}

mod app_state;

mod mini_salsa {
    pub mod theme;
}

use app_state::{AppPlugin, AppState, HomeEvent};
use widgets::prelude::*;

use crossterm::event::{KeyCode, MouseEventKind};
use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*, 
    state::app::StatesPlugin,
};
use bevy_ratatui::{
    error::exit_on_error, event::{KeyEvent, MouseEvent}, terminal::RatatuiContext, RatatuiPlugins,
};

fn main() {    

    App::new()
        // .add_plugins(RatatuiPlugins {
        //     enable_mouse_capture: true,
        //     ..default()
        // })
        .add_plugins(RatatuiPlugins::default())
        .add_plugins(StatesPlugin)
        .init_state::<AppState>()
        .add_plugins(AppPlugin)        
        .add_systems(PreUpdate, keyboard_events_handler)
        .add_systems(PreUpdate, mouse_events_handler)
        .add_systems(Update, ui_system.pipe(exit_on_error))

        
        .run();
}

fn mouse_events_handler(
    mut app_state: ResMut<State<AppState>>,
    mut mouse_events: EventReader<MouseEvent>,
    mut home_events: EventWriter<HomeEvent>,
) {
    for event in mouse_events.read() {
        match app_state.get() {
            AppState::Home => {
                home_events.send(HomeEvent::MouseEvent(event.clone()));
            }
        }
    }
}

fn keyboard_events_handler(
    mut keyboard_events: EventReader<KeyEvent>,
    mut exit_event: EventWriter<AppExit>,
    mut home_events: EventWriter<HomeEvent>,
    app_state: Res<State<AppState>>,
) {
    // get event
    for event in keyboard_events.read() {
        match app_state.get() {
            AppState::Home => {
                match event.code {
                    KeyCode::Char('q') => {
                        exit_event.send_default();
                    }
                    _ => {
                        home_events.send(HomeEvent::KeyEvent(event.clone()));
                    }
                }
            }
        }
    }
}

fn ui_system(
    mut context: ResMut<RatatuiContext>,
    app_state: Res<State<AppState>>,
) -> color_eyre::Result<()> {
    context.draw(|frame| {
        let area = frame.area();
        // pass to app_state for rendering of state
        frame.render_widget(app_state.get(), area);
    })?;
    Ok(())
}
