use bevy::{prelude::*, asset::LoadState};
use std::collections::HashMap;
use std::time::Duration;

const WALKING_SPEED : f32 = 100.0;
const RUNNING_SPEED : f32 = 100.0;
const GRAVITY : f32 = -100.0;
const JUMPING_SPEED : f32 = 100.0;
const CEILING_Y : f32 = 300.0;
const FLOOR_Y : f32 = -300.0;
const LEFT_WALL_X : f32 = -450.0;
const RIGHT_WALL_X : f32 = 450.0;
const SPRITE_WIDTH_HEIGHT : f32 = 100.0;
const ANIMATION_TIME : f32 = 0.1;

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
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Movement{
    Idle,
    JumpLoop,
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
struct ChangedMovement(bool);

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

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Resource)]
struct AnimationImageHandles{image_handles : HashMap<Movement, Vec<Handle<Image>>>,}

#[derive(Resource)]
struct AnimationIndicies{indicies : HashMap<Movement, [usize;2]>,}

fn load_textures(mut commands: Commands,
                 asset_server: Res<AssetServer>,) {

    let mut image_handle_hashmap: HashMap<Movement, Vec<Handle<Image>>> = HashMap::new();

    let animations = vec!(
                 ("Idle",Movement::Idle),
                //  ("Walking",Movement::Walking),
                //  ("Running",Movement::Running),
                 ("JumpLoop",Movement::JumpLoop),
                );

    for (animation_name, animation_type) in animations {
        let mut image_handles: Vec<Handle<Image>> = Vec::new();
        let texture_folder_path = format!("textures/{}", animation_name);
        let untyped_handles = asset_server.load_folder(texture_folder_path).unwrap();
        for handle in untyped_handles.iter() {
            let image_handle: Handle<Image> = handle.clone().typed();
            image_handles.push(image_handle);
        }
        image_handle_hashmap.insert(animation_type, image_handles);
    }

    commands.insert_resource(AnimationImageHandles{image_handles : image_handle_hashmap});
}

fn check_textures_loaded(
    mut next_state: ResMut<NextState<AppState>>,
    animation_hashmap: ResMut<AnimationImageHandles>,
    asset_server: Res<AssetServer>,
) {
    let mut all_loaded = true;

    for movement in animation_hashmap.image_handles.keys() {
        let animation_sprite_handles = animation_hashmap.image_handles.get(movement).unwrap();
        for handle in animation_sprite_handles {
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
    }

    if all_loaded {
        next_state.set(AppState::InGame);
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self{
            player: Player::Player1,
            fighter: Fighter::IDF,
            health : Health(100.0),
            position : Position{x : 0.0, y :0.0},
            velocity : Velocity{x : 0.0, y :0.0},
            movement : Movement::JumpLoop,
            stance : Stance::Idle,
            sprite : SpriteSheetBundle::default(),
        }
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    animation_hashmap: Res<AnimationImageHandles>,
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

    //fighters
    let mut sprite_count = 0;
    for animation_type in animation_hashmap.image_handles.keys() {
        let animation_sprite_handles = animation_hashmap.image_handles.get(animation_type).unwrap();
        sprite_count += animation_sprite_handles.len();
    }

    let mut atlas_builder = TextureAtlasBuilder::default()
                                                .max_size(Vec2::new(sprite_count as f32 * SPRITE_WIDTH_HEIGHT,
                                                                    sprite_count as f32 * SPRITE_WIDTH_HEIGHT));
    
    let mut animation_indicies_hashmap: HashMap<Movement, [usize;2]> = HashMap::new();
    let mut index = 0;
    for movement in animation_hashmap.image_handles.keys() {
        let animation_sprite_handles = animation_hashmap.image_handles.get(movement).unwrap();
        for handle in animation_sprite_handles {
            atlas_builder.add_texture(handle.clone(), textures.get(handle).unwrap());
        }
        animation_indicies_hashmap.insert(*movement, [index, animation_sprite_handles.len()]);
        index += animation_sprite_handles.len();
    }
    let texture_atlas = atlas_builder.finish(&mut textures).unwrap();
    let texture_atlas_handle = texture_atlases.add(texture_atlas.clone());
    commands.insert_resource(AnimationIndicies{indicies : animation_indicies_hashmap});
    assert!(index == sprite_count);

    let sprite_sheet_bundle = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::default(),
        ..default()};

    //spawn texture_atlas
    commands.spawn((SpriteBundle {
        texture: texture_atlas.texture.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..default()
    },));

    //player1
    commands.spawn((PlayerBundle{sprite : sprite_sheet_bundle, ..default()},
    AnimationTimer(Timer::from_seconds(ANIMATION_TIME, TimerMode::Repeating))
    ));
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
                if *movement != Movement::JumpLoop {
                    if keyboard_input.just_pressed(KeyCode::W) {
                        *movement = Movement::JumpLoop;
                        velocity.y = JUMPING_SPEED;
                    } else if keyboard_input.pressed(KeyCode::S) {
                        *movement = Movement::Docking;
                        velocity.x = 0.0;
                    } else if keyboard_input.pressed(KeyCode::A) {
                        if *movement!=Movement::Walking {*movement = Movement::Walking;}
                        velocity.x = -WALKING_SPEED;
                    } else if keyboard_input.pressed(KeyCode::D) {
                        if *movement!=Movement::Walking {*movement = Movement::Walking;}
                        velocity.x = WALKING_SPEED;
                    } else {
                        if *movement!=Movement::Idle {*movement = Movement::Idle;}
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
            if *movement != Movement::Idle {
                *movement = Movement::Idle;
                velocity.y = 0.0;
                position.y = FLOOR_Y;
            }
        } else {
            assert!(*movement == Movement::JumpLoop);
            velocity.y = velocity.y + GRAVITY * dt;
        }
    }
}

fn draw_fighters(time: Res<Time>,
                animation_indicies: Res<AnimationIndicies>,
                mut query: Query<(&Position,
                               &Velocity,
                               Ref<Movement>,
                               &mut AnimationTimer,
                               &mut TextureAtlasSprite,
                               &mut Transform,)>) {
    for (position,
         velocity,
         movement,
         mut animation_timer,
         mut sprite,
         mut transform) in query.iter_mut() {
        
        animation_timer.tick(Duration::from_secs_f32(time.delta_seconds()));
        if animation_timer.just_finished() {
            let movement_indicies = animation_indicies.indicies.get(&movement).unwrap();
            sprite.index = sprite.index.max(movement_indicies[0]);
            sprite.index = ((sprite.index - movement_indicies[0] + 1) % movement_indicies[1]) + movement_indicies[0];
            info!("sprite index is now {}", sprite.index);
        }
        
        transform.translation = Vec3::new(position.x, position.y, 0.0);

        if velocity.x > 0.0 {
            sprite.flip_x = false;
        } else if velocity.x < 0.0 {
            sprite.flip_x = true;
        }


    }
}
