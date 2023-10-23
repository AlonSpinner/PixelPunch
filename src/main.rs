use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, player1)
        .run();
}

#[derive(Component)]
struct Health(f64);
#[derive(Component)]
struct Position(f64,f64);
#[derive(Component, PartialEq)]
enum Movement{
    Standing,
    Jumping,
    Docking,
    Running,
    Walking,
}
#[derive(Component)]
enum Stance{
    Defending,
    Attacking,
    Idle,
}
#[derive(Component)]
enum Fighter{
    IDF,
    HAMAS,
}

#[derive(Component)]
enum Player{
    Player1,
    Player2,
}

#[derive(Bundle)]
struct PlayerBundle{
    player: Player,
    fighter: Fighter,
    health: Health,
    position: Position,
    movement: Movement,
    stance: Stance,
    sprite: SpriteSheetBundle,

}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self{
            player: Player::Player1,
            fighter: Fighter::IDF,
            health : Health(100.0),
            position : Position(0.0,0.0),
            movement : Movement::Standing,
            stance : Stance::Idle,
            sprite : SpriteSheetBundle::default(),
        }
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>) {
    
    commands.spawn(Camera2dBundle::default());

    //background
    let mut window = windows.single_mut();
    window.title = "pixel punch".into();
    let texture_handle = asset_server.load("background.png");    
    commands.spawn(SpriteBundle {
        texture: texture_handle,
        sprite: Sprite {
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..default()
        },
        ..default()
    });

    //player1
    commands.spawn(PlayerBundle::default());
}

fn player1(mut query: Query<(&Player, &Movement)>,
        keyboard_input: Res<Input<KeyCode>>) {
    for (player, movement) in query.iter() {
    match player {
        Player::Player1 => {
            if movement != &Movement::Jumping {
                if keyboard_input.just_pressed(KeyCode::W) {
                    
                    
                } else if keyboard_input.just_pressed(KeyCode::S) {
                
                } else if keyboard_input.just_pressed(KeyCode::A) {

                } else if keyboard_input.just_pressed(KeyCode::D) {

                }
            }
        }
        &Player::Player2 => {}
    }
}}