use bevy::sprite;
use bevy::{prelude::*, asset::LoadState};
use bevy_tile_atlas::TileAtlasBuilder;
use strum_macros::{EnumString, Display};
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use std::path::PathBuf;

const WALKING_SPEED : f32 = 100.0;
const RUNNING_SPEED : f32 = 100.0;
const GRAVITY : f32 = -100.0;
const JUMPING_SPEED : f32 = 100.0;
const CEILING_Y : f32 = 300.0;
const FLOOR_Y : f32 = -300.0;
const LEFT_WALL_X : f32 = -450.0;
const RIGHT_WALL_X : f32 = 450.0;
const ANIMATION_TIME : f32 = 0.1;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_state::<AppState>()
    .add_systems(OnEnter(AppState::Setup), load_assets)
    .add_systems(Update, check_textures_loaded.run_if(in_state(AppState::Setup)))
    
    .add_systems(OnEnter(AppState::InGame), setup_game)
    .add_systems(
        PreUpdate,
        player_control.run_if(in_state(AppState::InGame)),
    )
    .add_systems(
        Update,
        (update_state,
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
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, EnumString, Display)]
enum FighterMovement {
    Idle,
    #[strum(serialize = "JumpLoop")]
    JumpLoop,
    #[strum(serialize = "Sliding")]
    Docking,
    Running,
    Walking,
}
impl FighterMovement {
    fn change_to(&mut self, new_movement: FighterMovement) {
        //only change if new movement is different to allow Bevy's change detection to work
        if &new_movement != self {
            *self = new_movement;
        }
    }
}

#[derive(Component)]
enum Stance{
    Defending,
    Attacking,
    Idle,
}
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Display)]
enum Fighter{
    IDF,
    HAMAS,
}
impl Fighter {
    fn movements(&self) -> Vec<FighterMovement> {
        match *self {
            Fighter::IDF => vec!(FighterMovement::Idle,
                                 FighterMovement::JumpLoop,
                                 FighterMovement::Docking,
                                 FighterMovement::Running,
                                 FighterMovement::Walking),
            Fighter::HAMAS => vec!(FighterMovement::Idle,
                                 FighterMovement::JumpLoop,
                                 FighterMovement::Docking,
                                 FighterMovement::Running,
                                 FighterMovement::Walking),
        }
    }
}

#[derive(Component)]
enum Player{
    Player1,
    Player2,
}

#[derive(Component)]
struct PlayerControls{
    up : KeyCode,
    down : KeyCode,
    left : KeyCode,
    right : KeyCode,
    attack : KeyCode,
    defend : KeyCode,
}

