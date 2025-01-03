mod widgets {
    pub mod prelude;
    pub mod key_bindings;
    pub mod mouse_context;
}

use widgets::prelude::*;

use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*,
};
use bevy_ratatui::{
    event::{KeyEvent, MouseEvent},
    terminal::RatatuiContext,
    RatatuiPlugins,
};

fn main() {
    let wait_duration = std::time::Duration::from_secs_f64(1. / 60.);

    App::new()
        .add_plugins(RatatuiPlugins {
            enable_mouse_capture: true,
            ..default()
        })
        .add_plugins(ScheduleRunnerPlugin::run_loop(wait_duration))
        .run();
}
