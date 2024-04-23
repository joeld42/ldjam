use bevy::prelude::*;
use crate::summongame::*;
use crate::gamestate::*;

use rand::Rng;
use rand::prelude::SliceRandom;

pub fn build_map (
    asset_server: Res<AssetServer>,
    stuff: Res<GoodStuff>,
    mut commands: Commands,
    mut gamestate: ResMut<SummonGame>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ev_gamestate: EventWriter<GameStateChanged>,
    mut ev_turn: EventWriter<TurnAdvance>,
)
{

    /*
    // wait to start this here because of browser audio stuff... need to find a better way
    // MUUUUSSSIICC
    commands.spawn(AudioBundle {
        source: asset_server.load("SummoningStuff_OGG.ogg"),
        settings: PlaybackSettings::LOOP,
        ..default()
    });
    */


    // Count number of active players to get target size for map
    let mut player_count = 0;
    for i in 0..stuff.player_stuff.len() {
        if stuff.player_stuff[i].ptype != PlayerType::NotActive {
            player_count += 1;
        }
    }

    // Remember this
    gamestate.player_count = player_count;


    // First, set up the map indices and build the map
    let mut rng = rand::thread_rng();
    let mut index = 0;
    let mut space_count = 0;
    for map_space in &mut gamestate.snapshot.map {
        map_space.ndx = index;
        index = index + 1;

        let hex_pos = worldpos_from_mapindex( map_space.ndx );

        // this trims the board and makes it more rounder
        if hex_pos.length() < 8.0 {

            // todo: replace this with adding some obstacles with preset shapes
            if rng.gen_ratio(1, 8) {
                map_space.contents = MapSpaceContents::Blocked;
            } else {
                map_space.contents = MapSpaceContents::Playable;
                space_count += 1;
            }
        }
    }


    println!("Hello from build_map, Players {} target spaces {} have {}.",
            player_count, player_count * 16, space_count );

    let target_spaces = player_count * 16;
    let mut attempts = 1000;
    while space_count > target_spaces && attempts > 0{
        // erode away the board edges
        let edge_spaces = gamestate.snapshot.map.edge_spaces_corners();

        let random_index = rng.gen_range(0..edge_spaces.len());
        let selected_index = edge_spaces[random_index];

        // Try removing this space
        let mut map_copy = gamestate.snapshot.map;
        map_copy.spaces[selected_index as usize].contents = MapSpaceContents::NotInMap;

        if map_copy.check_reachability() {
            //gamestate.snapshot.map.spaces[selected_index].contents = MapSpaceContents::NotInMap;
            gamestate.snapshot.map = map_copy;
            space_count -= 1;
        }

        attempts -= 1;
        println!("iter {} edge spaces now has {} entries space_count {}", attempts, edge_spaces.len(), space_count );
    }

    if attempts == 0 {
        println!("Warning! Failed to erode map.");
    }

    // Find starting spaces
    let mut edge_spaces = gamestate.snapshot.map.edge_spaces();
    edge_spaces.shuffle( &mut rng );

    for i in 0..stuff.player_stuff.len() {
        if stuff.player_stuff[i].ptype != PlayerType::NotActive {
            let selected_index = edge_spaces[i] as usize;

            gamestate.snapshot.map.spaces[ selected_index ].player = (i+1) as u8;
            gamestate.snapshot.map.spaces[ selected_index ].power = 16;

            ev_gamestate.send( GameStateChanged::CircleAdded( selected_index as i32 ) );
        }
    }


    // Now build the map visuals based on the map data
    let hex_scene = asset_server.load("hexagon.glb#Scene0");

    let mut map_visuals = Vec::new();
    for map_space in &gamestate.snapshot.map {
        let hex_pos = worldpos_from_mapindex( map_space.ndx );
        let ent = match map_space.contents {
            MapSpaceContents::NotInMap => Entity::PLACEHOLDER,
            MapSpaceContents::Blocked => {
                commands.spawn((PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0, 3.0, 1.0)),
                    material: materials.add(Color::rgb_u8(96, 60, 100)),
                    transform: Transform::from_translation( hex_pos ),
                    ..default()
                }, MapSpaceVisual { ndx : map_space.ndx as usize, circle: None } )).id()
            },
            MapSpaceContents::Playable => {
                commands.spawn( ( SceneBundle {
                    scene: hex_scene.clone(),
                    transform: Transform::from_translation( hex_pos ),
                    ..default()
                }, MapSpaceVisual { ndx : map_space.ndx as usize, circle: None } )).id()
            },
        };

        map_visuals.push( ent )
    }

    // Add give the new visuals to map
    gamestate.map_visuals = map_visuals;


    println!("Map size {}", gamestate.map_visuals.len());

    // Send a turn advance to update the player prompt
    ev_turn.send( TurnAdvance(gamestate.player_turn) );

}

pub fn worldpos_from_mapindex( mapindex : i32 ) -> Vec3
{
    let row : i32 = mapindex / (MAP_SZ as i32);
    let col : i32 = mapindex % (MAP_SZ as i32);

    // offset if col is odd


    // Make a vec3 from row and col
    let sqrt3 = 1.7320508075688772;
    let offset = if col % 2 == 1 { HEX_SZ * sqrt3 / 2.0 } else { 0.0 };
    Vec3::new((col as f32 - 4.5) * (HEX_SZ * (3.0/2.0) ), 0.0,
    (-row as f32 + 5.0) * (HEX_SZ * sqrt3) + offset )
}
