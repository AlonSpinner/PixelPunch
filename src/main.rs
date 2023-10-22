use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,) {
    
    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        sprite: Sprite {
            // custom_size: Some(Vec2::splat(160.0)),
            ..default()
        },
        ..default()
    });
    }

fn update() {}