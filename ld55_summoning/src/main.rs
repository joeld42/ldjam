//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;

fn main() {

    App::new()    
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some( Window {
                title: "LD55 Summoning".into(),
                canvas: Some("#mygame-canvas".into()),
                ..default()
            }),
            ..default()                         
        }))
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,    
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),        
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });



    // 2D scene -------------------------------
    // Load card atlas
    let texture = asset_server.load("cardfish_cards.png");
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(180.0, 256.0), 11, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);


    


    commands.spawn(Camera2dBundle { 
        camera: Camera {
            order: 2, // Draw sprites on top of 3d world
            ..default()
        },
        ..default()
    });

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },            
            ..default()
        },        
    ));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("bevy_bird_dark.png"),
        ..default()
    });
}