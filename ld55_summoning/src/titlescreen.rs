use bevy::prelude::*;
use crate::summongame::{GameAppState, DontDeleteOnAppStateChange};


// Rest of the code

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        println!("In TitleScreenPlugin build...");
        //app.add_state(GameAppState::TitleScreen);
        app
            .add_systems( OnEnter(GameAppState::TitleScreen), title_setup )
            .add_systems(Update, title_update.run_if(in_state(GameAppState::TitleScreen)))
            .add_systems( OnExit(GameAppState::TitleScreen), title_teardown )
            ;
    }
}

fn title_setup( mut commands: Commands ) {
    println!("Title screen setup!");

    commands.spawn((
        TextBundle::from_section(
            "Title Screen Goes Here",
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
    ));

    commands.spawn((
        TextBundle::from_section(
            "Dont Delete Me",
            TextStyle {
                color: Color::GREEN,
                font_size: 20.,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Px(12.0),
            ..default()
        }), DontDeleteOnAppStateChange
    ));

}

fn title_update (
    // mut world : &mut World,
    //mut commands: Commands,
    mut game_state: ResMut<NextState<GameAppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
)
{
    println!("Titles update...");
    if keyboard_input.just_pressed( KeyCode::KeyW ) {
        println!("W pressed, start game");
        game_state.set(GameAppState::Gameplay);
    }
}

fn title_teardown(
    mut commands: Commands,
    despawn_q: Query<Entity, Without<DontDeleteOnAppStateChange>>) {
    println!("Title screen teardown!");

    // for entity in &despawn_q {
    //     commands.entity(entity).despawn_recursive();
    // }
}
