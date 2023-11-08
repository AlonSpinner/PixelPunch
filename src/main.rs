use bevy::{prelude::*,
     asset::LoadState,
     diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},};
use bevy_tile_atlas::TileAtlasBuilder;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use std::path::PathBuf;

pub mod fighters;
use fighters::*;
pub mod controls;
use controls::*;

//scene

const CEILING_Y : f32 = 300.0;
const FLOOR_Y : f32 = -300.0;
const LEFT_WALL_X : f32 = -450.0;
const RIGHT_WALL_X : f32 = 450.0;

//controls and visuals
const ANIMATION_TIME : f32 = 0.1;
const DOUBLE_TAP_DURATION : f32 = 0.2; //seconds

const FIGHTERS : [Fighter;2]= [Fighter::IDF, Fighter::HAMAS];

fn main() {
    App::new()
    .add_plugins((DefaultPlugins.set(ImagePlugin::default_nearest()),
                    // FrameTimeDiagnosticsPlugin,
                    // LogDiagnosticsPlugin::default(),
                ))
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
enum Player{
    Player1,
    Player2,
}

#[derive(Bundle)]
struct PlayerBundle{
    player: Player,
    fighter: Fighter,
    health: FighterHealth,
    position: FighterPosition,
    velocity: FighterVelocity,
    movement: FighterMovement,
    movement_duration: FighterMovementDuration,
    sprite: SpriteSheetBundle,
    controls: PlayerControls,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self{
            player: Player::Player1,
            fighter: Fighter::IDF,
            health : FighterHealth(100.0),
            position : FighterPosition{x : 0.0, y :0.0},
            velocity : FighterVelocity{x : 0.0, y :0.0},
            movement : FighterMovement::Jumping{inital_velocity : -JUMPING_SPEED, gravity : GRAVITY},
            movement_duration : FighterMovementDuration(0),
            sprite : SpriteSheetBundle::default(),
            controls : PlayerControls::default(),
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Resource)]
struct AssetLoading {
    fighters_movement_sprites: HashMap<Fighter, HashMap<String, Vec<Handle<Image>>>>,
    background_sprites: Vec<Handle<Image>>,
}

struct FighterAnimationHash {
    hashmap: HashMap<String, [usize;2]>,
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
    for fighter in FIGHTERS {
        let fighter_movement_graph = FIGHTERS_MOVEMENT_GRAPH.get(&fighter).unwrap();
        let mut fighter_movement_sprites: HashMap<String,Vec<Handle<Image>>> = HashMap::new();
        for movement_name in fighter_movement_graph.movements() {
            let mut sprites_vec: Vec<Handle<Image>> = Vec::new();
            let path = PathBuf::from("textures").join(fighter.to_string()).join(&movement_name);
            let untyped_handles = asset_server.load_folder(path).unwrap();
            for handle in untyped_handles.iter() {
                let image_handle = handle.clone().typed();
                sprites_vec.push(image_handle);
            }
        fighter_movement_sprites.insert(movement_name.clone(), sprites_vec);
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
        let mut movement_indicies: HashMap<String, [usize;2]> = HashMap::new();
        let mut index : usize = 0;
        for (movement_name, sprites_handles) in movement_sprites_handles.iter() {
            for sprite_handle in sprites_handles.iter() {
                atlas_builder.add_texture(sprite_handle.clone(), textures.get(&sprite_handle).unwrap()).unwrap();
            }
            movement_indicies.insert(movement_name.clone(), [index, index + sprites_handles.len()-1]);
            index += sprites_handles.len();
        }
        let texture_atlas_handle = texture_atlases.add(atlas_builder.finish(&mut textures).unwrap());
        let fighter_animation_hash = FighterAnimationHash{hashmap : movement_indicies, atlas_handle : texture_atlas_handle};
        fighters_movement_animation_indicies.0.insert(*fighter, fighter_animation_hash);
    }

    //player1
    let player = Player::Player1;
    let fighter = FIGHTERS[0];
    let sprite_sheet_bundle = SpriteSheetBundle {
        texture_atlas: fighters_movement_animation_indicies.0.get(&fighter).unwrap().atlas_handle.clone(),
        sprite: TextureAtlasSprite::default(),
        ..default()};
    commands.spawn(PlayerBundle{sprite : sprite_sheet_bundle.clone(),
                                        player : player,
                                        fighter : fighter,
                                        position : FighterPosition{x : LEFT_WALL_X + 200.0, y :0.0},
                                        velocity : FighterVelocity{x : 0.0, y :-JUMPING_SPEED},
                                        ..default()});

    // player2
    let player = Player::Player2;
    let fighter = Fighter::HAMAS;
    let player2_controls = PlayerControls{
        up: KeyCode::Up,
        down: KeyCode::Down,
        left: KeyCode::Left,
        right: KeyCode::Right,
        defend: KeyCode::ShiftRight,
        attack: KeyCode::Return,
    };

    let sprite_sheet_bundle = SpriteSheetBundle {
        texture_atlas: fighters_movement_animation_indicies.0.get(&fighter).unwrap().atlas_handle.clone(),
        sprite: TextureAtlasSprite{flip_x : true, ..default()},
        ..default()};
    commands.spawn(PlayerBundle{sprite : sprite_sheet_bundle,
                                        player : player,
                                        fighter : fighter,
                                        position : FighterPosition{x : RIGHT_WALL_X - 200.0, y :0.0},
                                        velocity : FighterVelocity{x : 0.0, y :-JUMPING_SPEED},
                                        controls: player2_controls,
                                        ..default()});
    
    //insert resources
    commands.insert_resource(AnimationTimer(Timer::from_seconds(ANIMATION_TIME, TimerMode::Repeating)));
    commands.insert_resource(fighters_movement_animation_indicies);
}

fn player_control(mut query: Query<(&Fighter,
                                    &mut PlayerControls,
                                    &mut FighterMovement,
                                    &mut FighterMovementDuration,
                                    &mut FighterPosition,
                                    &mut FighterVelocity)>,
                                    keyboard_input_resource: Res<Input<KeyCode>>,
                                    time : Res<Time>) {
    
    
    let time_elapsed = time.elapsed_seconds();
    let time_delta = time.delta_seconds();

    let keyboard_input = keyboard_input_resource.into_inner();

    for (fighter,
        mut player_controls,
        mut movement,
        mut movement_duration,
        mut position,
        mut velocity) in query.iter_mut() {

        let previous_movement = movement.clone();
        let keyset = player_controls.into_keytargetset(keyboard_input);
        let fighter_graph = FIGHTERS_MOVEMENT_GRAPH.get(&fighter).unwrap();

        if let Some(new_movement_node) = fighter_graph.nodes.get(&keyset) {    
            //check if we are evolving movement
            if new_movement_node.movement == previous_movement { 
                if let Some(evovled_movment_node) = new_movement_node.evovled_node.as_ref() {
                    if evovled_movment_node.player_enter_condition(FLOOR_Y, position.y, &previous_movement) {
                        info!("evoled movement");
                        movement.change_to(evovled_movment_node.movement.clone());
                        movement.enter_position_velocity(&mut position, &mut velocity);
                    }
                }
            //look for new movement
            } else if new_movement_node.player_enter_condition(FLOOR_Y, position.y, &previous_movement) {
                info!("found new movement from hash");
                movement.change_to(new_movement_node.movement.clone());
                movement.enter_position_velocity(&mut position, &mut velocity);
            }

        //havent found movement in hashmap, maybe its in subset of keys
        } else {
            for (movement_key_set, new_movement_node) in fighter_graph.nodes.iter() {
                if new_movement_node.movement == FighterMovement::Idle {continue;}
                if movement_key_set.is_subset(&keyset) {
                    if new_movement_node.movement.to_string() != movement.to_string() &&
                    new_movement_node.player_enter_condition(FLOOR_Y, position.y, &previous_movement) {
                        info!("found new movement from subset");
                        movement.change_to(new_movement_node.movement.clone());
                        movement.enter_position_velocity(&mut position, &mut velocity);
                        break;
                    }
                }
            }
        }
        if movement.is_changed() {
            movement_duration.0 = 0;
        } else {
            movement_duration.0 += 1; //increment movement duration by 1 frame
        }
    }
}
fn update_state(mut query: Query<(&mut FighterPosition,
                                      &mut FighterVelocity,
                                      &mut FighterMovement)>,
                                      time: Res<Time>,) {
    let dt = time.delta_seconds();
    
    for (mut position,
         mut velocity,
        movement) in query.iter_mut() {
        
        movement.update_position_velocity(&mut position, &mut velocity, dt);
        position.x = position.x.clamp(LEFT_WALL_X,RIGHT_WALL_X);
        position.y = position.y.clamp(FLOOR_Y, CEILING_Y);
    }
}

fn draw_fighters(time: Res<Time>,
                fighters_movement_animation_indicies: Res<FightersMovementAnimationIndicies>,
                mut animation_timer: ResMut<AnimationTimer>,
                mut query: Query<(&Fighter,
                               &FighterPosition,
                               &FighterVelocity,
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
                                                                                .hashmap.get(&movement.to_string()).unwrap();
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
