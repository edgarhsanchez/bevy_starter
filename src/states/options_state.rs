use bevy::prelude::States;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum OptionsState {
    #[default]
    None,
    NewGameOver,
    NewGameDown,
    KeyBindingsOver,
    KeyBindingsDown,
    VideoOver,
    VideoDown,
    AudioOver,
    AudioDown,
    BackOver,
    BackDown,
}