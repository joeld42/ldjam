//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    core_pipeline::{
        //bloom::{BloomCompositeMode, BloomSettings},
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    }, input::keyboard::Key, pbr::NotShadowCaster,
        prelude::*, render::{mesh::VertexAttributeValues,
             texture::{ImageAddressMode, ImageSamplerDescriptor}}, window::WindowResized

};


pub mod summongame;
pub mod gamestate;
pub mod titlescreen;

use summongame::GameAppState;

use gamestate::gen_valid_moves;
use gamestate::evaluate_position;

use titlescreen::TitleScreenPlugin;

use rand::Rng;
use rand::seq::SliceRandom;

//use std::collections::HashSet;
use std::{f32::consts::PI, time::Duration};

use crate::gamestate::{GameSnapshot, MapDirection, INVALID};
use crate::gamestate::MapSpaceContents;


const HEX_SZ : f32 = 1.0;

// #[derive(Resource,Default)]
// struct CardDeck {
//     texture: Handle<Image>,
//     layout: Handle<TextureAtlasLayout>,

//     // todo: card stats, etc
// }

#[derive(Default, PartialEq)]
enum PlayerType {
    Local,
    AI, // AI(AIPolicy)
    #[default]
    NotActive
}

#[derive(Default)]
struct PlayerStuff
{
    color: Color,
    color2 : Color,
    ring_mtl: [ Handle<StandardMaterial>; 21 ],
    ptype : PlayerType,
    out_of_moves : bool,
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

#[derive(Event)]
struct TurnAdvance(i32);

#[derive(Event)]
struct PlayerSettingsChanged;

// FIXME: this should be a singleton component and not a resource
#[derive(Resource)]
struct GameState {
    //map : GameMap,
    snapshot : GameSnapshot,
    map_visuals: Vec<Entity>,
    player_count : i32,
    player_turn : i32,
    turn_num : i32,
    round_scoring_finished : bool,
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            snapshot: GameSnapshot::default(),
            map_visuals: Vec::new(),
            player_count: 0,
            player_turn: 0,
            turn_num: 0,
            round_scoring_finished : true,
        }
    }
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct GameCamera;

#[derive(Component)]
struct PlayerHelp;

#[derive(Component)]
struct RoundScoringFrame;

#[derive(Component)]
struct TitleScreenCrap;

#[derive(Component)]
struct RoundIcon(i32);

#[derive(Component)]
struct TurnIcon(i32);

#[derive(Component)]
struct PlayerScore(i32);

#[derive(Component)]
struct PlayerSetting(i32);


#[derive(Component)]
struct CircleAnimator {
    target : Vec3,
}

#[derive(Component)]
struct GameCursor {
    ndx : usize,
    cursor_world : Vec3,
    drag_from : Option<usize>,
    _drag_dest : Option<usize>,
    split_pct : f32,
}

#[derive(Component)]
struct SplitLabel {
    is_dest : bool
}

