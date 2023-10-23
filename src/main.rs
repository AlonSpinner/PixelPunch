use bevy::prelude::*;

const WALKING_SPEED : f32 = 5.0;
const RUNNING_SPEED : f32 = 10.0;
const GRAVITY : f32 = -1.0;
const JUMPING_SPEED : f32 = 5.0;
const CEILING_Y : f32 = 300.0;
const FLOOR_Y : f32 = -300.0;
const LEFT_WALL_X : f32 = -450.0;
const RIGHT_WALL_X : f32 = 450.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, (player_control,
                                                update_motion,
                                                draw_fighters)).run();
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
    InAir,
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
            movement : Movement::InAir,
            stance : Stance::Idle,
            sprite : SpriteSheetBundle::default(),
        }
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
    let player_texture = asset_server.load("textures/0.png");
    let texture_atlas =TextureAtlas::new_empty(player_texture, Vec2::new(100.0,100.0));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(PlayerBundle{sprite : SpriteSheetBundle {
                                                texture_atlas: texture_atlas_handle,
                                                ..default() }
                ..default()
                });
}

fn player_control(mut query: Query<(&Player,
                                    &mut Movement,
                                    &mut Velocity)>,
        keyboard_input: Res<Input<KeyCode>>) {
    for (player,
        mut movement,
        mut velocity) in query.iter_mut() {
        match player {
            Player::Player1 => {
                if *movement != Movement::InAir {
                    if keyboard_input.just_pressed(KeyCode::W) {
                        *movement = Movement::InAir;
                        velocity.y = JUMPING_SPEED;
                    } else if keyboard_input.pressed(KeyCode::S) {
                        *movement = Movement::Docking;
                        velocity.x = 0.0;
                    } else if keyboard_input.pressed(KeyCode::A) {
                        *movement = Movement::Walking;
                        velocity.x = -WALKING_SPEED;
                    } else if keyboard_input.pressed(KeyCode::D) {
                        *movement = Movement::Walking;
                        velocity.x = WALKING_SPEED;
                    }
                }
            }
            &Player::Player2 => {}
        }
}}

fn update_motion(mut query: Query<(&mut Position,
                                      &mut Velocity,
                                      &mut Movement)>,
                                      time: Res<Time>,) {
    let dt = time.delta_seconds();
    
    for (mut position,
         mut velocity,
         mut movement) in query.iter_mut() {
        
        position.x = (position.x + dt*velocity.x).clamp(LEFT_WALL_X,RIGHT_WALL_X);
        position.y = (position.y + dt*velocity.y).clamp(FLOOR_Y, CEILING_Y);

        if position.y == 0.0 {
            *movement = Movement::Standing;
            velocity.y = 0.0;
        } else {
            assert!(*movement == Movement::InAir);
            velocity.y = velocity.y - GRAVITY * dt;
        }
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
