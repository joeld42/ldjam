use bevy::prelude::*;


use crate::gamestate::{GameSnapshot, INVALID};

// Global State of the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameAppState {
    #[default]
    TitleScreen,
    Gameplay,
}

#[derive(Default, PartialEq, Debug)]
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

#[derive(Event)]
pub enum GameStateChanged {
    CircleAdded(i32),
    CircleSplit(i32,i32),  // old ndx -> new ndx
}

#[derive(Event)]
pub struct TurnAdvance(pub i32);


#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct MapSpaceVisual
{
    pub ndx : usize,
    pub circle : Option<Entity>,
}

pub const HEX_SZ : f32 = 1.0;

// FIXME: this should be a singleton component and not a resource
#[derive(Resource)]
pub struct SummonGame {
    //map : GameMap,
    pub snapshot : GameSnapshot,
    pub map_visuals: Vec<Entity>,
    pub player_count : i32,
    pub player_turn : i32,
    pub turn_num : i32
}

impl Default for SummonGame {
    fn default() -> SummonGame {
        SummonGame {
            snapshot: GameSnapshot::default(),
            map_visuals: Vec::new(),
            player_count: 0,
            player_turn: 0,
            turn_num: 0,
        }
    }
}