#[derive(Component)]
struct AIController {
    turn_timer: Timer,
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
                resolution: (948.0, 533.0).into(),
                //canvas: Some("#mygame-canvas".into()),
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
        })) // add_plugins
        .add_plugins(
            TitleScreenPlugin
        )
        .init_state::<GameAppState>()
        //.insert_resource( CardDeck::default() )
        .insert_resource( GoodStuff::default() )
        .insert_resource( GameState::default() )
        .add_systems(Startup, setup)
        //.add_systems(Startup, build_map )
        .add_systems(Update, build_map )
        .add_systems( Update, handle_input )
        .add_systems( Update, on_gamestate_changed )
        .add_systems( Update, draw_split_feedback )
        .add_systems( Update, player_guidance )
        .add_systems( Update, update_ai )
        .add_systems( Update, update_circ_anim )
        .add_systems( Update, update_ui )
        .add_systems( Update, player_settings )
        .add_event::<GameStateChanged>()
        .add_event::<TurnAdvance>()
        .add_event::<PlayerSettingsChanged>()
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    //mut cards: ResMut<CardDeck>,
    mut stuff: ResMut<GoodStuff>,
    mut config_store: ResMut<GizmoConfigStore>,
    mut ev_settings: EventWriter<PlayerSettingsChanged>,
    //game: Res<GameState>,
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

    stuff.player_stuff[1].color  = Color::rgb_u8(1, 205, 254);
    stuff.player_stuff[1].color2 = Color::rgb_u8(1, 150, 114);

    stuff.player_stuff[2].color  = Color::rgb_u8(5, 254, 161);
    stuff.player_stuff[2].color2 = Color::rgb_u8(1, 152, 30);

    // stuff.player_stuff[3].color  = Color::rgb_u8(185, 103, 255);
    // stuff.player_stuff[3].color2 = Color::rgb_u8(52, 37, 174);
    stuff.player_stuff[3].color  = Color::rgb_u8(161, 39, 255);
    stuff.player_stuff[3].color2 = Color::rgb_u8(52, 37, 174);

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

    // cursor cube for easier debugging
    // commands.spawn((PbrBundle {
    //     mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
    //     material: materials.add(Color::rgb_u8(255, 144, 10)),
    //     transform: Transform::from_xyz(5.0, 0.5, 5.0),
    //     ..default()
    // }, GameCursor { ndx : 0,
    //     drag_from : None, drag_dest : None, cursor_world : Vec3::ZERO, split_pct : 0.5,
    //     } ));



    // cursor with no cube
    commands.spawn((GameCursor { ndx : 0,
            drag_from : None, _drag_dest : None, cursor_world : Vec3::ZERO, split_pct : 0.5,
            }, Transform::default() ));

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
        transform: Transform::from_xyz(-4.0, 10.0, 1.0),
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

        commands.spawn((
            TextBundle::from_section(
                "Hello CyberSummoner\n\
                Instructions go here",
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
            PlayerHelp,
        ));


        commands.spawn((
            TextBundle::from_section("00",
                TextStyle {
                    font_size: 30.,
                    ..default()
                },
            )
            .with_style( Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
            SplitLabel { is_dest : true },
        ));

        commands.spawn((
            TextBundle::from_section("00",
                TextStyle {
                    font_size: 30.,
                    ..default()
                },
            )
            .with_style( Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            }),
            SplitLabel { is_dest : false },
        ));


    commands.spawn( AIController {
        turn_timer : Timer::new(Duration::from_secs_f32( 3.0 ), TimerMode::Once),
    });


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

    commands.spawn((SpriteBundle {
        texture: asset_server.load("title.png"),
        transform: Transform::from_xyz( 0.0, 0.0, 3.0 ).with_scale( Vec3::splat( 0.6)),
        ..default()
    }, TitleScreenCrap ));

    // setup player status
    stuff.player_stuff[0].ptype = PlayerType::Local;
    stuff.player_stuff[1].ptype = PlayerType::AI;
    stuff.player_stuff[2].ptype = PlayerType::AI;
    stuff.player_stuff[3].ptype = PlayerType::NotActive;

    let mut yy = 350.0;
    for i in 0..4 {

            commands.spawn((
                TextBundle::from_section("Player # -- ???",
                    TextStyle {
                        color: stuff.player_stuff[i].color,
                        font_size: 30.,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(yy),
                    left: Val::Px( 300.0),
                    ..default()
                }),
                PlayerSetting(i as i32),
                TitleScreenCrap) );
            yy += 30.0;
    }

    ev_settings.send( PlayerSettingsChanged );


    commands.spawn((SpriteBundle {
        texture: asset_server.load("turn_frame.png"),
        transform: Transform::from_xyz( 500.0, 0.0, 1.0 ),
        ..default()
    }, RoundScoringFrame )).with_children(|parent| {
        for i in 0..4 {
            let icon_y  = 150.0 - (i as f32) * 100.0;
            parent.spawn(( SpriteBundle {
                texture: asset_server.load("icon_rat.png"),
                transform: Transform::from_xyz( 30.0, icon_y, 2.0 ).with_scale( Vec3::splat( 0.6 )),
                ..default()
            }, RoundIcon(i)));

            // Turn indicators
            for j in 0..4 {
                parent.spawn( (SpriteBundle {
                    sprite : Sprite {
                        color : Color::rgba( 1.0, 1.0, 1.0, 0.02 ),
                        ..default()
                    },
                    texture: asset_server.load("hex.png"),
                    transform: Transform::from_xyz(  (j as f32) * 25.0, icon_y - 50.0, 2.0 ).with_scale( Vec3::splat( 0.3 )),
                    ..default()
                }, TurnIcon(i*4 + j)));
            }
        }
    });

    let mut xx = 12.0;
    for i in 0..4 {
        //if (stuff.player_stuff[i].ptype != PlayerType::NotActive)
        //{
            commands.spawn((
                TextBundle::from_section("0",
                    TextStyle {
                        color: stuff.player_stuff[i].color,
                        font_size: 42.,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(xx),
                    left: Val::Px(12.0),
                    ..default()
                }),
                PlayerScore( i as i32),
            ));

            xx += 50.0;
        //}
    }


}


