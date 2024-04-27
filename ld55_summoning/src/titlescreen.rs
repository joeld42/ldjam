use bevy::{prelude::*, scene::ron::de};
use crate::summongame::{ GameAppState, PlayerType, GoodStuff };

#[derive(Component)]
pub struct TitleScreenCleanup;

#[derive(Component)]
struct PlayerSetting(i32);


#[derive(Event)]
struct PlayerSettingsChanged;

// Actions from the Player Settings buttons
#[derive(Component)]
enum PlayerSettingsButtonAction {
    ChangeProfile(i32),    
    ChangeMode(i32),
}


// Resource  stuff
#[derive(Resource,Default)]
pub struct TitleScreenStuff {
    pub pics_human: Vec<Handle<Image>>,
    pub pics_bot: Vec<Handle<Image>>,
}

use rand::Rng;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        println!("In TitleScreenPlugin build...");
        //app.add_state(GameAppState::TitleScreen);
        app
            .insert_resource( TitleScreenStuff::default() )
            .add_systems( OnEnter(GameAppState::TitleScreen), title_setup )
            .add_systems(Update, (
                title_update,
                
                player_settings,
                player_settings_action

                )
                .run_if(in_state(GameAppState::TitleScreen)))
            .add_systems( OnExit(GameAppState::TitleScreen), title_teardown )
            .add_event::<PlayerSettingsChanged>();
    }
}

