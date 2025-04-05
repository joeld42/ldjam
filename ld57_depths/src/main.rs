use bevy::{asset::AssetMetaCheck, prelude::*, scene::SceneInstanceReady};
use bevy_skein::SkeinPlugin;

fn main() {
    App::new()
        .register_type::<Character>()
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
            // log the component from the gltf spawn
            |trigger: Trigger<SceneInstanceReady>,
             children: Query<&Children>,
             characters: Query<&Character>| {
                for entity in children
                    .iter_descendants(trigger.target())
                {
                    let Ok(character) =
                        characters.get(entity)
                    else {
                        continue;
                    };
                    info!(?character);
                }
            },
        )
        .add_systems(Startup, startup)
        .run();
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct Character {
    name: String,
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(SceneRoot(asset_server.load(
        // Change this to your exported gltf file
        GltfAssetLabel::Scene(0).from_asset("simpleworld.glb"),
    )));
}