// fn test_start_game (
//     // mut world : &mut World,
//     mut commands: Commands,
//     keyboard_input: Res<ButtonInput<KeyCode>>,
// )
// {
//     if keyboard_input.just_pressed( KeyCode::KeyW ) {
//         println!("W pressed");
//         //world.run_system_once(build_map);
//     }
// }

// world.run_system_once(count_entities);
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



fn handle_input(
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    mut cursor_q: Query<(&mut Transform, &mut GameCursor)>,
    maptile_query: Query<(Entity, &GlobalTransform, &MapSpaceVisual), With<MapSpaceVisual>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    stuff: Res<GoodStuff>,
    mut game: ResMut<GameState>,
    mut ev_gamestate: EventWriter<GameStateChanged>,
    mut ev_turn: EventWriter<TurnAdvance>,
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

        cursor_info.ndx = ndx;
        cursor_info.cursor_world = point;

        let active_player = game.player_turn;

        // Figure out split amount based on distance
        if cursor_info.drag_from.is_some() {

            let drag_from_ndx = cursor_info.drag_from.unwrap() as i32;
            let drag_from_pos = worldpos_from_mapindex(drag_from_ndx as i32);
            let d = cursor_info.cursor_world.distance( drag_from_pos );
            let dnorm = ((d - 1.0).max(0.0) / 3.0).min( 1.0);

            //println!("Dist is {}, {}", d, dnorm );
            cursor_info.split_pct = dnorm;
        }

        if mouse_button_input.just_pressed(MouseButton::Left) {

            // Make sure there is some power to drag from
            if (ndx != INVALID) && (game.snapshot.map.spaces[ ndx ].power > 1 ) &&
            (game.snapshot.map.spaces[ ndx ].player == (active_player + 1) as u8 ) {
                cursor_info.drag_from = Some( ndx );
                println!("Drag from: {}", ndx );
            }
        }

        if mouse_button_input.just_released(MouseButton::Left) {

            if cursor_info.drag_from.is_some() {

                let drag_from_ndx = cursor_info.drag_from.unwrap() as i32;
                let drag_from_pos = worldpos_from_mapindex(drag_from_ndx as i32);

                let mapdir = mapdir_from_drag( cursor_info.cursor_world, drag_from_pos );
                let found = game.snapshot.map.search_dir( drag_from_ndx,  mapdir );
                if (found != drag_from_ndx) && (found != gamestate::INVALID as i32)
                {
                    let found_ndx = found as usize;
                    if game.snapshot.map.spaces[ found_ndx ].player == 0 {

                        let src_pow = game.snapshot.map.spaces[ drag_from_ndx as usize ].power as i32;
                        let split_count = calc_split(cursor_info.split_pct, src_pow);
                        if split_count > 0 {
                            game.snapshot.map.spaces[ found_ndx ].player = (active_player + 1) as u8;
                            game.snapshot.map.spaces[ found_ndx ].power = split_count as u8;
                            //ev_gamestate.send( GameStateChanged::CircleAdded( found_ndx as i32) );
                            ev_gamestate.send( GameStateChanged::CircleSplit( drag_from_ndx, found_ndx as i32) );

                            game.snapshot.map.spaces[ drag_from_ndx as usize].power -= split_count as u8;
                            ev_gamestate.send( GameStateChanged::CircleAdded( drag_from_ndx) );


                            // Advance to the next player's turn
                            let mut pnum = game.player_turn;
                            loop {
                                pnum = pnum + 1;
                                if pnum >= stuff.player_stuff.len() as i32 {
                                    pnum = 0;
                                }

                                if stuff.player_stuff[pnum as usize].ptype != PlayerType::NotActive {
                                    break;
                                }

                                if pnum == game.player_turn {
                                    println!("Didn't find any active players?");
                                    break;
                                }
                            }
                            game.player_turn = pnum;
                            game.turn_num += 1;

                            game.snapshot.update_scores();

                            ev_turn.send( TurnAdvance(pnum) );
                        }
                    }
                }
            }
        }

        if !mouse_button_input.pressed(MouseButton::Left) {
            if cursor_info.drag_from.is_some() {
                println!("Drag clear" );
            }
            cursor_info.drag_from = None;
        }
    }
}

