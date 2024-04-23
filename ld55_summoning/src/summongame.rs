use bevy::prelude::*;


// Global State of the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameAppState {
    #[default]
    TitleScreen,
    Gameplay,
}

#[derive(Default, PartialEq)]
pub enum PlayerType {
    Local,
    AI, // AI(AIPolicy)
    #[default]
    NotActive
}

#[derive(Default)]
pub struct PlayerStuff
{
    pub color: Color,
    pub color2 : Color,
    pub ring_mtl: [ Handle<StandardMaterial>; 21 ],
    pub ptype : PlayerType,
    pub out_of_moves : bool,
}

// Resource  stuff
#[derive(Resource,Default)]
pub struct GoodStuff {
    pub ring_mesh: Handle<Mesh>,
    pub player_stuff : [ PlayerStuff ; 4],
}
