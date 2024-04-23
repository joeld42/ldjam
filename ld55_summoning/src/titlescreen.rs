use bevy::prelude::*;
use crate::summongame::{ GameAppState, PlayerType, GoodStuff };

#[derive(Component)]
pub struct TitleScreenCleanup;

#[derive(Component)]
struct PlayerSetting(i32);


#[derive(Event)]
struct PlayerSettingsChanged;


pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        println!("In TitleScreenPlugin build...");
        //app.add_state(GameAppState::TitleScreen);
        app
            .add_systems( OnEnter(GameAppState::TitleScreen), title_setup )
            .add_systems(Update, (
                title_update,
                player_settings)
                .run_if(in_state(GameAppState::TitleScreen)))
            .add_systems( OnExit(GameAppState::TitleScreen), title_teardown )
            .add_event::<PlayerSettingsChanged>();
    }
}

fn title_setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut stuff: ResMut<GoodStuff>,
    mut ev_settings: EventWriter<PlayerSettingsChanged>,
) {
    println!("Title screen setup!");

    commands.spawn((SpriteBundle {
        texture: asset_server.load("title.png"),
        transform: Transform::from_xyz( 0.0, 0.0, 3.0 ).with_scale( Vec3::splat( 0.6)),
        ..default()
    }, TitleScreenCleanup ));


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
        TitleScreenCleanup,
    ));

    commands.spawn(
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
        })
    );

    // setup player status
    stuff.player_stuff[0].ptype = PlayerType::Local;
    stuff.player_stuff[1].ptype = PlayerType::AI;
    stuff.player_stuff[2].ptype = PlayerType::AI;
    stuff.player_stuff[3].ptype = PlayerType::NotActive;

    let mut yy = 350.0;
    for i in 0..4 {

            commands.spawn((
                TextBundle::from_section("Player # -- ???",
                    TextStyle {
                        color: stuff.player_stuff[i].color,
                        font_size: 30.,
                        ..default()
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(yy),
                    left: Val::Px( 300.0),
                    ..default()
                }),
                PlayerSetting(i as i32),
                TitleScreenCleanup) );
            yy += 30.0;
    }

    ev_settings.send( PlayerSettingsChanged );

}

fn title_update (
    // mut world : &mut World,
    //mut commands: Commands,
    mut stuff: ResMut<GoodStuff>,
    mut game_state: ResMut<NextState<GameAppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ev_settings: EventWriter<PlayerSettingsChanged>,
)
{
    println!("Titles update...");

    let mut z : i32 = -1;
    for i in 0..4 {
        let keycode = match i {
            0 => KeyCode::Digit1,
            1 => KeyCode::Digit2,
            2 => KeyCode::Digit3,
            3 => KeyCode::Digit4,
            _ => KeyCode::KeyQ,
        };
        if keyboard_input.just_pressed(  keycode ) {
            z = i;
            break;
        }
    }

    if z >= 0 {
        let z = z as usize;
        if stuff.player_stuff[z].ptype == PlayerType::Local {
            stuff.player_stuff[z].ptype = PlayerType::AI;
        } else if stuff.player_stuff[z].ptype == PlayerType::AI {
            stuff.player_stuff[z].ptype = PlayerType::NotActive;
        } else if stuff.player_stuff[z].ptype == PlayerType::NotActive {
            stuff.player_stuff[z].ptype = PlayerType::Local;
        }

        ev_settings.send( PlayerSettingsChanged );
    }

    let mut pcount : i32 = 0;
    for i in 0..4 {
        if stuff.player_stuff[i].ptype != PlayerType::NotActive {
            pcount += 1;
        }
    }

    // Start game?
    if pcount > 0 {
        if keyboard_input.just_pressed( KeyCode::Enter ) ||
            keyboard_input.just_pressed( KeyCode::Space )
        {
            game_state.set(GameAppState::Gameplay);
        }
    }
}

fn player_settings(
    stuff: Res<GoodStuff>,
    mut setting_q: Query<(&mut Text, &PlayerSetting)>,
    mut ev_settings: EventReader<PlayerSettingsChanged>,
) {
    for _ev in ev_settings.read() {

        for (mut text, plr) in &mut setting_q {

            let plr_type = match stuff.player_stuff[plr.0 as usize].ptype {
                PlayerType::Local => "Human",
                PlayerType::AI => "AI",
                PlayerType::NotActive => "None",
            };

            text.sections[0].value = format!("Player {} -- {}", plr.0 + 1, plr_type);
        }

    }

}



fn title_teardown(
    mut commands: Commands,
    despawn_q: Query<Entity, With<TitleScreenCleanup>>) {
    println!("Title screen teardown!");

    for entity in &despawn_q {
        commands.entity(entity).despawn_recursive();
    }
}
