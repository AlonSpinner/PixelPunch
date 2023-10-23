use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, (player1_action,
                                                propogate_motion,
                                                draw_fighters))
        .run();
}

#[derive(Component)]
struct Health(f32);
#[derive(Component)]
struct Position {
    x : f32,
    y : f32,
}
#[derive(Component)]
struct Velocity {
    x : f32,
    y : f32,
}
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
    velocity: Velocity,
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
            position : Position{x : 0.0, y :0.0},
            velocity : Velocity{x : 0.0, y :0.0},
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

fn player1_action(mut query: Query<(&Player, &mut Movement)>,
        keyboard_input: Res<Input<KeyCode>>) {
    for (player,mut movement) in query.iter_mut() {
        match player {
            Player::Player1 => {
                if *movement != Movement::Jumping {
                    if keyboard_input.just_pressed(KeyCode::W) {
                        *movement = Movement::Jumping;
                        info!("jumping");
                    } else if keyboard_input.pressed(KeyCode::S) {
                        *movement = Movement::Docking;
                        info!("docking");
                    } else if keyboard_input.pressed(KeyCode::A) {
                        *movement = Movement::Walking;
                        info!("walking left");
                    } else if keyboard_input.pressed(KeyCode::D) {
                        *movement = Movement::Walking;
                        info!("walking right");
                    }
                }
            }
            &Player::Player2 => {}
        }
}}

fn propogate_motion(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x = (position.x + velocity.x).clamp(0.0,1.0);
        position.y = (position.y + velocity.y).clamp(0.0, 1.0);
    }
}

fn draw_fighters(query: Query<(&Position,
                               &Movement,
                               &Stance,
                               &mut TextureAtlasSprite,
                               &mut Transform,)>) {
    for (position,
         movement,
         stance,
         sprite,
         transform) in query.iter() {
        //choose correct sprite and draw at in the position

        transform.with_translation(Vec3::new(position.x, position.y, 0.0));
    }
}
