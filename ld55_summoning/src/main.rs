//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    core_pipeline::{
        //bloom::{BloomCompositeMode, BloomSettings},
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    },
    pbr::NotShadowCaster,
    prelude::*,    
    render::{color, mesh::VertexAttributeValues, texture::{ImageAddressMode, ImageSamplerDescriptor}},

};

use rand::Rng;

use std::f32::consts::PI;

use crate::gamestate::{GameMap, MapDirection, INVALID};
use crate::gamestate::MapSpaceContents;
pub mod gamestate;

const HEX_SZ : f32 = 1.0;

#[derive(Resource,Default)]
struct CardDeck {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,

    // todo: card stats, etc 
}

#[derive(Default)]
struct PlayerStuff
{
    color: Color,
    color2 : Color,
    ring_mtl: [ Handle<StandardMaterial>; 21 ],
}

// Resource  stuff
#[derive(Resource,Default)]
struct GoodStuff {
    ring_mesh: Handle<Mesh>,
    player_stuff : [ PlayerStuff ; 4],
}


#[derive(Event)]
enum GameStateChanged {
    CircleAdded(i32),
    CircleSplit(i32,i32),  // old ndx -> new ndx
}

#[derive(Resource)]
struct GameState {
    map : GameMap,
    map_visuals: Vec<Entity>,
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            map: GameMap::default(),
            map_visuals: Vec::new(),
        }
    }
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct GameCamera;

#[derive(Component)]
struct GameCursor {
    ndx : usize,
    cursor_world : Vec3,
    drag_from : Option<usize>,
    drag_dest : Option<usize>,
}


#[derive(Component)]
struct MapSpaceVisual 
{
    ndx : usize,
    circle : Option<Entity>,
}

