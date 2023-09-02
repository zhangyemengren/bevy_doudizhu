//! Displays a single [`Sprite`], created from an image.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, setup);
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