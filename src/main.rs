use bevy::{prelude::*, asset::LoadState};

const WALKING_SPEED : f32 = 100.0;
const RUNNING_SPEED : f32 = 100.0;
const GRAVITY : f32 = -100.0;
const JUMPING_SPEED : f32 = 100.0;
const CEILING_Y : f32 = 300.0;
const FLOOR_Y : f32 = -300.0;
const LEFT_WALL_X : f32 = -450.0;
const RIGHT_WALL_X : f32 = 450.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_state::<AppState>()
    .add_systems(OnEnter(AppState::Setup), load_textures)
    .add_systems(Update, check_textures_loaded.run_if(in_state(AppState::Setup)))
    
    .add_systems(OnEnter(AppState::InGame), setup_game)
    .add_systems(
        PreUpdate,
        player_control.run_if(in_state(AppState::InGame)),
    )
    .add_systems(
        Update,
        (update_motion,
                 draw_fighters).run_if(in_state(AppState::InGame)),
    )
    .add_systems(Update, bevy::window::close_on_esc)
    .run();
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Setup,
    InGame,
}

#[derive(Component)]
struct Health(f32);
#[derive(Component, Debug)]
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

#[derive(Component)]
enum AnimationType{
    Idle,
    Walking,
    Running,
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

#[derive(Resource)]
struct AnimationSpriteHandles{
    handles: Vec<Handle<Image>>,
    animation_type: AnimationType,}

fn load_textures(mut commands: Commands,
                 asset_server: Res<AssetServer>,) {
    let animations = vec!(
                 ("Idle",AnimationType::Idle),
                 ("Walking",AnimationType::Walking),
                 ("Running",AnimationType::Running),
                );

    for (animation_name, animation_type) in animations {
        let mut image_handles: Vec<Handle<Image>> = Vec::new();
        let texture_folder_path = format!("textures/{}", animation_name);
        let untyped_handles = asset_server.load_folder(texture_folder_path).unwrap();
        for handle in untyped_handles.iter() {
            let image_handle: Handle<Image> = handle.clone().typed();
            image_handles.push(image_handle);
        }
        commands.insert_resource(AnimationSpriteHandles{handles : image_handles,
                                               animation_type : animation_type});
    }
}

fn check_textures_loaded(
    mut next_state: ResMut<NextState<AppState>>,
    animation_sprite_handles: ResMut<AnimationSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    let mut all_loaded = true;

    for handle in &animation_sprite_handles.handles {
        let load_state = asset_server.get_load_state(handle);
        match load_state {
            LoadState::Loaded => continue,
            LoadState::NotLoaded | LoadState::Loading => {
                all_loaded = false;
                break;
            }
            LoadState::Failed => {
                panic!("Failed to load sprite");
            }
            _ => {
                panic!("Unexpected load state");
            }
        }
    }

    if all_loaded {
        next_state.set(AppState::InGame);
    }
}


// fn build_textures_atlas(mut commands: Commands,
//                  asset_server: Res<AssetServer>,
//                  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
//                  mut textures: ResMut<Assets<Image>>,
//                 sprite_folder: ResMut<SpriteHandles>) {
//     let mut atlas_builder = TextureAtlasBuilder::default();
//     for texture_handle in &sprite_folder.0 {
//         if let Some(tex) = textures.get(&texture_handle) {
//             atlas_builder.add_texture(texture_handle.clone(), tex)
//         }
            
//     }
//     let texture_atlas = atlas_builder.finish(&mut textures).unwrap();
//     let texture_atlas_handle = texture_atlases.add(texture_atlas);
// }

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

fn setup_game(
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
            ..default()},
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });

    //player1
    let player_texture = asset_server.load("textures/0.png");
    let texture_atlas = TextureAtlas::from_grid(player_texture, Vec2::new(900.0, 900.0), 1, 1,None, None); // 1x1 grid since we have only one sprite
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn(PlayerBundle{sprite : SpriteSheetBundle {
                                        texture_atlas: texture_atlas_handle,
                                        sprite: TextureAtlasSprite{index : 0,
                                                                   custom_size : Some(Vec2::new(100.0,100.0)),
                                                                  ..default()},
                                        ..default()},
                        ..default() });
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
                    } else {
                        *movement = Movement::Standing;
                        velocity.x = 0.0;
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

        if position.y <= FLOOR_Y {
            *movement = Movement::Standing;
            velocity.y = 0.0;
            position.y = FLOOR_Y;
        } else {
            assert!(*movement == Movement::InAir);
            velocity.y = velocity.y + GRAVITY * dt;
        }
    }
}

fn draw_fighters(mut query: Query<(&Position,
                               &Velocity,
                               &Movement,
                               &Stance,
                               &mut TextureAtlasSprite,
                               &mut Transform,)>) {
    for (position,
         velocity,
         movement,
         stance,
         mut sprite,
         mut transform) in query.iter_mut() {
        //choose correct sprite and draw at in the position
        transform.translation = Vec3::new(position.x, position.y, 0.0);
        if velocity.x > 0.0 {
            sprite.flip_x = false;
        } else if velocity.x < 0.0 {
            sprite.flip_x = true;
        }


    }
}