fn main() {

    App::new()    
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
            primary_window: Some( Window {
                title: "LD55 Summoning".into(),
                canvas: Some("#mygame-canvas".into()),
                ..default()
            }),            
            ..default()          
        }).set( ImagePlugin {
            default_sampler: ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..default()            
            },
            ..default()
        })
        ) // add_plugins
        //.insert_resource( CardDeck::default() )
        .insert_resource( GoodStuff::default() )
        .insert_resource( GameState::default() )
        .add_systems(Startup, setup)
        .add_systems(Startup, build_map )
        //.add_systems( Update, spawn_cards)
        .add_systems( Update, test_rings)
        .add_systems( Update, handle_input )
        .add_systems( Update, on_gamestate_changed )
        .add_systems( Update, draw_split_feedback )
        .add_event::<GameStateChanged>()
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,    
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    //mut cards: ResMut<CardDeck>,
    mut stuff: ResMut<GoodStuff>,
    mut config_store: ResMut<GizmoConfigStore>,
    mut gamestate: ResMut<GameState>,
    asset_server: Res<AssetServer>
) {


    // set up gizmos
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width *= 2.0;

    // circular base
    let mut plane_mesh = Mesh::from( Plane3d { normal: Direction3d::Y } )
                    .with_generated_tangents().unwrap();

    // scale the UVs
    let uvs = plane_mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
    let uvscale = 3.0;
    match uvs {
        VertexAttributeValues::Float32x2(values) => {
            for uv in values.iter_mut() {
                uv[0] *= uvscale;
                uv[1] *= uvscale; 
            }
        },
        _ => (),
    };

    commands.spawn((PbrBundle {
        //mesh: meshes.add(Circle::new(4.0)),
        mesh: meshes.add( plane_mesh ),
        material: materials.add( StandardMaterial{
            base_color_texture: Some( asset_server.load("tx_hextest/Hex Test_BaseColor-256x256.PNG") ),
            normal_map_texture: Some( asset_server.load("tx_hextest/Hex Test_Normal-256x256.PNG") ),
            emissive: Color::WHITE * 50.0,
            emissive_texture: Some( asset_server.load("tx_hextest/Hex Test_Emissive-256x256.PNG") ),
            perceptual_roughness: 1.0,
            metallic: 1.0,
            metallic_roughness_texture: Some( asset_server.load("tx_hextest/Hex Test_MetalRoughness-256x256.PNG") ),
            occlusion_texture: Some( asset_server.load("tx_hextest/Hex Test_AmbientOcclusion-256x256.PNG") ),
            ..default()
        }),
         transform: Transform::from_scale(Vec3::new(10.0, 10.0, 10.0)),
        //     Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)).with_scale( Vec3::new(4.0, 4.0, 4.0) ),
        ..default()
    }, Ground ));


    // Stuff for summoning circles
    let ring_mesh = Mesh::from( Plane3d { normal: Direction3d::Y } ).with_generated_tangents().unwrap();
    stuff.ring_mesh = meshes.add( ring_mesh );

    stuff.player_stuff[0].color  = Color::rgb_u8(255, 113, 206);
    stuff.player_stuff[0].color2 = Color::rgb_u8(161, 45, 172 );

    stuff.player_stuff[1].color  = Color::rgb_u8(185, 103, 255);
    stuff.player_stuff[1].color2 = Color::rgb_u8(52, 37, 174);

    stuff.player_stuff[2].color  = Color::rgb_u8(1, 205, 254);
    stuff.player_stuff[2].color2 = Color::rgb_u8(1, 150, 114);

    stuff.player_stuff[3].color  = Color::rgb_u8(5, 254, 161);
    stuff.player_stuff[3].color2 = Color::rgb_u8(1, 152, 30);
    
    for i in 1..=20 {
        //let ring_texname = format!("ring_{:02}.png", i);
        let ring_texname = format!("tx_rings/RingGen_{:02}_BaseColor.PNG", i );
        let ring_emit_texname = format!("tx_rings/RingGen_{:02}_Emissive.PNG", i );

        for p in 0..4 {

            let mut color_main = stuff.player_stuff[p].color * 200.0;
            color_main.set_a(1.0);

            let mut color_support = stuff.player_stuff[p].color * 1.5;
            color_support.set_a( 1.0 );

            let ring_mtl = StandardMaterial {
                base_color: color_support,
                base_color_texture: Some(asset_server.load(ring_texname.clone())),
                emissive: color_main,
                emissive_texture: Some(asset_server.load(ring_emit_texname.clone())),
                alpha_mode: AlphaMode::Blend,
                ..default()
            };
            
            stuff.player_stuff[p].ring_mtl[i - 1] = materials.add(ring_mtl);
        }
    }
        
    // cursor cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(255, 144, 10)),        
        transform: Transform::from_xyz(5.0, 0.5, 5.0),
        ..default()
    }, GameCursor { ndx : 0, drag_from : None, drag_dest : None, cursor_world : Vec3::ZERO } )).id();
    
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color : Color::rgb_u8( 75, 187, 235 ),
            //color : Color::WHITE,
            intensity: 5_000_000.0,
            //intensity: 1.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, 1.0),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            //color : Color::rgb_u8( 200, 147, 50 ),
            color : Color::rgb_u8( 180, 27, 77 ),
            //shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 10.0, 0.0)
                .with_rotation(Quat::from_euler( EulerRot::XYZ, -PI / 4., -PI / 6., 0.0)),
            //.with_rotation(Quat::from_rotation_x( -PI / 4.)),
        ..default()
    });
    
    // camera
    commands.spawn( ( Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz( 0.0, 15.0, 12.0)
                                    .looking_at( Vec3 { x:0.0, y: 0.0, z : 3.0 }, Vec3::Y),
            tonemapping: Tonemapping::TonyMcMapface,         
            ..default()
            },
            BloomSettings::NATURAL,
            GameCamera
        ));

    // 2D scene -------------------------------
    commands.spawn(Camera2dBundle { 
        camera: Camera {
            hdr: true,
            order: 2, // Draw sprites on top of 3d world
            ..default()
        },
        ..default()
    });

    // Load card atlas
    // let texture = asset_server.load("cardfish_cards.png");
    // let layout = TextureAtlasLayout::from_grid(
    //     Vec2::new( 567.0*(256.0/811.0), 256.0), 11, 2, None, None);
    // let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // cards.texture = texture;
    // cards.layout = texture_atlas_layout;

    // commands.spawn((
    //     SpriteSheetBundle {
    //         texture,
    //         atlas: TextureAtlas {
    //             layout: texture_atlas_layout,
    //             index: 0,
    //         },            
    //         ..default()
    //     },        
    // ));

    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("bevy_bird_dark.png"),
    //     ..default()
    // });
}

// fn spawn_cards ( 
//     mut commands: Commands,        
//     cards: Res<CardDeck>,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
// )
// {
//     if keyboard_input.just_pressed( KeyCode::KeyW ) {
//         println!("W pressed");
//         let mut rng = rand::thread_rng();
//         commands.spawn((
//             SpriteSheetBundle {
//                 texture: cards.texture.clone(),
//                 atlas: TextureAtlas {
//                     layout: cards.layout.clone(),
//                     index: rng.gen_range(1..20),
//                 },                     
//                 transform: Transform::from_xyz(rng.gen::<f32>() * 1000.0 - 500.0, rng.gen::<f32>() * 700.0 - 350.0, 0.0),
//                 ..default()
//             },        
//         ));
//     }
// }