fn draw_map_dir( gizmos: &mut Gizmos, game : &GameState, ndx : i32, dir : MapDirection, color : Color, verbose : bool ) -> Vec3
{
    let found = game.snapshot.map.search_dir( ndx,  dir );
    if verbose {
        let dir_str = format!("{:?}", dir);
        let dir_str_padded = format!("{:<10}", dir_str);
        println!("   {} {} Open {}", dir_str_padded, gamestate::move_dir( ndx, dir ),  found );
    }
    if (found != ndx) && (found != gamestate::INVALID as i32) {
        let pos_a = worldpos_from_mapindex(ndx) + Vec3::Y * 0.25;
        let pos_b = worldpos_from_mapindex(found) + Vec3::Y * 0.25;
        gizmos.line(pos_a, pos_b, color );
        gizmos.cuboid(
            Transform::from_translation(pos_b), //.with_scale(Vec3::splat(1.25)),
            color );

        // Return the found pos
        pos_b

    } else {
        Vec3::ZERO
    }


}

fn mapdir_from_drag( pos : Vec3, start_pos : Vec3 ) -> MapDirection
{
    // get best angle from arrow
    let dir = pos - start_pos;
    let angle = dir.z.atan2(dir.x);
    let mut angle_degrees = angle.to_degrees() + (90.0 + 30.0);
    if angle_degrees < 0.0 {
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
    camera_q: Query<(&Camera, &Transform, &GlobalTransform), With<GameCamera>>,
    mut label_q: Query<(&SplitLabel, &mut Style, &mut Text, &mut Visibility)>,
    stuff: Res<GoodStuff>,
    game: Res<GameState>,
    mut gizmos: Gizmos,
)
{
    let offs = Vec3 { x : 0.0, y : 0.15, z : 0.0 };

    let ( _cursor_transform, cursor_info) = cursor_q.single();
    let player_col = stuff.player_stuff[ game.player_turn as usize].color;

    if cursor_info.drag_from.is_some() {
        // Draw a gizmo for drag_from
        let drag_from_ndx = cursor_info.drag_from.unwrap();
        let drag_from_pos = worldpos_from_mapindex(drag_from_ndx as i32);
        gizmos.arrow( drag_from_pos + offs, cursor_info.cursor_world + offs, Color::YELLOW );

        // cursor_info.cursor_world - drag_from_pos;
        let mapdir = mapdir_from_drag( cursor_info.cursor_world, drag_from_pos );
        let dst_pos = draw_map_dir( &mut gizmos, &game, drag_from_ndx as i32, mapdir, player_col, false);

        let src_pow = game.snapshot.map.spaces[ drag_from_ndx ].power as i32;
        let split_count = calc_split(cursor_info.split_pct, src_pow);

        for (lblinfo, mut style, mut label, mut vis) in &mut label_q {

            let wpos;
            if lblinfo.is_dest {
                label.sections[0].value = format!("{}", split_count );
                wpos = dst_pos;
            } else {
                label.sections[0].value = format!("{}", src_pow - split_count );
                wpos = drag_from_pos;
            }
            label.sections[0].style.color = player_col;

            let (camera, _camera_transform, camera_global_transform) = camera_q.single();
            let viewport_position = camera
                .world_to_viewport(camera_global_transform, wpos)
                .unwrap();

            style.top = Val::Px(viewport_position.y);
            style.left = Val::Px(viewport_position.x);

            *vis = Visibility::Visible;

        }

        // println!( "Drag angle: {} degrees dir {:?}", angle_degrees, mapdir );
    } else {
        // not dragging, should we show preview?
        let ndx = cursor_info.ndx as i32;

        // Hide the split labels
        for (_, _, _, mut vis) in &mut label_q {
            *vis = Visibility::Hidden;
        }

        // look at the hovered square
        if (ndx >= 0) && (ndx < 100) {
            let mapsq = game.snapshot.map.spaces[ ndx as usize ];

            // TODO: player check
            if (mapsq.contents == MapSpaceContents::Playable) && (mapsq.power > 1) && (mapsq.player == (game.player_turn + 1) as u8) {
                draw_map_dir( &mut gizmos, &game, ndx, MapDirection::North, player_col, false);
                draw_map_dir( &mut gizmos, &game, ndx, MapDirection::NorthEast,player_col,  false );
                draw_map_dir( &mut gizmos, &game, ndx, MapDirection::SouthEast,player_col,  false);
                draw_map_dir( &mut gizmos, &game, ndx, MapDirection::South, player_col, false);
                draw_map_dir( &mut gizmos, &game, ndx, MapDirection::SouthWest,player_col,  false);
                draw_map_dir( &mut gizmos, &game, ndx, MapDirection::NorthWest, player_col, false );
            }
        }

    }

}

fn calc_split( split_pct : f32, src_pow: i32) -> i32 {
    let split_count = split_pct * ((src_pow - 1) as f32);
    let split_count = split_count as i32;
    split_count
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
    mut stuff: ResMut<GoodStuff>,
    mut commands: Commands,
    mut gamestate: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ev_gamestate: EventWriter<GameStateChanged>,
    mut ev_turn: EventWriter<TurnAdvance>,
    mut ev_settings: EventWriter<PlayerSettingsChanged>,
    titlescreen_q : Query<Entity, With<TitleScreenCrap>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
)
{

    // already built
    if gamestate.player_count > 0 {
        return;
    }

    /*
    // wait to start this here because of browser audio stuff... need to find a better way
    // MUUUUSSSIICC
    commands.spawn(AudioBundle {
        source: asset_server.load("SummoningStuff_OGG.ogg"),
        settings: PlaybackSettings::LOOP,
        ..default()
    });
    */

    // this all sucks but the contest is ending
    if gamestate.player_count == 0 {

        let should_run = keyboard_input.just_pressed( KeyCode::Enter ) || keyboard_input.just_pressed( KeyCode::Space );


        let mut z = -1;
        if keyboard_input.just_pressed( KeyCode::Digit1) {
            z = 0;
        }

        if keyboard_input.just_pressed( KeyCode::Digit2) {
            z = 1;
        }

        if keyboard_input.just_pressed( KeyCode::Digit3) {
            z = 2;
        }

        if keyboard_input.just_pressed( KeyCode::Digit4) {
            z = 3;
        }

        if z >= 0 {
            let z = z as usize;
            if stuff.player_stuff[z].ptype == PlayerType::Local {
                stuff.player_stuff[z].ptype = PlayerType::AI;
            } else if stuff.player_stuff[z].ptype == PlayerType::AI {
                stuff.player_stuff[z].ptype = PlayerType::NotActive;
            } else if stuff.player_stuff[z].ptype == PlayerType::NotActive {
                stuff.player_stuff[z].ptype = PlayerType::Local;
            }

            ev_settings.send( PlayerSettingsChanged );
        }

        let mut pcount : i32 = 0;
        for i in 0..4 {
            if stuff.player_stuff[i].ptype != PlayerType::NotActive {
                pcount += 1;
            }
        }

        if !should_run || pcount == 0 { return };
    }

    // Despawn all the title screen stuff
    for e in &titlescreen_q {
        commands.entity(e).despawn_recursive();
    }

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

fn player_settings(
    stuff: Res<GoodStuff>,
    mut setting_q: Query<(&mut Text, &PlayerSetting)>,
    mut ev_settings: EventReader<PlayerSettingsChanged>,
) {
    for ev in ev_settings.read() {

        for (mut text, plr) in &mut setting_q {

            let plr_type = match stuff.player_stuff[plr.0 as usize].ptype {
                PlayerType::Local => "Human",
                PlayerType::AI => "AI",
                PlayerType::NotActive => "None",
            };

            text.sections[0].value = format!("Player {} -- {}", plr.0 + 1, plr_type);
        }

    }

}


fn player_guidance(
    //mut commands: Commands,
    mut stuff: ResMut<GoodStuff>,
    game: Res<GameState>,
    //mut helper_q: Query<(&mut Text, &mut Style), With<PlayerHelp>>,
    mut helper_q: Query<&mut Text, With<PlayerHelp>>,
    mut turnicon_q: Query<(&mut Sprite, &TurnIcon)>,
    mut score_q: Query<(&mut Text, &PlayerScore), Without<PlayerHelp>>,
    mut ev_turn: EventReader<TurnAdvance>, )
{
    for ev in ev_turn.read() {

        let mut text = helper_q.single_mut();
        let pinfo = &mut stuff.player_stuff[ev.0 as usize];
        //text.style.color = pinfo.color;
        text.sections[0].style.color = pinfo.color;

        let moves = gen_valid_moves( game.snapshot, ev.0 as usize);
        if moves.is_empty() {
            pinfo.out_of_moves = true;

            text.sections[0].value = if pinfo.ptype == PlayerType::Local {
                format!("Player {} has no moves and must pass.", ev.0 + 1 )
            } else {
                "Computer Player is out of moves and must pass".into()
            }

        } else {
            text.sections[0].value = if pinfo.ptype == PlayerType::Local {
                format!("Player {}'s turn.", ev.0 + 1 )
            } else {
                "Waiting for Computer Player".into()
            }
        }

        if game.player_count > 0 {
            let icon_n = game.turn_num / game.player_count;
            for (mut sprite, turn) in &mut turnicon_q {
                if turn.0 < icon_n {
                    sprite.color = Color::rgba( 1.0, 1.0, 1.1, 0.3 );
                } else if turn.0 == icon_n {
                    sprite.color = pinfo.color;
                }
            }

            // Update score displays
            for (mut text, score) in &mut score_q {
                text.sections[0].value = format!( "{:02}", game.snapshot.score[ score.0 as usize ]);

            }
        }

    }
}


fn on_gamestate_changed(
    mut commands: Commands,
    stuff: Res<GoodStuff>,
    gamestate: Res<GameState>,
    mut q_mapvis : Query<&mut MapSpaceVisual>,
    mut ev_gamestate: EventReader<GameStateChanged>, )
{
    for ev in ev_gamestate.read() {

        let mut split_from_ndx : Option<usize> = None;
        let spawn_ndx;

        match ev {
            GameStateChanged::CircleAdded(ndx ) => {
                spawn_ndx = *ndx as usize;
            }
            GameStateChanged::CircleSplit( src, dest) => {
                spawn_ndx = *dest as usize;
                split_from_ndx = Some(*src as usize);
            }
        }

        if spawn_ndx != INVALID
        {

            let spc = gamestate.snapshot.map.spaces[spawn_ndx];
            println!("Added circle at {}, power is {}, player {}", spawn_ndx, spc.power, spc.player  );

            // Get the maptile entity that is the parent


            // Remove any existing childs
            let ent_vis = gamestate.map_visuals[spawn_ndx];
            let vis = q_mapvis.get( gamestate.map_visuals[spawn_ndx]).unwrap();
            match vis.circle {
                Some(child_ent) => {
                    commands.entity(ent_vis).remove_children( &[ child_ent ]);
                    commands.entity( child_ent ).despawn();
                }
                None => {}
            }

            //commands.entity(ent_vis).
            let ring_sz = if spc.power == 1 { 0.9 } else { 1.25 };

            let targ_pos = Vec3 { x: 0.0, y : 0.2, z : 0.0 };
            let mut spawn_pos = targ_pos;
            if split_from_ndx.is_some() {
                let split_from_ndx = split_from_ndx.unwrap();
                let start_pos = worldpos_from_mapindex( split_from_ndx as i32 );
                let targ_pos_w = worldpos_from_mapindex( spawn_ndx as i32 );

                spawn_pos = (start_pos - targ_pos_w) + targ_pos;
                //println!( "Spawn Pos is {:?}", spawn_pos );
            }


            let mtl = stuff.player_stuff[spc.player as usize - 1].ring_mtl[ (spc.power as usize) - 1 ].clone();
            let ent_ring =

            commands.spawn((PbrBundle {
                mesh: stuff.ring_mesh.clone(),
                material: mtl,
                transform: Transform {
                    translation : spawn_pos,
                    scale: Vec3::splat( ring_sz ),
                    ..default()
                },
                ..default()
            }, NotShadowCaster, CircleAnimator { target : targ_pos }) ).id();


            let mut vis = q_mapvis.get_mut( gamestate.map_visuals[spawn_ndx] ).unwrap();
            vis.circle = Some(ent_ring);

            commands.entity(ent_vis).add_child(ent_ring);
        }
    }
}

fn update_ai(
    //mut commands: Commands,
    time: Res<Time>,
    stuff: Res<GoodStuff>,
    mut q_ai : Query<&mut AIController>,
    mut ev_turn: EventWriter<TurnAdvance>,
    mut ev_gamestate: EventWriter<GameStateChanged>,
    mut game: ResMut<GameState>,
) {
    let pinfo = &stuff.player_stuff[game.player_turn as usize];
    let mut should_advance_turn = false;
    let mut ai = q_ai.single_mut();
    if pinfo.ptype == PlayerType::Local && pinfo.out_of_moves {
        ai.turn_timer.tick( time.delta());
        if ai.turn_timer.finished() {
            should_advance_turn = true;
        }
    } else if pinfo.ptype == PlayerType::AI {
            ai.turn_timer.tick( time.delta());
            if ai.turn_timer.finished() {
                // Take AI Turn
                let moves = gen_valid_moves( game.snapshot, game.player_turn as usize);
                if moves.is_empty() {
                    println!("AI has no valid moves and will pass.");
                } else {
                    println!("AI has {} valid moves", moves.len() );
                    // Choose a move a random
                    let mut rng = rand::thread_rng();

                    let old = game.snapshot;
                    game.snapshot = moves[ rng.gen_range( 0..moves.len())];

                    let mut curr_strength:i32=-1000000000;
                    for curr_move in moves{
                        let plyr_evals=evaluate_position(curr_move);
                        let mut new_str:i32=rng.gen_range( 0..1000);
                        for player in 0..4{
                            if player==game.player_turn{
                                new_str+=plyr_evals[player as usize]*(game.player_count-1);
                            }
                            else{
                                new_str-=plyr_evals[player as usize];
                            }
                        }
                        if(new_str>curr_strength){
                            curr_strength=new_str;
                            game.snapshot=curr_move;
                        }
                    }

                    // check for splits by looking for decrease
                    let mut split_ndx = None;
                    for mapsq in &game.snapshot.map {
                        let oldsq = old.map.spaces[ mapsq.ndx as usize ];
                        if oldsq.power > mapsq.power {
                            split_ndx = Some( mapsq.ndx );
                            break;
                        }
                    }

                    // Send any adds
                    for mapsq in &game.snapshot.map {
                        let oldsq = old.map.spaces[ mapsq.ndx as usize ];
                        if oldsq.power != mapsq.power {
                            if split_ndx.is_none() || oldsq.power > mapsq.power {
                                ev_gamestate.send( GameStateChanged::CircleAdded( mapsq.ndx ) );
                            } else {
                                ev_gamestate.send( GameStateChanged::CircleSplit( split_ndx.unwrap(), mapsq.ndx ) );
                            }


                        }
                    }
                }

                should_advance_turn = true;
            }
    }

    if should_advance_turn {
        // Reset turn timer
        ai.turn_timer.reset();
        ai.turn_timer.set_duration( Duration::from_secs_f32( 1.0 ) );

        // Advance to the next player's turn
        // todo Wrap this logic up
        let mut pnum = game.player_turn;
        loop {
            pnum = pnum + 1;
            if pnum >= stuff.player_stuff.len() as i32 {
                pnum = 0;
            }

            if stuff.player_stuff[pnum as usize].ptype != PlayerType::NotActive {
                break;
            }

            if pnum == game.player_turn {
                println!("Didn't find any active players?");
                break;
            }
        }
        game.player_turn = pnum;
        game.turn_num += 1;

        game.snapshot.update_scores();

        ev_turn.send( TurnAdvance(pnum) );
    }

}

fn update_ui(
    _time: Res<Time>,
    mut scoreframe_q : Query<&mut Transform, With<RoundScoringFrame>>,
    mut helper_q: Query<&mut Style, (With<PlayerHelp>,Without<RoundScoringFrame>)>,
    mut ev_window: EventReader<WindowResized>,
 )
{
    //zzzwindows.single().sc
    for ev in ev_window.read() {
        println!("Window resized {} {}", ev.width, ev.height );
        let mut xform = scoreframe_q.single_mut();
        xform.translation = Vec3 { x : (ev.width - 177.0) /2.0, y : 0.0, z : 1.0 };

        let mut style = helper_q.single_mut();
        style.top = Val::Px( ev.height - 30.0);
    }
}

fn update_circ_anim( _time: Res<Time>,
    mut circ_q : Query<(&mut Transform, &CircleAnimator)> )
{
    //println!("update_circle_anim");
    for (mut xform, ca ) in &mut circ_q {

        xform.translation = Vec3::lerp( xform.translation, ca.target, 0.1 );
        //println!("xlate {:?} targ {:?}", xform.translation, ca.target );
    }
}
