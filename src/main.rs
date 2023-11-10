use bevy::{prelude::*,
     asset::LoadState,
     diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, ecs::event,};
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
    movement_node_name : FighterMovementNodeName,
    movement_duration: FighterMovementDuration,
    event_keytargetset_stack : KeyTargetSetStack,
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
            movement_node_name : FighterMovementNodeName("InAir".to_string()),
            movement_duration : FighterMovementDuration(0.0),
            event_keytargetset_stack : KeyTargetSetStack::new(10, 0.5),
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
        for sprite_name in fighter_movement_graph.name_map.values().map(|x| x.sprite_name.clone()) {
            let mut sprites_vec: Vec<Handle<Image>> = Vec::new();
            let path = PathBuf::from("textures").join(fighter.to_string()).join(&sprite_name);
            let untyped_handles = asset_server.load_folder(path).unwrap();
            for handle in untyped_handles.iter() {
                let image_handle = handle.clone().typed();
                sprites_vec.push(image_handle);
            }
        fighter_movement_sprites.insert(sprite_name.clone(), sprites_vec);
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
                                    &PlayerControls,
                                    &mut KeyTargetSetStack,
                                    &mut FighterMovementDuration,
                                    &mut FighterMovementNodeName,
                                    &mut FighterPosition,
                                    &mut FighterVelocity)>,
                                    keyboard_input_resource: Res<Input<KeyCode>>,
                                    time: Res<Time>,
                                    ) {
    let keyboard_input = keyboard_input_resource.into_inner();

    for (fighter,
        player_controls,
        mut event_keytargetset_stack,
        mut movement_duration,
        mut movement_node_name,
        mut position,
        mut velocity) in query.iter_mut() {

        let persistent_keytargetset = player_controls.into_persistent_keytargetset(&keyboard_input);
        let event_keytargetset = player_controls.into_event_keytargetset(&keyboard_input);
        let fighter_map = FIGHTERS_MOVEMENT_GRAPH.get(&fighter).unwrap();
        event_keytargetset_stack.update(time.delta_seconds());
        
        if event_keytargetset != KeyTargetSet::empty() {event_keytargetset_stack.push(event_keytargetset);}
        let joined_event_keytargetset = event_keytargetset_stack.join();

        //if cant exist movement
        if !fighter_map.name_map.get(&movement_node_name.0).unwrap()
                .player_exit_condition(FLOOR_Y, position.y, movement_duration.0) {
            movement_duration.0 += time.delta_seconds();

        //check for events
        } else if let Some(movement_node) = fighter_map.keyset_map.get(&joined_event_keytargetset) {
            if movement_node.player_enter_condition(FLOOR_Y, position.y, &movement_node_name.0, &joined_event_keytargetset) {
                movement_node.enter(&mut position, &mut velocity);
                movement_duration.0 = 0.0;
                movement_node_name.0 = movement_node.name.clone();
            }
        //check for persistent movements
        } else if let Some(movement_node) = fighter_map.keyset_map.get(&persistent_keytargetset) {
            if movement_node.name != movement_node_name.0 && 
                    movement_node.player_enter_condition(FLOOR_Y, position.y, &movement_node_name.0, &persistent_keytargetset) {
                movement_node.enter(&mut position, &mut velocity);
                movement_duration.0 = 0.0;
                movement_node_name.0 = movement_node.name.clone();
            }
        } else if movement_node_name.0 != "Idle".to_string() {
            movement_duration.0 = 0.0;
            movement_node_name.0 = "Idle".to_string();
        }
    
        if movement_node_name.is_changed() {
            info!("fighter {} changed movement to {}", fighter.to_string(), movement_node_name.0);
        }
    }
}
fn update_state(mut query: Query<(&Fighter,
                                    &mut FighterPosition,
                                    &mut FighterVelocity,
                                    &FighterMovementNodeName,)>,
                                    time: Res<Time>,) {
    let dt = time.delta_seconds();
    
    for (fighter,
        mut position,
        mut velocity,
        movement_node_index) in query.iter_mut() {

        let fighter_map = FIGHTERS_MOVEMENT_GRAPH.get(&fighter).unwrap();
        let movement_node = fighter_map.name_map.get(&movement_node_index.0).unwrap();
        
        movement_node.update(&mut position, &mut velocity, dt);
        position.x = position.x.clamp(LEFT_WALL_X,RIGHT_WALL_X);
        position.y = position.y.clamp(FLOOR_Y, CEILING_Y);
    }
}

fn draw_fighters(time: Res<Time>,
                fighters_movement_animation_indicies: Res<FightersMovementAnimationIndicies>,
                mut animation_timer: ResMut<AnimationTimer>,
                mut query: Query<(&Fighter,
                                &FighterMovementNodeName,
                                &FighterPosition,
                                &FighterVelocity,
                                &mut TextureAtlasSprite,
                                &mut Transform,)>) {
    
    animation_timer.tick(Duration::from_secs_f32(time.delta_seconds()));
    for (fighter,
        movement_node_name,
        position,
        velocity,
        mut sprite,
        mut transform) in query.iter_mut() {

        let movement_node = FIGHTERS_MOVEMENT_GRAPH.get(&fighter).unwrap().name_map.get(&movement_node_name.0).unwrap();
        let sprite_name = &movement_node.sprite_name;
        
        if animation_timer.just_finished() {
            let movement_indicies = fighters_movement_animation_indicies.0.get(&fighter).unwrap()
                                                                                .hashmap.get(sprite_name).unwrap();
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
