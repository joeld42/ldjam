use bevy::prelude::*;
use crate::summongame::GameAppState;


// Rest of the code

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        println!("In TitleScreenPlugin build...");
        //app.add_state(GameAppState::TitleScreen);
        app.add_systems( Startup, setup_title_screen );
    }
}

fn setup_title_screen() {
    println!("Title screen loaded!");
}
