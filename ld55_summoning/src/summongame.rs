use bevy::prelude::*;


// Global State of the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameAppState {
    #[default]
    TitleScreen,
    Gameplay,
}

#[derive(Component)]
pub struct DontDeleteOnAppStateChange;
