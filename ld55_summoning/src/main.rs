//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{
    core_pipeline::{
        //bloom::{BloomCompositeMode, BloomSettings},
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    },
    prelude::*,    
    render::mesh::VertexAttributeValues,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},

};

use rand::Rng;

use std::f32::consts::PI;

use crate::gamestate::GameMap;
use crate::gamestate::MapSpaceContents;
pub mod gamestate;

const HEX_SZ : f32 = 3.0;

#[derive(Resource,Default)]
struct CardDeck {
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,

    // todo: card stats, etc 
}



#[derive(Resource,Default)]
struct GameState {
    map : GameMap,
    map_visuals: Vec<Entity>,
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
        .insert_resource( CardDeck::default() )
        .insert_resource( GameState::default() )
        .add_systems(Startup, setup)
        .add_systems(Startup, build_map )
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

    commands.spawn(PbrBundle {
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
        transform: Transform::from_xyz(-4.0, 8.0, 1.0),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 320.0,
            //shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 2.0, 0.0)
            .with_rotation(Quat::from_rotation_x( -PI / 4.)),
        ..default()
    });
    
    // camera
    commands.spawn( ( Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            tonemapping: Tonemapping::TonyMcMapface,         
            ..default()
            },
            BloomSettings::NATURAL,
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

    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("bevy_bird_dark.png"),
    //     ..default()
    // });
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
                transform: Transform::from_xyz(rng.gen::<f32>() * 1000.0 - 500.0, rng.gen::<f32>() * 700.0 - 350.0, 0.0),
                ..default()
            },        
        ));
    }
}

fn worldpos_from_mapindex( mapindex : i32 ) -> Vec3
{
    let row : i32 = mapindex / (gamestate::MAP_SZ as i32);
    let col : i32 = mapindex % (gamestate::MAP_SZ as i32);

    // Make a vec3 from row and col
    Vec3::new((col as f32 - 5.0) * HEX_SZ, 0.0, (row as f32 - 5.0) * HEX_SZ )    
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
    mut commands: Commands,
    mut gamestate: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,    
    mut materials: ResMut<Assets<StandardMaterial>>,
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

        if hex_pos.length() < 8.0 {
            //println!("Map includes hex {}, World Position: {:?} len {}", map_space.ndx, hex_pos, hex_pos.length());
            if rng.gen_ratio(1, 8) {
                map_space.contents = MapSpaceContents::Blocked;
            } else {
                map_space.contents = MapSpaceContents::Playable;
            }
        }
    }

    // Now build the map visuals based on the map data
    let mut map_visuals = Vec::new();
    for map_space in &gamestate.map {
        let hex_pos = worldpos_from_mapindex( map_space.ndx );
        let ent = match map_space.contents {
            MapSpaceContents::NotInMap => Entity::PLACEHOLDER,
            MapSpaceContents::Blocked => {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.5, 3.0, 1.5)),
                    material: materials.add(Color::rgb_u8(96, 60, 100)),        
                    transform: Transform::from_translation( hex_pos ),
                    ..default()
                }).id()
            },
            MapSpaceContents::Playable => {
                commands.spawn(PbrBundle {
                            mesh: meshes.add(Cuboid::new(2.0, 0.3, 2.0)),
                            material: materials.add(Color::rgb_u8(96, 255, 130)),        
                            transform: Transform::from_translation( hex_pos ),
                            ..default()
                        }).id()
            },
        };

        map_visuals.push( ent )
    }
    
    // Add give the new visuals to map
    gamestate.map_visuals = map_visuals;


    println!("Map size {}", gamestate.map_visuals.len());    


}