fn title_setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut stuff: ResMut<GoodStuff>,
    mut title_stuff: ResMut<TitleScreenStuff>,
    mut ev_settings: EventWriter<PlayerSettingsChanged>,
) {
    println!("Title screen setup!");


    let title_img = asset_server.load("summoner_title.png");
    let playerframe_img = asset_server.load("ui_playerframe.png");
    let border_img = asset_server.load("panel-transparent-border-027.png");


    for i in 1..=5 
    {
        title_stuff.pics_human.push( asset_server.load(format!("portrait{}.png", i)));
    }

    for i in 1..=3 
    {
        title_stuff.pics_bot.push( asset_server.load(format!("portrait_bot{}.png", i)));
    }

    let slicer = TextureSlicer {
        border: BorderRect::square(22.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    let title_sz = 80.0;
    commands
        .spawn(NodeBundle {
            style: Style {
                left: Val::Percent((100.0 - title_sz) / 2.0),
                width: Val::Percent(title_sz),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            //background_color: BackgroundColor( Color::Rgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 0.2 } ),
            ..default()
        })
        .with_children(|parent| {


            // ---- Cyber Summoner Title Image -----------------------
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    margin: UiRect::top( Val::Percent( 2.0 )),
                    ..default()
                },
                image: UiImage::new(title_img),
                //background_color: BackgroundColor( Color::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.01 } ),
                ..default()
            });

            // ---- Player Select Tiles -----------------------
            let mut rng = rand::thread_rng();
            let tile_scale = 0.9;
            parent.spawn( NodeBundle {
                style: Style {
                    flex_wrap: FlexWrap::Wrap,
                    //align_items: AlignItems::SpaceEvenly,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                //background_color: BackgroundColor( Color::Rgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 0.2 } ),
                ..default()
            }).with_children( |tileparent| {

                for i in 0..4 {

                    tileparent.spawn(

                        ImageBundle {
                            style: Style {
                                width: Val::Px(156.0 * tile_scale),
                                height: Val::Px(234.0 * tile_scale),
                                margin: UiRect::all( Val::Px( 8.0 )),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            image: UiImage::new(playerframe_img.clone() ),
                            background_color: BackgroundColor( stuff.player_stuff[i].color, ),
                            ..default()
                        })
                        .with_children(|parent| {

                            //==== Player#1 header
                            parent.spawn(TextBundle::from_section(
                                format!("Player {}", (i+1) ).clone(),
                                TextStyle {
                                    //font: asset_server.load("Cyberthrone.ttf"),
                                    font_size: 14.0,
                                    color: Color::rgb(1.0, 1.0, 1.0),
                                    ..default()
                                },
                            ).with_style( Style {
                                width: Val::Px(130.0 * tile_scale),
                                margin: UiRect::new( Val::Px( 12.0 ), Val::Px( 8.0 ), Val::Px( 15.0 ), Val::Px( 6.0 )),
                                ..default()
                            }));

                            // Portrait
                            let pic = title_stuff.pics_human[ rng.gen_range( 0..title_stuff.pics_human.len() ) ].clone();
                            parent.spawn( ImageBundle {
                                style: Style {
                                    width: Val::Px( 100.0 * tile_scale ),
                                    height: Val::Px( 100.0 * tile_scale  ),
                                    ..default()
                                },
                                image: UiImage::new( pic ),
                                ..default()
                            });

                            //==== PlayerName
                            parent.spawn(TextBundle::from_section(
                                "Name",
                                TextStyle {
                                    //font: asset_server.load("Cyberthrone.ttf"),
                                    font_size: 20.0,
                                    color: stuff.player_stuff[i].color,
                                    ..default()
                                },
                            ).with_style( Style {
                                width: Val::Px(130.0 * tile_scale),
                                margin: UiRect::new( Val::Px( 40.0 ), Val::Px( 20.0 ),
                                                        Val::Px( 6.0 ), Val::Px( 15.0 )),
                                ..default()
                            }));

                            // Human/AI/None Selection Bar
                            parent.spawn( NodeBundle {
                                style: Style {
                                    width: Val::Percent(95.0),
                                    //flex_wrap: FlexWrap::Wrap,
                                    //align_items: AlignItems::SpaceEvenly,
                                    justify_content: JustifyContent::SpaceEvenly,
                                    ..default()
                                },
                                background_color: BackgroundColor( stuff.player_stuff[i].color ),
                                ..default()
                            })
                            .with_children( |btnparent| {

                                //for btn_ndx in 0..3 {
                                let btn_names = ["Human", "AI", "None" ];
                                for btn_ndx in 0..btn_names.len() {
                                    let btn_name = btn_names[ btn_ndx];

                                    
                                    btnparent.spawn(
                                        (ButtonBundle {
                                            style: Style {
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                height: Val::Px(20.0),
                                                ..default()
                                            },
                                            //background_color: BackgroundColor( btncolor  ),
                                            ..default()
                                        },
                                        PlayerSetting(i as i32),
                                        PlayerSettingsButtonAction::ChangeMode( btn_ndx as i32),
                                     ))
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            btn_name,
                                            TextStyle {
                                                //font: asset_server.load("Cyberthrone.ttf"),
                                                font_size: 20.0,
                                                //color: txtcolor,
                                                ..default()
                                            },
                                        ));
                                    });
                                }

                            });

                        });
                }

            });


            // ---- Start Game Button -----------------------
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(50.0),
                            height: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            //margin: UiRect::all(Val::Px(20.0)),
                            margin: UiRect::new( Val::Px(20.0), Val::Px(20.0),Val::Px(8.0),Val::Px(50.0) ),
                            ..default()
                        },
                        image: border_img.clone().into(),
                        ..default()
                    },
                    ImageScaleMode::Sliced(slicer.clone()),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start Game",
                        TextStyle {
                            font: asset_server.load("Cyberthrone.ttf"),
                            font_size: 50.0,
                            color: Color::rgb(1.0, 0.3, 0.9),
                        },
                    ));
                });

        });





    // setup player status
    stuff.player_stuff[0].ptype = PlayerType::Local;
    stuff.player_stuff[1].ptype = PlayerType::AI;
    stuff.player_stuff[2].ptype = PlayerType::AI;
    stuff.player_stuff[3].ptype = PlayerType::NotActive;


    // let mut yy = 350.0;
    // for i in 0..4 {

    //         commands.spawn((
    //             TextBundle::from_section("Player # -- ???",
    //                 TextStyle {
    //                     color: stuff.player_stuff[i].color,
    //                     font_size: 30.,
    //                     ..default()
    //                 },
    //             )
    //             .with_style(Style {
    //                 position_type: PositionType::Absolute,
    //                 top: Val::Px(yy),
    //                 left: Val::Px( 300.0),
    //                 ..default()
    //             }),

    //             PlayerSetting(i as i32),
    //             TitleScreenCleanup) );
    //         yy += 30.0;
    // }

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
 //   println!("Titles update...");

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
    mut setting_q: Query<(&Children, &mut BackgroundColor, &PlayerSetting, &PlayerSettingsButtonAction)>,
    mut text_query: Query<&mut Text>,
    mut ev_settings: EventReader<PlayerSettingsChanged>,
) {
    
    for _ev in ev_settings.read() {

        println!("Got player settings event ev" );

        for (children, mut bg, plr, plr_action) in &mut setting_q {
            let pndx = plr.0 as usize;

            if let PlayerSettingsButtonAction::ChangeMode(mode) = plr_action {
                
                let mtype = match mode {
                    0 => PlayerType::Local,
                    1 => PlayerType::AI,
                    _ => PlayerType::NotActive,
                };

                let (btncolor, txtcolor) = if stuff.player_stuff[pndx].ptype == mtype {
                    (Color::rgba(0.0, 0.0, 0.0, 0.5), Color::WHITE)
                } else {
                    (Color::rgba(0.0, 0.0, 0.0, 0.0), stuff.player_stuff[pndx].color2)
                };

                let mut text = text_query.get_mut(children[0]).unwrap();

                println!("Setting PLR {} to {:?}", pndx, mtype );
                bg.0 = btncolor;
                text.sections[0].style.color = txtcolor;
            }            
        }
    }

}

fn player_settings_action(
    interaction_query: Query<
        (&Interaction, &PlayerSettingsButtonAction, &PlayerSetting),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_settings: EventWriter<PlayerSettingsChanged>,
    mut stuff: ResMut<GoodStuff>,
) {
    for (interaction, menu_button_action, player) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                PlayerSettingsButtonAction::ChangeProfile(inc) => {
                    println!("Change Profile PLR {} inc {}", player.0, *inc );
                }
                
                PlayerSettingsButtonAction::ChangeMode(mode) => {
                    println!("Change mode PLR {} mode {}", player.0, mode );
                    stuff.player_stuff[player.0 as usize].ptype = match mode {
                        0 => PlayerType::Local,
                        1 => PlayerType::AI,
                        _ => PlayerType::NotActive,
                    };
                    ev_settings.send( PlayerSettingsChanged );
                }                
            }
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
