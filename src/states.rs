use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Tutorial,
    Start,
    InGame,
    GameOver,
    SelectUpgrade,
}
