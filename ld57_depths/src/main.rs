use bevy::{asset::AssetMetaCheck, core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, pbr::CascadeShadowConfigBuilder, prelude::*, scene::SceneInstanceReady, window::WindowResolution};
use bevy_skein::SkeinPlugin;

use std::f32::consts::PI;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[reflect(Default)]
struct FollowCam {
    offset : Vec3,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[reflect(Default)]
struct PlayerBoat{
    throttle: f32,
    turn: f32,
    velocity: Vec3,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct Seafloor;


fn player_controls(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,

    terrain_q: Query<(), With<Seafloor>>,
    mut ray_cast: MeshRayCast,
    mut player_q: Query<(&mut PlayerBoat, &mut Transform), With<PlayerBoat>>,
) {

    if let Ok((mut pboat, mut pxform)) = player_q.get_single_mut() {
        // if pboat.is_err() {
        //     // player not loaded yet
        //     return;
        // }
        // let (mut pboat, mut pxform) = pboat.unwrap();

        if keys.just_pressed(KeyCode::Space) {
            // Space was pressed
            println!("Space was pressed...");
        }

        // if keys.just_released(KeyCode::ControlLeft) {
        //     // Left Ctrl was released
        // }
        // if keys.pressed(KeyCode::KeyW) {
        //     // W is being held down
        // }
        // // we can check multiple at once with `.any_*`
        // if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        //     // Either the left or right shift are being held down
        // }
        // if keys.any_just_pressed([KeyCode::Delete, KeyCode::Backspace]) {
        //     // Either delete or backspace was just pressed
        // }

        let mut throttle_decay = true;
        let mut throttle : f32 = 0.0;
        let engine_speed = 0.1;
        if keys.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            throttle += engine_speed * time.delta_secs();
            throttle_decay = false;
        }
        if keys.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            throttle -= engine_speed * time.delta_secs();
            throttle_decay = false;
        }

        let turn_speed = 90.0;
        let mut steer = 0.0;
        if keys.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            steer += turn_speed * time.delta_secs();
        }
        if keys.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            steer -= turn_speed * time.delta_secs();
        }

        if (throttle_decay) {
            pboat.throttle *= 0.99;
            //println!("throttle decay {:}", pboat.throttle);
        }

        pboat.throttle += throttle;

        let fwd = pxform.rotation * Vec3::NEG_Z;
        // pxform.translation += fwd * pboat.throttle;
        let next_pos = pxform.translation + (fwd * pboat.throttle);

        pxform.rotate_local_y( steer.to_radians() );

        // Check current depth
         let ray = Ray3d::new(next_pos + Vec3 { x: 0.0, y: 100.0, z: 0.0 }, Dir3::NEG_Y);
          let filter = |entity| terrain_q.contains(entity);

         let settings = RayCastSettings::default().with_filter(&filter);
          let hits = ray_cast.cast_ray(ray, &settings);

            let mut collide = false;
          for (ent,hit) in hits {
            let depth = hit.distance - 100.0;
            //print!("Hit {:} -- {:?}", ent, depth );
            if depth <= -0.2 {
                collide = true;
                break;
            }
          }

          if !collide {
            pxform.translation = next_pos;
          } else {
            pboat.throttle = 0.0;
          }


    }
}

fn follow_cam(
    time: Res<Time>,
    player_q: Query<(&PlayerBoat, &Transform), With<PlayerBoat>>,
    mut followcam_q : Query<(&mut Transform, &mut FollowCam), Without<PlayerBoat>>
) {

    if let Ok((pboat, pxform)) = player_q.get_single() {
        followcam_q.iter_mut().for_each(|(mut cam_xform, mut followcam)|{

            if followcam.offset.length_squared() < 0.001 {
                // Assign offset to our initial offset if not yet
                followcam.offset = cam_xform.translation - pxform.translation;
            }

            //cam_xform.translation = pxform.translation + Vec3 { x: 0.0, y:10.0, z:-5.0 };
            // Smoothly move the camera towards the target position
            let target_position = pxform.translation + (pxform.rotation * followcam.offset);
            cam_xform.translation = cam_xform
                .translation
                .lerp(target_position, time.delta_secs() * 0.9 ); // Adjust the speed as needed

            // Optionally, make the camera look at the player
            cam_xform.look_at(pxform.translation, Vec3::Y);

        });
    }
}

fn main() {
    App::new()
        .register_type::<PlayerBoat>()
        .register_type::<FollowCam>()
        .register_type::<Seafloor>()
        .add_plugins((
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {

                    // provide the ID selector string here
                    canvas: Some("#game-canvas".into()),
                    fit_canvas_to_parent: true,
                    ..default()
                    }),
                ..default()
        }).set(
        AssetPlugin {
                file_path: "ld57assets".to_string(),
                meta_check: AssetMetaCheck::Never,
                    ..Default::default()
                }
        )  ,
        SkeinPlugin::default() ))

        .add_observer(
            |trigger: Trigger<SceneInstanceReady>,
             children: Query<&Children>,
             mut commands: Commands,
             cameras: Query<&FollowCam, With<Camera>>,
             players: Query<&PlayerBoat>| {
                for entity in children
                    .iter_descendants(trigger.entity())
                {
                    let Ok(player) =
                        players.get(entity)
                    else {
                        continue;
                    };
                    println!("Got player (entity is {:})", entity);
                    info!(?player);
                }

                // Add bloom to cameras
                for ent in children.iter_descendants( trigger.entity()) {
                    let Ok(cam) =
                        cameras.get(ent)
                    else {
                        continue;
                    };
                    println!("Got camera (entity is {:})", ent );
                    commands.entity(ent).insert((
                        Tonemapping::TonyMcMapface,
                        Bloom::default(),
                    ));

                    info!(?ent);
                }

            },
        )

        .add_plugins(WorldInspectorPlugin::new())

        .add_systems(Startup, startup)
        .add_systems( Update,
            (player_controls,
                     follow_cam) )
        .run();
}


fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    commands.spawn(SceneRoot(asset_server.load(
        // Change this to your exported gltf file
        GltfAssetLabel::Scene(0).from_asset("depthfish_proto.glb"),
    )));

    // Add a directional light
    // directional 'sun' light
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 10.0,
            maximum_distance: 200.0,
            ..default()
        }
        .build(),
    ));


}
