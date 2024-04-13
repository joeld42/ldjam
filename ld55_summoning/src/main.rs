//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;

// use bevy_prng::WyRand;
// use bevy_rand::prelude::GlobalEntropy;
// use rand_core::RngCore;

use rand::Rng;

#[derive(Resource,Default)]
struct CardDeck {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,

    // todo: card stats, etc 
}

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
        .insert_resource( CardDeck::default() )
        .add_systems(Startup, setup)
        .add_systems( Update, spawn_cards)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,    
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut cards: ResMut<CardDeck>,
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
    commands.spawn(Camera2dBundle { 
        camera: Camera {
            order: 2, // Draw sprites on top of 3d world
            ..default()
        },
        ..default()
    });

    // Load card atlas
    let texture = asset_server.load("cardfish_cards.png");
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new( 567.0*(256.0/811.0), 256.0), 11, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    cards.texture = texture;
    cards.layout = texture_atlas_layout;

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

    commands.spawn(SpriteBundle {
        texture: asset_server.load("bevy_bird_dark.png"),
        ..default()
    });
}

fn spawn_cards ( 
    mut commands: Commands,        
    cards: Res<CardDeck>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
)
{
    if keyboard_input.just_pressed( KeyCode::KeyW ) {
        println!("W pressed");
        let mut rng = rand::thread_rng();
        commands.spawn((
            SpriteSheetBundle {
                texture: cards.texture.clone(),
                atlas: TextureAtlas {
                    layout: cards.layout.clone(),
                    index: rng.gen_range(1..20),
                },     
                //Transform::from_xyz(rng.gen<f32>() * 100.0, 0.0f, rng.gen<f32>() * 100.0),       
                transform: Transform::from_xyz(rng.gen::<f32>() * 1000.0 - 500.0, rng.gen::<f32>() * 700.0 - 350.0, 0.0),
                ..default()
            },        
        ));
    }
}

