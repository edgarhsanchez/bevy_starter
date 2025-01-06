mod widgets {
    pub mod prelude;
    pub mod key_bindings;
}

mod app_state;

mod mini_salsa {
    pub mod theme;
}

use app_state::AppState;
use widgets::prelude::*;

use crossterm::event::KeyCode;
use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*, 
    state::app::StatesPlugin,
};
use bevy_ratatui::{
    error::exit_on_error, event::KeyEvent, terminal::RatatuiContext, RatatuiPlugins,
};

fn main() {    

    App::new()
        .add_plugins(RatatuiPlugins {
            enable_mouse_capture: true,
            ..default()
        })
        .add_plugins(StatesPlugin)
        .add_systems(PreUpdate, keyboard_events_handler)
        .add_systems(Update, ui_system.pipe(exit_on_error))
        .init_state::<AppState>()
        .run();
}

fn keyboard_events_handler(mut keyboard_events: EventReader<KeyEvent>, mut exit_event: EventWriter<AppExit>) {
    // get event
    for event in keyboard_events.read() {
        // find relevant widget handler

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
