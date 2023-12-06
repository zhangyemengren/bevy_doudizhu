use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

const BACK_CARD: &str = "cards/card_back.png";
const CLUBS_CARD: [&str; 13] = [
    "cards/card_clubs_A.png",
    "cards/card_clubs_02.png",
    "cards/card_clubs_03.png",
    "cards/card_clubs_04.png",
    "cards/card_clubs_05.png",
    "cards/card_clubs_06.png",
    "cards/card_clubs_07.png",
    "cards/card_clubs_08.png",
    "cards/card_clubs_09.png",
    "cards/card_clubs_10.png",
    "cards/card_clubs_J.png",
    "cards/card_clubs_Q.png",
    "cards/card_clubs_K.png",
];
const DIAMONDS_CARD: [&str; 13] = [
    "cards/card_diamonds_A.png",
    "cards/card_diamonds_02.png",
    "cards/card_diamonds_03.png",
    "cards/card_diamonds_04.png",
    "cards/card_diamonds_05.png",
    "cards/card_diamonds_06.png",
    "cards/card_diamonds_07.png",
    "cards/card_diamonds_08.png",
    "cards/card_diamonds_09.png",
    "cards/card_diamonds_10.png",
    "cards/card_diamonds_J.png",
    "cards/card_diamonds_Q.png",
    "cards/card_diamonds_K.png",
];
const HEARTS_CARD: [&str; 13] = [
    "cards/card_hearts_A.png",
    "cards/card_hearts_02.png",
    "cards/card_hearts_03.png",
    "cards/card_hearts_04.png",
    "cards/card_hearts_05.png",
    "cards/card_hearts_06.png",
    "cards/card_hearts_07.png",
    "cards/card_hearts_08.png",
    "cards/card_hearts_09.png",
    "cards/card_hearts_10.png",
    "cards/card_hearts_J.png",
    "cards/card_hearts_Q.png",
    "cards/card_hearts_K.png",
];
const SPADES_CARD: [&str; 13] = [
    "cards/card_spades_A.png",
    "cards/card_spades_02.png",
    "cards/card_spades_03.png",
    "cards/card_spades_04.png",
    "cards/card_spades_05.png",
    "cards/card_spades_06.png",
    "cards/card_spades_07.png",
    "cards/card_spades_08.png",
    "cards/card_spades_09.png",
    "cards/card_spades_10.png",
    "cards/card_spades_J.png",
    "cards/card_spades_Q.png",
    "cards/card_spades_K.png",
];
const JOKER_CARD: [&str; 2] = ["cards/card_joker_black.png", "cards/card_joker_red.png"];


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Resource)]
struct AllCards(Vec<&'static str>);

impl Default for AllCards {
    fn default() -> Self {
        let mut rng = thread_rng();

        let mut all_cards = [
            &CLUBS_CARD[..],
            &DIAMONDS_CARD[..],
            &HEARTS_CARD[..],
            &SPADES_CARD[..],
            &JOKER_CARD[..],
        ]
        .concat();

        all_cards.shuffle(&mut rng);

        Self(all_cards)
    }
}

fn main() {
    App::new()
        .insert_resource(AllCards::default())
        // 窗口插件及配置
        .add_plugins((
            // set 设置DefaultPlugins包含的插件配置
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "I am a window!".into(),
                    // 窗口大小
                    resolution: (1200., 800.).into(),
                    ..default()
                }),
                ..default()
            }),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, setup);
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut all_cards: ResMut<AllCards>) {
    commands.spawn(Camera2dBundle::default());

    let mut rng = thread_rng();


    all_cards.0.shuffle(&mut rng);

    let w = 64.0; //有白边
    let h = 64.0;
    let origin = (-600.0 + w * 0.5, 400.0 - h * 0.5);
    let mut x_offset = origin.0;
    let mut y_offset = origin.1;

    for card in all_cards.0.iter() {
        commands.spawn(SpriteBundle {
            texture: asset_server.load(*card),
            transform: Transform::from_xyz(x_offset, y_offset, 0.0),
            ..default()
        });
        x_offset += w;
        if x_offset > 600.0 - w {
            x_offset = origin.0;
            y_offset -= h;
        }
    }
    // 按钮
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: Default::default(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}


fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut all_cards: ResMut<AllCards>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                let mut rng = thread_rng();


                all_cards.0.shuffle(&mut rng);

                let w = 64.0; //有白边
                let h = 64.0;
                let origin = (-600.0 + w * 0.5, 400.0 - h * 0.5);
                let mut x_offset = origin.0;
                let mut y_offset = origin.1;

                for card in all_cards.0.iter() {
                    commands.spawn(SpriteBundle {
                        texture: asset_server.load(*card),
                        transform: Transform::from_xyz(x_offset, y_offset, 0.0),
                        ..default()
                    });
                    x_offset += w;
                    if x_offset > 600.0 - w {
                        x_offset = origin.0;
                        y_offset -= h;
                    }
                }
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