fn test_rings ( 
    mut gamestate: ResMut<GameState>,
    cursor_q: Query<(&Transform, &GameCursor)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ev_gamestate: EventWriter<GameStateChanged>,
)
{
    if keyboard_input.just_pressed( KeyCode::KeyW ) {
        println!("W pressed");

        let (xform, cursor_info) = cursor_q.single();        
        if (gamestate.map.spaces[ cursor_info.ndx ].player == 0) {
            gamestate.map.spaces[ cursor_info.ndx ].player = 1;            
        }

        gamestate.map.spaces[ cursor_info.ndx ].power = gamestate.map.spaces[ cursor_info.ndx ].power + 1;
        println!("index {} power now {}", cursor_info.ndx,  gamestate.map.spaces[ cursor_info.ndx ].power );

        ev_gamestate.send( GameStateChanged::CircleAdded( cursor_info.ndx as i32) );
    }
}

// fn handle_input (
//     mut commands: Commands,
//     mouse_buttons: Res<Input<MouseButton>>,
//     windows: Res<Windows>,
// ) {
//     if let Some(cursor_position) = windows.single().cursor_position() {
//         zz
//     }
// }


fn handle_input(
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    mut cursor_q: Query<(&mut Transform, &mut GameCursor)>,
    maptile_query: Query<(Entity, &GlobalTransform, &MapSpaceVisual), With<MapSpaceVisual>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,    
    mut game: ResMut<GameState>,
    mut ev_gamestate: EventWriter<GameStateChanged>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(ground.translation(), Plane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.    
    gizmos.circle(
        point + ground.up() * 0.15,
        Direction3d::new_unchecked(ground.up()), // Up vector is already normalized.
        0.2,
        Color::WHITE,
    );
    

    // Find the closest map tile to the cursor
    let mut closest_tile: Option<(Entity, &GlobalTransform, f32, usize)> = None;    

    for (entity, transform, vis) in maptile_query.iter() {                
        let distance = transform.translation().distance(point);
        if let Some((_, _, closest_distance, _)) = closest_tile {            
            if distance < closest_distance {
                closest_tile = Some((entity, transform, distance, vis.ndx ));
            }
        } else {
            closest_tile = Some((entity, transform, distance, vis.ndx ));
        }
    }
    
    if let Some(( _closest_entity, tile_xform, _, ndx )) = closest_tile {        

        let (mut cursor_transform, mut cursor_info) = cursor_q.single_mut();
                
        let (scale, rot, pos) = tile_xform.to_scale_rotation_translation();
        cursor_transform.translation = pos;
        cursor_transform.rotation = rot;
        cursor_transform.scale = scale;

        //if (ndx != cursor_info.ndx) {

        // {
        //     let verbose = (ndx != cursor_info.ndx);
        //     let ndx = ndx as i32;
        //     if (verbose) {
        //         println!("Map Index: {}", ndx );
        //     }

        //     // look at the hovered square
        //     if ((ndx >= 0) && (ndx < 100)) {
        //         let mapsq = game.map.spaces[ ndx as usize ];
                
        //         // TODO: player check
        //         if (mapsq.contents == MapSpaceContents::Playable) && (mapsq.power > 1) {
        //             draw_map_dir( &mut gizmos, &game, ndx, MapDirection::North, verbose);
        //             draw_map_dir( &mut gizmos, &game, ndx, MapDirection::NorthEast, verbose );
        //             draw_map_dir( &mut gizmos, &game, ndx, MapDirection::SouthEast, verbose);
        //             draw_map_dir( &mut gizmos, &game, ndx, MapDirection::South, verbose);
        //             draw_map_dir( &mut gizmos, &game, ndx, MapDirection::SouthWest, verbose);
        //             draw_map_dir( &mut gizmos, &game, ndx, MapDirection::NorthWest, verbose );            
        //         }
        //     }
        // }
        
        cursor_info.ndx = ndx;        
        cursor_info.cursor_world = point;

        if (mouse_button_input.just_pressed(MouseButton::Left)) {

            // Make sure there is some power to drag from
            if (ndx >=0) && (ndx != INVALID) && (game.map.spaces[ ndx ].power > 1 ) {            
                cursor_info.drag_from = Some( ndx );
                println!("Drag from: {}", ndx );
            }
        }
        
        if (mouse_button_input.just_released(MouseButton::Left)) {
            println!("TODO split" );            
            if (cursor_info.drag_from.is_some()) {
                
                let drag_from_ndx = cursor_info.drag_from.unwrap() as i32;
                let drag_from_pos = worldpos_from_mapindex(drag_from_ndx as i32);        

                let mapdir = mapdir_from_drag( cursor_info.cursor_world, drag_from_pos );
                let found = game.map.search_dir( drag_from_ndx,  mapdir );
                if (found != drag_from_ndx) && (found != gamestate::INVALID as i32) 
                {
                    let found_ndx = found as usize;
                    if (game.map.spaces[ found_ndx ].player == 0) {
                        game.map.spaces[ found_ndx ].player = 1;
                        game.map.spaces[ found_ndx ].power = 1;                        
                        ev_gamestate.send( GameStateChanged::CircleAdded( found_ndx as i32) );

                        game.map.spaces[ drag_from_ndx as usize].power -= 1;
                        ev_gamestate.send( GameStateChanged::CircleAdded( drag_from_ndx) );
                    }
            
                    
                }

            }
        }

        if (!mouse_button_input.pressed(MouseButton::Left)) {
            if (cursor_info.drag_from.is_some()) {
                println!("Drag clear" );
            }
            cursor_info.drag_from = None;            
        }
    }
}

fn draw_map_dir( gizmos: &mut Gizmos, game : &GameState, ndx : i32, dir : MapDirection, verbose : bool )
{    
    let found = game.map.search_dir( ndx,  dir );
    if (verbose) {
        let dir_str = format!("{:?}", dir);
        let dir_str_padded = format!("{:<10}", dir_str);                    
        println!("   {} {} Open {}", dir_str_padded, gamestate::move_dir( ndx, dir ),  found );    
    }
    if (found != ndx) && (found != gamestate::INVALID as i32) {
        let pos_a = worldpos_from_mapindex(ndx) + Vec3::Y * 0.25;
        let pos_b = worldpos_from_mapindex(found) + Vec3::Y * 0.25;
        gizmos.line(pos_a, pos_b, Color::ORANGE );
        gizmos.cuboid( 
            Transform::from_translation(pos_b), //.with_scale(Vec3::splat(1.25)),
            Color::ORANGE );

    }

}

fn mapdir_from_drag( pos : Vec3, start_pos : Vec3 ) -> MapDirection
{
    // get best angle from arrow
    let dir = pos - start_pos;
    let angle = dir.z.atan2(dir.x);
    let mut angle_degrees = angle.to_degrees() + (90.0 + 30.0);
    if (angle_degrees < 0.0) {
        angle_degrees = angle_degrees + 360.0;
    }
    
    match (angle_degrees / 60.0).floor() as i32 {
        0 => MapDirection::North,
        1 => MapDirection::NorthEast,
        2 => MapDirection::SouthEast,
        3 => MapDirection::South,
        4 => MapDirection::SouthWest,
        5 => MapDirection::NorthWest,
        _ => MapDirection::North, // Default case
    }
}

fn draw_split_feedback(
    cursor_q: Query<(&Transform, &GameCursor)>,    
    game: Res<GameState>,
    mut gizmos: Gizmos,
)
{
    let offs = Vec3 { x : 0.0, y : 0.15, z : 0.0 };

    let (cursor_transform, cursor_info) = cursor_q.single();

    if cursor_info.drag_from.is_some() {
        // Draw a gizmo for drag_from
        let drag_from_ndx = cursor_info.drag_from.unwrap();
        let drag_from_pos = worldpos_from_mapindex(drag_from_ndx as i32);        
        gizmos.arrow( drag_from_pos + offs, cursor_info.cursor_world + offs, Color::YELLOW );

        // cursor_info.cursor_world - drag_from_pos;
        let mapdir = mapdir_from_drag( cursor_info.cursor_world, drag_from_pos );        
        draw_map_dir( &mut gizmos, &game, drag_from_ndx as i32, mapdir, false);

        // println!( "Drag angle: {} degrees dir {:?}", angle_degrees, mapdir );
    } else {
        // not dragging, should we show preview?                
        let ndx = cursor_info.ndx as i32;        

        // look at the hovered square
        if ((ndx >= 0) && (ndx < 100)) {
            let mapsq = game.map.spaces[ ndx as usize ];
            
            // TODO: player check
            if (mapsq.contents == MapSpaceContents::Playable) && (mapsq.power > 1) {
                draw_map_dir( &mut gizmos, &game, ndx, MapDirection::North, false);
                draw_map_dir( &mut gizmos, &game, ndx, MapDirection::NorthEast, false );
                draw_map_dir( &mut gizmos, &game, ndx, MapDirection::SouthEast, false);
                draw_map_dir( &mut gizmos, &game, ndx, MapDirection::South, false);
                draw_map_dir( &mut gizmos, &game, ndx, MapDirection::SouthWest, false);
                draw_map_dir( &mut gizmos, &game, ndx, MapDirection::NorthWest, false );            
            }
        }
        
    }
            
}


fn worldpos_from_mapindex( mapindex : i32 ) -> Vec3
{
    let row : i32 = mapindex / (gamestate::MAP_SZ as i32);
    let col : i32 = mapindex % (gamestate::MAP_SZ as i32);

    // offset if col is odd
    

    // Make a vec3 from row and col        
    let sqrt3 = 1.7320508075688772;
    let offset = if col % 2 == 1 { HEX_SZ * sqrt3 / 2.0 } else { 0.0 };
    Vec3::new((col as f32 - 4.5) * (HEX_SZ * (3.0/2.0) ), 0.0,
    (-row as f32 + 5.0) * (HEX_SZ * sqrt3) + offset )
}

// fn spawn_mapspace_empty( mut commands: Commands ) -> Entity {
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
//         material: materials.add(Color::rgb_u8(124, 144, 255)),        
//         transform: Transform::from_xyz(0.0, 0.5, 0.0),
//         ..default()
//     }).id()
// }

fn build_map (
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut gamestate: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,    
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ev_gamestate: EventWriter<GameStateChanged>,
) 
{
    println!("Hello from build_map.");
    
    // First, set up the map indices and build the map
    let mut rng = rand::thread_rng();
    let mut index = 0;
    for map_space in &mut gamestate.map {
        map_space.ndx = index;
        index = index + 1;

        let hex_pos = worldpos_from_mapindex( map_space.ndx );

        if hex_pos.length() < 100.0 {
            //println!("Map includes hex {}, World Position: {:?} len {}", map_space.ndx, hex_pos, hex_pos.length());
            if rng.gen_ratio(1, 8) {
                map_space.contents = MapSpaceContents::Blocked;
            } else {
                map_space.contents = MapSpaceContents::Playable;

                if rng.gen_ratio(1, 4) {
                    map_space.player = rng.gen_range(1..=4);
                    map_space.power = rng.gen_range(1..=20);
    
                    // send a gamestate change to mark the init
                    ev_gamestate.send( GameStateChanged::CircleAdded( map_space.ndx ) );
                }
            }            
        }
    }

    // Now build the map visuals based on the map data
    let hex_scene = asset_server.load("hexagon.glb#Scene0");

    let mut map_visuals = Vec::new();
    for map_space in &gamestate.map {
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

}

fn on_gamestate_changed( 
    mut commands: Commands,
    stuff: Res<GoodStuff>,
    gamestate: Res<GameState>,    
    mut q_mapvis : Query<&mut MapSpaceVisual>,    
    mut ev_gamestate: EventReader<GameStateChanged>, ) 
{
    for ev in ev_gamestate.read() {

        match ev {
            GameStateChanged::CircleAdded(ndx ) => {
                
                let ndx = *ndx as usize;
                let spc = gamestate.map.spaces[ndx];
                println!("Added circle at {}, power is {}, player {}", ndx, spc.power, spc.player  );

                // Get the maptile entity that is the parent

                
                // Remove any existing childs                
                let ent_vis = gamestate.map_visuals[ndx];
                let vis = q_mapvis.get( gamestate.map_visuals[ndx]).unwrap();
                match vis.circle {                    
                    Some(child_ent) => { 
                        commands.entity(ent_vis).remove_children( &[ child_ent ]); 
                        commands.entity( child_ent ).despawn();
                    }
                    None => {}
                }

                //commands.entity(ent_vis).
                let ring_sz = if spc.power == 1 { 0.9 } else { 1.25 };

                let ent_ring = commands.spawn((PbrBundle {            
                    mesh: stuff.ring_mesh.clone(),
                    material: stuff.player_stuff[spc.player as usize - 1].ring_mtl[ (spc.power as usize) - 1 ].clone(),
                    transform: Transform {
                        translation : Vec3 { x: 0.0, y : 0.2, z : 0.0 },
                        scale: Vec3::splat( ring_sz ),
                        ..default()
                    },
                    //transform: Transform::from_scale(Vec3::new(10.0, 10.0, 10.0)),
                    //     Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)).with_scale( Vec3::new(4.0, 4.0, 4.0) ),
                    ..default()
                }, NotShadowCaster) ).id();


                let mut vis = q_mapvis.get_mut( gamestate.map_visuals[ndx] ).unwrap();
                vis.circle = Some(ent_ring);

                commands.entity(ent_vis).add_child(ent_ring);


            }
            GameStateChanged::CircleSplit( src, dest) => {
                println!("Split circle at {} to {}", src, dest  );
            }
        }
    }
}