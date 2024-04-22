use bevy::prelude::*;
use crate::summongame::GameAppState;


// Rest of the code

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        println!("In TitleScreenPlugin build...");
        //app.add_state(GameAppState::TitleScreen);
        app.add_systems( OnEnter(GameAppState::TitleScreen), setup_title_screen );
    }
}

fn setup_title_screen( mut commands: Commands ) {
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

}

fn setup_title_teardown() {
    println!("Title screen teardown!");
}