impl Default for PlayerControls {
    fn default() -> Self {
        Self{
            up : KeyCode::W,
            down : KeyCode::S,
            left : KeyCode::A,
            right : KeyCode::D,
            attack : KeyCode::G,
            defend : KeyCode::H,
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle{
    player: Player,
    fighter: Fighter,
    health: Health,
    position: Position,
    velocity: Velocity,
    movement: FighterMovement,
    stance: Stance,
    sprite: SpriteSheetBundle,
    controls: PlayerControls,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self{
            player: Player::Player1,
            fighter: Fighter::IDF,
            health : Health(100.0),
            position : Position{x : 0.0, y :0.0},
            velocity : Velocity{x : 0.0, y :0.0},
            movement : FighterMovement::JumpLoop,
            stance : Stance::Idle,
            sprite : SpriteSheetBundle::default(),
            controls : PlayerControls::default(),
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Resource)]
struct AssetLoading {
    fighters_movement_sprites: HashMap<Fighter, HashMap<FighterMovement, Vec<Handle<Image>>>>,
    background_sprites: Vec<Handle<Image>>,
}

struct FighterAnimationHash {
    hashmap: HashMap<FighterMovement, [usize;2]>,
    atlas_handle: Handle<TextureAtlas>,
}
#[derive(Resource)]
struct FightersMovementAnimationIndicies(HashMap<Fighter,FighterAnimationHash>);

fn load_assets(mut commands: Commands,
                 asset_server: Res<AssetServer>) {

    let mut assets = AssetLoading {
        fighters_movement_sprites: HashMap::new(),
        background_sprites: Vec::new(),
    };

    //load background sprites
    assets.background_sprites.push(asset_server.load("background.png"));

    //load fighter sprites
    let fighters = vec!(Fighter::IDF, Fighter::HAMAS);
    for fighter in fighters {
        let mut fighter_movement_sprites: HashMap<FighterMovement,Vec<Handle<Image>>> = HashMap::new();
        for movement in fighter.movements() {
            let mut sprites_vec: Vec<Handle<Image>> = Vec::new();
            let path = PathBuf::from("textures").join(fighter.to_string()).join(movement.to_string());
            let untyped_handles = asset_server.load_folder(path).unwrap();
            for handle in untyped_handles.iter() {
                let image_handle = handle.clone().typed();
                sprites_vec.push(image_handle);
            }
        fighter_movement_sprites.insert(movement, sprites_vec);
        }
        assets.fighters_movement_sprites.insert(fighter, fighter_movement_sprites);
    }
    commands.insert_resource(assets);
}

fn check_textures_loaded(
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    asset_loading: Res<AssetLoading>,
) { 
    for sprite_handles in asset_loading.background_sprites.iter() {
        let sprite_load_state = asset_server.get_load_state(sprite_handles);
        match sprite_load_state {
            LoadState::Loaded => {}
            LoadState::NotLoaded | LoadState::Loading => {return;}
            LoadState::Failed => {
                panic!("Failed to load sprite");
            }
            _ => {
                panic!("Unexpected load state");
            }
        }
    }
    
    for (_, movement_sprites_handles) in asset_loading.fighters_movement_sprites.iter() {
        for (_, sprites_handles) in movement_sprites_handles.iter() {
            for sprite_handle in sprites_handles.iter() {
                let sprite_load_state = asset_server.get_load_state(sprite_handle);
                match sprite_load_state {
                    LoadState::Loaded => {}
                    LoadState::NotLoaded | LoadState::Loading => {return;}
                    LoadState::Failed => {
                        panic!("Failed to load sprite");
                    }
                    _ => {
                        panic!("Unexpected load state");
                    }
                }
            }
        }
    }
    next_state.set(AppState::InGame);
    info!("all assets loaded")
}

fn setup_game(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
    asset_loading: Res<AssetLoading>,
    mut windows: Query<&mut Window>) {
    
    commands.spawn(Camera2dBundle::default());

    //background
    let mut window = windows.single_mut();
    window.title = "pixel punch".into();
    let background_handle = asset_loading.background_sprites[0].clone();  
    commands.spawn(SpriteBundle {
        texture: background_handle,
        sprite: Sprite {
            custom_size: Some(Vec2::new(window.width(), window.height())),
            ..default()},
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        ..default()
    });

    //build texture atlases for all fighters
    let mut fighters_movement_animation_indicies = FightersMovementAnimationIndicies(HashMap::new());
    for (fighter, movement_sprites_handles) in asset_loading.fighters_movement_sprites.iter() {
        let mut atlas_builder: TileAtlasBuilder = TileAtlasBuilder::default();
        let mut movement_indicies: HashMap<FighterMovement, [usize;2]> = HashMap::new();
        let mut index : usize = 0;
        for (movement, sprites_handles) in movement_sprites_handles.iter() {
            for sprite_handle in sprites_handles.iter() {
                atlas_builder.add_texture(sprite_handle.clone(), textures.get(&sprite_handle).unwrap()).unwrap();
            }
            movement_indicies.insert(movement.clone(), [index, index + sprites_handles.len()-1]);
            index += sprites_handles.len();
        }
        let texture_atlas_handle = texture_atlases.add(atlas_builder.finish(&mut textures).unwrap());
        let fighter_animation_hash = FighterAnimationHash{hashmap : movement_indicies, atlas_handle : texture_atlas_handle};
        fighters_movement_animation_indicies.0.insert(*fighter, fighter_animation_hash);
    }

    //player1
    let player = Player::Player1;
    let fighter = Fighter::IDF;
    let sprite_sheet_bundle = SpriteSheetBundle {
        texture_atlas: fighters_movement_animation_indicies.0.get(&fighter).unwrap().atlas_handle.clone(),
        sprite: TextureAtlasSprite::default(),
        ..default()};
    commands.spawn(PlayerBundle{sprite : sprite_sheet_bundle.clone(),
                                        player : player,
                                        fighter : fighter,
                                        position : Position{x : LEFT_WALL_X + 200.0, y :0.0},
                                        ..default()});

    //player2
    let player = Player::Player2;
    let fighter = Fighter::HAMAS;
    let player2_controls = PlayerControls{up : KeyCode::Up,
                                          down : KeyCode::Down,
                                          left : KeyCode::Left,
                                          right : KeyCode::Right,
                                          attack : KeyCode::J,
                                          defend : KeyCode::K};
    let sprite_sheet_bundle = SpriteSheetBundle {
        texture_atlas: fighters_movement_animation_indicies.0.get(&fighter).unwrap().atlas_handle.clone(),
        sprite: TextureAtlasSprite{flip_x : true, ..default()},
        ..default()};
    commands.spawn(PlayerBundle{sprite : sprite_sheet_bundle,
                                        player : player,
                                        fighter : fighter,
                                        position : Position{x : RIGHT_WALL_X - 200.0, y :0.0},
                                        controls: player2_controls,
                                        ..default()});
    
    //insert resources
    commands.insert_resource(AnimationTimer(Timer::from_seconds(ANIMATION_TIME, TimerMode::Repeating)));
    commands.insert_resource(fighters_movement_animation_indicies);
}

fn player_control(mut query: Query<(&PlayerControls,
                                    &mut FighterMovement,
                                    &mut Velocity)>,
        keyboard_input: Res<Input<KeyCode>>) {
    for (player_controls,
        mut movement,
        mut velocity) in query.iter_mut() {
        if *movement != FighterMovement::JumpLoop {
            if keyboard_input.just_pressed(player_controls.up) {
                movement.change_to(FighterMovement::JumpLoop);
                velocity.y = JUMPING_SPEED;
            } else if keyboard_input.pressed(player_controls.down) {
                movement.change_to(FighterMovement::Docking);
                velocity.x = 0.0;
            } else if keyboard_input.pressed(player_controls.left) {
                movement.change_to(FighterMovement::Walking);
                velocity.x = -WALKING_SPEED;
            } else if keyboard_input.pressed(player_controls.right) {
                movement.change_to(FighterMovement::Walking);
                velocity.x = WALKING_SPEED;
            } else {
                movement.change_to(FighterMovement::Idle);
                velocity.x = 0.0;
            }
        }
    }
}
fn update_state(mut query: Query<(&mut Position,
                                      &mut Velocity,
                                      &mut FighterMovement)>,
                                      time: Res<Time>,) {
    let dt = time.delta_seconds();
    
    for (mut position,
         mut velocity,
         mut movement) in query.iter_mut() {
        
        position.x = (position.x + dt*velocity.x).clamp(LEFT_WALL_X,RIGHT_WALL_X);
        position.y = (position.y + dt*velocity.y).clamp(FLOOR_Y, CEILING_Y);

        if position.y <= FLOOR_Y {
            movement.change_to(FighterMovement::Idle);
            velocity.y = 0.0;
            position.y = FLOOR_Y;
        } else {
            assert!(*movement == FighterMovement::JumpLoop);
            velocity.y = velocity.y + GRAVITY * dt;
        }
    }
}

fn draw_fighters(time: Res<Time>,
                fighters_movement_animation_indicies: Res<FightersMovementAnimationIndicies>,
                mut animation_timer: ResMut<AnimationTimer>,
                mut query: Query<(&Fighter,
                               &Position,
                               &Velocity,
                               Ref<FighterMovement>,
                               &mut TextureAtlasSprite,
                               &mut Transform,)>) {
    
    animation_timer.tick(Duration::from_secs_f32(time.delta_seconds()));
    for (fighter,
         position,
         velocity,
         movement,
         mut sprite,
         mut transform) in query.iter_mut() {
        
        if animation_timer.just_finished() {
            let movement_indicies = fighters_movement_animation_indicies.0.get(&fighter).unwrap()
                                                                                .hashmap.get(&movement).unwrap();
            if sprite.index < movement_indicies[0] || sprite.index > movement_indicies[1]-1 {
                sprite.index = movement_indicies[0];
            } else {
                sprite.index += 1;
            }
        }
        
        transform.translation = Vec3::new(position.x, position.y, 0.0);

        if velocity.x > 0.0 {
            sprite.flip_x = false;
        } else if velocity.x < 0.0 {
            sprite.flip_x = true;
        }


    }
}
