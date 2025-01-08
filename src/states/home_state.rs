use bevy::prelude::States;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum HomeState {
    #[default]
    Home,
    Options,
}