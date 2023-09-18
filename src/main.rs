//! Displays a single [`Sprite`], created from an image.

use bevy::prelude::*;
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

fn main() {
    App::new()
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
            HelloPlugin,
        ))
        .run();
}
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, setup);
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("cards/card_back.png"),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("cards/card_clubs_02.png"),
        transform: Transform::from_xyz(100.0, 0.0, 0.0),
        ..default()
    });
}
