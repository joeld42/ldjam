use bevy::{asset::{AssetMetaCheck, RenderAssetUsages},
    render::view::RenderLayers,
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::CascadeShadowConfigBuilder, prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    scene::SceneInstanceReady, window::{WindowResized, WindowResolution}};

use bevy_skein::SkeinPlugin;

use std::f32::consts::PI;

//use bevy_inspector_egui::quick::WorldInspectorPlugin;

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

const SCAN_ROWS : usize = 128;
const SCAN_RES : usize = 64;
const SCAN_COLOR: Srgba = Srgba::new(0.255, 0.412, 0.882, 1.0);

/// Store the image handle that we will draw to, here.
#[derive(Resource)]
struct SonarImage(Handle<Image>);


#[derive(Default, Clone)]
struct ScanRow {
    pos_a : Vec3,
    pos_b : Vec3,
    spawn_time : f32,
}

#[derive(Resource, Default)]
struct SonarScan {
    is_scanning : bool,
    curr_row : usize,
    row_timeout : f32,
    rows : Vec<ScanRow>,
}

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

        if throttle_decay {
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
          for (_ent,hit) in hits {
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

// See https://www.shadertoy.com/view/ll2GD3
fn pal(t : f32, a : Vec3, b : Vec3, c : Vec3, d : Vec3 ) -> Vec3
{
    let CF = 6.28318;
    return a +
       b * Vec3{
            x: (CF * (c.x * t + d.x )).cos(),
            y: (CF * (c.y * t + d.y )).cos(),
            z: (CF * (c.z * t + d.z )).cos(),
     };
    //return a + b*cos( 6.28318*(c*t+d) );
}

fn update_sonar(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    sonar_img: Res<SonarImage>,
    terrain_q: Query<(), With<Seafloor>>,
    mut ray_cast: MeshRayCast,
    mut images: ResMut<Assets<Image>>,
    mut sonar: ResMut<SonarScan>,
    mut gizmos: Gizmos,
    player_q: Query<(&PlayerBoat, &Transform), With<PlayerBoat>>,
)
{
    // let window = window.single();
    // let width = window.resolution.width();
    // println!("Window width is {:}", width );

    if keys.just_pressed(KeyCode::Space) {
        // Space was pressed
        if sonar.is_scanning {
            println!( "Already scanning...");
        } else {
            println!("TODO: Start Scan");
            sonar.is_scanning = true;
            sonar.curr_row = 0;
            sonar.row_timeout = 0.0;
        }
    }

    let now = time.elapsed_secs();
    let scan_interval = 0.1; // time in seconds
    if sonar.is_scanning {
        sonar.row_timeout -= time.delta_secs();
        if (sonar.row_timeout <= 0.0) {
            // scan a row
            sonar.row_timeout = scan_interval;
            let next_row = sonar.curr_row;


            if let Ok((mut pboat, mut pxform)) = player_q.get_single() {


                let scan_width = 5.0;
                let scan_offs = 0.5;
                let pos_a = pxform.translation + (pxform.rotation * Vec3{ x: -scan_width, y: 0.1, z: scan_offs });
                let pos_b = pxform.translation + (pxform.rotation * Vec3{ x:  scan_width, y: 0.1, z: scan_offs });

                sonar.rows[ next_row ] = ScanRow {
                    pos_a : pos_a,
                    pos_b : pos_b,
                    spawn_time : now,
                };

                // vec3(0.5,0.5,0.5),vec3(0.5,0.5,0.5),vec3(1.0,1.0,0.5),vec3(0.8,0.90,0.30)
                let palA = Vec3{ x : 0.5, y : 0.5, z : 0.5 };
                let palB = Vec3{ x : 0.5, y : 0.5, z : 0.5 };
                let palC = Vec3{ x : 1.0, y : 1.0, z : 0.5 };
                let palD = Vec3{ x : 0.8, y : 0.9, z : 0.3 };

                // Get the image from Bevy's asset storage.
                let filter = |entity| terrain_q.contains(entity);
                let settings = RayCastSettings::default().with_filter(&filter);
                let image = images.get_mut(&sonar_img.0).expect("Image not found");
                for pp in 0..SCAN_RES {

                    let pt = (pp as f32) / (SCAN_RES as f32);
                    let p = pos_a.lerp( pos_b, pt );

                    let ray = Ray3d::new(p + Vec3 { x: 0.0, y: 100.0, z: 0.0 }, Dir3::NEG_Y);
                    let hits = ray_cast.cast_ray(ray, &settings);

                    let shallow = Color::srgb( 0.0, 0.5, 1.0 );
                    let deep = Color::srgb( 1.0, 0.7, 0.0 );
                    let mut scan_col = Color::srgb_u8(97,72, 60 );
                    if let Some((_ent, hit)) = hits.first() {
                        let dist = hit.distance - 100.0;
                        if (dist > 0.0) {
                            let t = (dist / 10.0).clamp( 0.0, 1.0);
                            //scan_col = shallow.mix( &deep, t );
                            let cc = pal( t, palA, palB, palC, palD );
                            scan_col = Color::srgb( cc.x, cc.y, cc.z );
                        }
                    }



                    image
                        .set_color_at( next_row as u32, pp as u32, scan_col )
                        .unwrap();
                }


                sonar.curr_row += 1;
                //println!("Scanned row {:}", sonar.curr_row );
                if sonar.curr_row >= SCAN_ROWS {
                    sonar.is_scanning = false;
                }
            }
        }
    }

    // Draw scan lines
    for i in 0..sonar.curr_row {
        let row = &sonar.rows[i];
        let age = 1.0 - (now - row.spawn_time).clamp( 0.0, 5.0) / 5.0;

        gizmos.line(row.pos_a, row.pos_b, SCAN_COLOR * (5.0 + (age*age) * 20.0) );

    }
}

fn resize_notificator(mut events: EventReader<WindowResized>) {
    for e in events.read() {
        println!("width = {} height = {}", e.width, e.height);
    }
}

fn follow_cam(
    time: Res<Time>,
    player_q: Query<(&PlayerBoat, &Transform), With<PlayerBoat>>,
    mut followcam_q : Query<(&mut Transform, &mut FollowCam), Without<PlayerBoat>>
) {

    if let Ok(( _pboat, pxform)) = player_q.get_single() {
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

                    resolution: WindowResolution::new(948., 533. ),

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
                    let Ok(_cam) =
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

        //.add_plugins(WorldInspectorPlugin::new())

        .add_systems(Startup, startup)
        .add_systems( Update,
            (resize_notificator,
                update_sonar,
                player_controls,
                     follow_cam) )
        .run();
}


fn startup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
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

    // Set up Sonar Image
    let mut image = Image::new_fill(
        // 2D image of size 256x256
        Extent3d {
            width: SCAN_ROWS as u32,
            height: SCAN_RES as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        // Initialize it with a beige color
        &([0, 0, 0, 0xff]),
        // Use the same encoding as the color we set
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    // To make it extra fancy, we can set the Alpha of each pixel,
    // so that it fades out in a circular fashion.
    // for y in 0..IMAGE_HEIGHT {
    //     for x in 0..IMAGE_WIDTH {
    //         let center = Vec2::new(IMAGE_WIDTH as f32 / 2.0, IMAGE_HEIGHT as f32 / 2.0);
    //         let max_radius = IMAGE_HEIGHT.min(IMAGE_WIDTH) as f32 / 2.0;
    //         let r = Vec2::new(x as f32, y as f32).distance(center);
    //         let a = 1.0 - (r / max_radius).clamp(0.0, 1.0);

    //         // Here we will set the A value by accessing the raw data bytes.
    //         // (it is the 4th byte of each pixel, as per our `TextureFormat`)

    //         // Find our pixel by its coordinates
    //         let pixel_bytes = image.pixel_bytes_mut(UVec3::new(x, y, 0)).unwrap();
    //         // Convert our f32 to u8
    //         pixel_bytes[3] = (a * u8::MAX as f32) as u8;
    //     }
    // }



    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true, // 1. HDR is required for bloom
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
        Transform::from_xyz(-2.0, 20.5, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        Bloom::NATURAL, // 3. Enable bloom for the camera
        FollowCam::default(),
    ));


    let handle = images.add(image);

    // Create a sprite entity using our image
    commands.spawn((Sprite::from_image(handle.clone()),
    Transform::from_xyz(0., -180., 0.).with_scale( Vec3 { x: 4.0, y: 2.0, z : 1.0 }),
        RenderLayers::layer(1) ) );
    commands.insert_resource(SonarImage(handle));


    commands.insert_resource(
        SonarScan{
            rows: vec![ScanRow::default(); SCAN_ROWS],
        ..default() } );

    // 2D Overlay scene
    commands.spawn((Camera {
            hdr: true,
            order: 2, // Draw sprites on top of 3d world
            ..default()
        }, Camera2d, RenderLayers::layer(1) ));

    // commands.spawn((
    //     Sprite::from_image(asset_server.load("bevy_bird_dark.png")),
    //     Transform::from_xyz(-200., 0., 0.),
    // ));


}
