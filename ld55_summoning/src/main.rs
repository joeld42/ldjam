//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    core_pipeline::{
        //bloom::{BloomCompositeMode, BloomSettings},
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    }, pbr::NotShadowCaster,
        prelude::*, render::{mesh::VertexAttributeValues,
             texture::{ImageAddressMode, ImageSamplerDescriptor}}, window::WindowResized

};


pub mod summongame;
use crate::summongame::*;

pub mod map;
use crate::map::{ build_map, worldpos_from_mapindex };

pub mod gamestate;
use gamestate::{ gen_valid_moves, evaluate_position };

pub mod titlescreen;
use titlescreen::TitleScreenPlugin;

use rand::Rng;

//use std::collections::HashSet;
use std::{f32::consts::PI, time::Duration};

use crate::gamestate::{ MapDirection, MapSpaceContents, INVALID};


#[derive(Component)]
struct GameCamera;

#[derive(Component)]
struct PlayerHelp;

#[derive(Component)]
struct PlayerScore(i32);

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
        .insert_resource( GoodStuff::default() )
        .insert_resource( SummonGame::default() )
        .add_systems(Startup, setup)

        .add_systems( OnEnter(GameAppState::Gameplay), (
            setup_gameplay,
            build_map) )

        .add_systems(Update, (
            handle_input,
            draw_split_feedback,
            on_gamestate_changed,
            player_guidance,
            update_circ_anim,
            update_ui,
            update_ai).run_if(in_state(GameAppState::Gameplay)))

        .add_event::<GameStateChanged>()
        .add_event::<TurnAdvance>()

        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut stuff: ResMut<GoodStuff>,
    mut config_store: ResMut<GizmoConfigStore>,
    asset_server: Res<AssetServer>
) {

    // set up gizmos
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width *= 2.0;


    // Stuff for summoning circles
    let ring_mesh = Mesh::from( Plane3d { normal: Direction3d::Y } ).with_generated_tangents().unwrap();
    stuff.ring_mesh = meshes.add( ring_mesh );

    stuff.player_stuff[0].color  = Color::rgb_u8(255, 113, 206);
    stuff.player_stuff[0].color2 = Color::rgb_u8(161, 45, 172 );

    stuff.player_stuff[1].color  = Color::rgb_u8(1, 205, 254);
    stuff.player_stuff[1].color2 = Color::rgb_u8(1, 150, 114);

    stuff.player_stuff[2].color  = Color::rgb_u8(5, 254, 161);
    stuff.player_stuff[2].color2 = Color::rgb_u8(1, 152, 30);

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

    // HUD
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




    // 2D scene -------------------------------
    commands.spawn( Camera2dBundle {
        camera: Camera {
            hdr: true,
            order: 2, // Draw sprites on top of 3d world
            ..default()
        },
        ..default()
    } );



}

fn build_hud(
    commands: &mut Commands,
    stuff: Res<GoodStuff>,
) {


    // Split feedback labels
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

    // player score labels
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

fn setup_gameplay (
    asset_server: Res<AssetServer>,
    stuff: Res<GoodStuff>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{
    println!("Hello from setup gameplay");
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
    }, Ground) );


    // cursor with no cube
    commands.spawn((GameCursor { ndx : 0,
        drag_from : None, _drag_dest : None, cursor_world : Vec3::ZERO, split_pct : 0.5,
        }, Transform::default() ));

    commands.spawn( AIController {
        turn_timer : Timer::new(Duration::from_secs_f32( 3.0 ), TimerMode::Once),
    });


    build_hud(&mut commands, stuff);
}

fn handle_input(
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    mut cursor_q: Query<(&mut Transform, &mut GameCursor)>,
    maptile_query: Query<(Entity, &GlobalTransform, &MapSpaceVisual), With<MapSpaceVisual>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    stuff: Res<GoodStuff>,
    mut game: ResMut<SummonGame>,
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

fn draw_map_dir( gizmos: &mut Gizmos, game : &SummonGame, ndx : i32, dir : MapDirection, color : Color, verbose : bool ) -> Vec3
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
    game: Res<SummonGame>,
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



// fn spawn_mapspace_empty( mut commands: Commands ) -> Entity {
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
//         material: materials.add(Color::rgb_u8(124, 144, 255)),
//         transform: Transform::from_xyz(0.0, 0.5, 0.0),
//         ..default()
//     }).id()
// }


fn player_guidance(
    //mut commands: Commands,
    mut stuff: ResMut<GoodStuff>,
    game: Res<SummonGame>,
    //mut helper_q: Query<(&mut Text, &mut Style), With<PlayerHelp>>,
    mut helper_q: Query<&mut Text, With<PlayerHelp>>,
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
    gamestate: Res<SummonGame>,
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
    mut game: ResMut<SummonGame>,
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
                        if new_str > curr_strength {
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
    mut helper_q: Query<&mut Style, With<PlayerHelp>>,
    mut ev_window: EventReader<WindowResized>,
 )
{
    for ev in ev_window.read() {
        println!("Window resized {} {}", ev.width, ev.height );

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
