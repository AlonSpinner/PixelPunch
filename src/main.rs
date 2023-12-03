use bevy::{prelude::*,
     asset::LoadState,
     diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}
    };
use bevy_tile_atlas::TileAtlasBuilder;
use bevy_prototype_lyon::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use std::path::PathBuf;
use std::sync::Arc;
use serde_yaml;

pub mod components_bundles;
use components_bundles::*;
pub mod fighters_movement_map;
use fighters_movement_map::*;
pub mod utils;
use utils::*;

//scene
const CEILING_Z : f32 = -100.0;
const FLOOR_Z : f32 = -335.0;
const NORTH_WALL_Y : f32 = 80.0;
const SOUTH_WALL_Y : f32 = -80.0;
const EAST_WALL_X : f32 = 600.0;
const WEST_WALL_X : f32 = -600.0;

//controls and visuals
const ANIMATION_TIME : f32 = 0.05;
const FIGHTERS : [Fighter;2]= [Fighter::IDF, Fighter::HAMAS];

//assets
const YAML_DATA: &str = include_str!("../assets/assets.yaml");

fn main() {
    App::new()
    .insert_resource(Msaa::Sample4)
    .insert_resource(FighterMovementMapCollection::default())
    .add_plugins((EmbeddedAssetPlugin::default(),
                    DefaultPlugins.set(ImagePlugin::default_nearest()),
                    ShapePlugin,
                    FrameTimeDiagnosticsPlugin,
                    LogDiagnosticsPlugin::default(),
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
        (update_state).run_if(in_state(AppState::InGame)),
    )
    .add_systems(
        PostUpdate,
        (draw_fighters,
                update_healthbars,
                update_shadows,
                ).run_if(in_state(AppState::InGame)),
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
                 asset_server: Res<AssetServer>,
                 figher_movement_map_collection: Res<FighterMovementMapCollection>,) {


    let mut assets = AssetLoading {
        fighters_movement_sprites: HashMap::new(),
        background_sprites: Vec::new(),
    };

    let yaml : serde_yaml::Mapping = serde_yaml::from_str(YAML_DATA)
        .expect("Failed to deserialize asset paths from YAML");

    //load background sprites
    assets.background_sprites.push(asset_server.load("background.png"));

    //load fighter sprites
    for fighter in FIGHTERS {
        let fighter_movement_graph = figher_movement_map_collection.0.get(&fighter).unwrap();
        let mut fighter_movement_sprites: HashMap<String,Vec<Handle<Image>>> = HashMap::new();
        for sprite_name in fighter_movement_graph.movement_map.values().map(|x| x.sprite_name()) {
            let mut sprites_vec: Vec<Handle<Image>> = Vec::new();
            let dirpath = PathBuf::from("textures").join(fighter.to_string()).join(sprite_name);
            
            let dirvalues = yaml.get(dirpath.to_str()
                .expect("dirpath has to contain only UTF-8 validity"))
                .expect("yaml does not contain the key").as_sequence()
                .expect("yaml value is not a sequence");
            for entry in dirvalues{
                let filename = entry.as_str().unwrap();
                let fullfilename = format!("{}/{}", dirpath.to_str().unwrap(), filename);
                let image_handle = asset_server.load(fullfilename);
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
        transform: Transform::from_xyz(0.0, 0.0, -NORTH_WALL_Y),
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

    let mut spawn_fighter = |player : Player,
                                            player_controls : PlayerControls,
                                            fighter : Fighter,
                                            position : FighterPosition,
                                            health_bar_x : f32,
                                            health_bar_reverse : bool,
                                            facing_east : bool,| {

    let sprite_sheet_bundle = SpriteSheetBundle {
        texture_atlas: fighters_movement_animation_indicies.0.get(&fighter).unwrap().atlas_handle.clone(),
        sprite: TextureAtlasSprite::default(),
        ..default()};
    let mut movement_stack = FighterMovementStack::new(10);
        movement_stack.push(FighterMovement::InAir);
    let fighter_id = commands.spawn(ControlledFighterBundle{
                                        player : player,
                                        controls : player_controls,
                                        fighter_bundle : FighterBundle {
                                            fighter: fighter,
                                            hitbox: FighterHitBox::default(),
                                            hurtbox: FighterHurtBox::default(),
                                            health : FighterHealth{current : 100.0, max : 100.0},
                                            position : position,
                                            velocity : FighterVelocity{x : 0.0, y :0.0, z :0.0},
                                            facing_east : FacingEast(facing_east),
                                            movement_stack : movement_stack,
                                            event_keytargetset_stack : KeyTargetSetStack::new(10, 0.5),
                                            sprite : sprite_sheet_bundle,
                                    }
    }).id();

    // shadow
    commands.spawn(ShadowBundle::new(Vec2::new(20.0,10.0),
                                        -NORTH_WALL_Y,
                                        false,
                                        Color::rgba(0.0, 0.0, 0.0, 0.8),
                                        Color::rgba(0.0, 0.0, 0.0, 0.0),
                                        2.0,
                                        fighter_id,
                                        -32.0
                                    ));


    //health bar
    let (healthbar_green_bundle,healthbar_red_bundle) = StatBarBundle::new_with_emptycolor(Color::rgb(0.0, 1.0, 0.0),
                            Color::rgb(1.0, 0.0, 0.0),
                                        window.width()/3.0,
                                        window.height()/20.0,
                                        Vec2::new(
                                            health_bar_x,
                                            -window.height()/2.0 + window.height() * 0.95),
                                        health_bar_reverse,
                                        false,
                                        fighter_id,
                                        0.0);
    let healthbar_green_id = commands.spawn(healthbar_green_bundle).id();
    commands.spawn(healthbar_red_bundle).set_parent(healthbar_green_id);
    };

    spawn_fighter(Player::Player1,
        PlayerControls::default(),
        FIGHTERS[0],
        FighterPosition { x: -window.width() * 0.4, y: 0.0, z: 0.0 },
        -window.width()/2.0 + window.width()* 0.02,
        false,
        true
    );

    spawn_fighter(Player::Player2,
        PlayerControls{
            up: KeyCode::Up,
            down: KeyCode::Down,
            left: KeyCode::Left,
            right: KeyCode::Right,
            attack: KeyCode::Period,
            jump: KeyCode::Comma,
            defend: KeyCode::M
        },
        FIGHTERS[1],
        FighterPosition { x: window.width() * 0.4, y: 0.0, z: 0.0 },
        window.width()/2.0 - window.width()* 0.02,
        true,
        false
    );
    
    //insert resources
    commands.insert_resource(AnimationTimer(Timer::from_seconds(ANIMATION_TIME, TimerMode::Repeating)));
    commands.insert_resource(fighters_movement_animation_indicies);
}

//given a vec of request nodes, enter requested node if vec.len()==1
//returns true if entered, else false
fn enter_requested_node<T>(request_movement_nodes : Vec<&Arc<T>>,
    movement_stack : &mut FighterMovementStack,
    position : &mut FighterPosition,
    velocity: &mut FighterVelocity,
    facing_east: &mut FacingEast,) -> Option<FighterMovement>
    where T: FighterMovementNodeTrait {
    match request_movement_nodes.len() {
        0 => None,
        1 => {
            let new_movement_node: &&Arc<T> = &request_movement_nodes[0];
            movement_stack.push(new_movement_node.movement());
            new_movement_node.state_enter(position, velocity, facing_east);
            Some(new_movement_node.movement())
        }
        _ => {
            let culprit_movements = request_movement_nodes.iter()
                        .map(|x| x.movement())
                        .collect::<Vec<_>>();
            panic!("two or more movements. the culprits are {:#?}", culprit_movements)
        },   
    }
}

//each variant might have a different signature for the exit
fn can_exit_node<T>(request_movement_node : &Arc<T>,
                 current_movement_node : &FighterMovementNode,
                pos_z : f32, current_movement_duration :f32,) -> bool
    where T: FighterMovementNodeTrait {
    match current_movement_node {
        FighterMovementNode::EventTriggered(node) => {
            (node.player_can_exit)(FLOOR_Z, pos_z, current_movement_duration, &request_movement_node.movement())
        }
        FighterMovementNode::Persistent(node) => {
            (node.player_can_exit)(FLOOR_Z, pos_z, current_movement_duration, &request_movement_node.movement())
        }
        FighterMovementNode::Uncontrollable(_) => true,
    }
}

fn player_control(mut query: Query<(&Fighter,
                                    &PlayerControls,
                                    &mut KeyTargetSetStack,
                                    &mut FighterMovementStack,
                                    &mut FighterPosition,
                                    &mut FighterVelocity,
                                    &mut FacingEast,)>,
                                    keyboard_input_resource: Res<Input<KeyCode>>,
                                    time: Res<Time>,
                                    figher_movement_map_collection: Res<FighterMovementMapCollection>,
                                    ) {
    let keyboard_input = keyboard_input_resource.into_inner();

    for (fighter,
        player_controls,
        mut event_keytargetset_stack,
        mut movement_stack,
        mut position,
        mut velocity,
        mut facing_east) in query.iter_mut() {

        let fighter_map = figher_movement_map_collection.0.get(&fighter).unwrap();

        //update event_keytargetset_stack and movement stack
        movement_stack.0.update(time.delta_seconds());
        let event_keytargetset = player_controls.into_event_keytargetset(&keyboard_input);
        event_keytargetset_stack.0.update(time.delta_seconds());
        event_keytargetset_stack.0.push(event_keytargetset.clone());

        let current_durative_movement = movement_stack.last()
            .expect("movement_stack is empty").clone();
        let current_movement_node = fighter_map
                .get_node_by_movement(&current_durative_movement.value)
                .expect("Failed to get last movement node");

        //try an event triggered from current event_keytargetset
        if let Some(movement_nodes) = fighter_map.event_map.get(&event_keytargetset) {
            let filtered_request_nodes = movement_nodes.iter().filter(|request_movement_node| {
                let can_enter = (request_movement_node.player_can_enter)(FLOOR_Z, position.z, &movement_stack, &mut event_keytargetset_stack, false);
                let can_exit = can_exit_node(request_movement_node, current_movement_node, position.z, current_durative_movement.duration);
                can_enter & can_exit
                }).collect::<Vec<_>>();
            if let Some(_) = enter_requested_node(filtered_request_nodes,
                 &mut movement_stack, &mut position, &mut velocity, &mut facing_east) {
                continue
            };
        }

        //try an event triggered movement from joined keytargetset
        let joined_event_keytargetset = event_keytargetset_stack.join();
        if let Some(movement_nodes) = fighter_map.event_map.get(&joined_event_keytargetset) {
            let filtered_request_nodes = movement_nodes.iter().filter(|request_movement_node| {
                let can_enter = (request_movement_node.player_can_enter)(FLOOR_Z, position.z, &movement_stack, &mut event_keytargetset_stack, true);
                let can_exit = can_exit_node(request_movement_node, current_movement_node, position.z, current_durative_movement.duration);
                can_enter & can_exit
                }).collect::<Vec<_>>();
            if let Some(_) = enter_requested_node(filtered_request_nodes,
                 &mut movement_stack, &mut position, &mut velocity, &mut facing_east) {
                continue
            };
        }        

        //check if repeating persistent movement and if not, try to enter a new persitent movement
        let persistent_keytargetset = player_controls.into_persistent_keytargetset(&keyboard_input);
        if let Some(movement_nodes) = fighter_map.persistent_map.get(&persistent_keytargetset) {
            let repeating_movement = movement_nodes.iter()
                        .fold(false,|v,x| v || x.base.movement == current_durative_movement.value);
            if repeating_movement {continue}; 

            let filtered_request_nodes = movement_nodes.iter().filter(|request_movement_node| {
                let can_enter = (request_movement_node.player_can_enter)(FLOOR_Z, position.z);
                let can_exit = can_exit_node(request_movement_node, current_movement_node, position.z, current_durative_movement.duration);
                can_enter & can_exit
                }).collect::<Vec<_>>();
            
            //
            if let Some(_) = enter_requested_node(filtered_request_nodes,
                &mut movement_stack,&mut position,&mut velocity, &mut facing_east) {
                continue
            };
        }

        //try to enter idle
        if current_durative_movement.value != FighterMovement::Idle {
            let idle_node = &fighter_map.get_uncontrollable_node(&FighterMovement::Idle)
                .expect("Failed to get idle node");
            let can_enter = (idle_node.player_can_enter)(FLOOR_Z, position.z);
            let can_exit = can_exit_node(&idle_node, current_movement_node, position.z, current_durative_movement.duration);

            if can_enter && can_exit {
                movement_stack.0.push(FighterMovement::Idle);
                (idle_node.base.state_enter)(&mut position, &mut velocity, &mut facing_east);
                continue
            }
        }
    
        //if all else failed, see if its a channel
        let full_keytargetset = player_controls.into_full_keytargetset(&keyboard_input);
        if let FighterMovementNode::EventTriggered(node) = current_movement_node {
            if let Some(channel) = node.channel {
                channel(&full_keytargetset ,&mut velocity)
            }
        }
    }  
}
fn update_state(mut query: Query<(&Fighter,
                                    &mut FighterPosition,
                                    &mut FighterVelocity,
                                    &mut FighterHealth,
                                    &FighterHitBox,
                                    &FighterHurtBox,
                                    &mut FighterMovementStack,)>,
                                    time: Res<Time>,
                                    figher_movement_map_collection: Res<FighterMovementMapCollection>,) {
    let dt = time.delta_seconds();
    
    for (fighter,
        mut position,
        mut velocity,
        mut health,
        hitbox,
        hurtbox,
        mut movement_stack) in query.iter_mut() {

        let fighter_map = figher_movement_map_collection.0.get(&fighter)
            .expect("fighter does not exist in the movement graph");
        if let Some(current_durative_movement) = movement_stack.last() {
            let movement_node = fighter_map.get_node_by_movement(&current_durative_movement.value)
                .expect("movement wasn't found in fighter_map");

            if let FighterMovementNode::EventTriggered(node) = movement_node {
                if let Some(duration_and_fallback) = &node.duration_and_fallback {
                    if current_durative_movement.duration > duration_and_fallback.duration {
                        movement_stack.push(duration_and_fallback.fallback);
                        continue;
                    }
                }
            }

            movement_node.state_update(&mut position, &mut velocity, dt);
            position.x = position.x.clamp(WEST_WALL_X,EAST_WALL_X);
            position.y = position.y.clamp(SOUTH_WALL_Y, NORTH_WALL_Y);
            position.z = position.z.clamp(FLOOR_Z, CEILING_Z);
        }
    }
}

fn update_healthbars(fighter_health_query: Query<&FighterHealth>,
                        mut statbar_query : Query<(&StatBarData, &mut Sprite)>) {
    for (data,
         mut sprite) in statbar_query.iter_mut() {
        if let Ok(health) = fighter_health_query.get(data.target_entity) {
            let value = health.current/health.max;
            sprite.rect = Some(Rect {
                min :  Vec2::new(0.0, 0.0),
                max : Vec2::new(data.max_length * value, data.thickness),
           });
        }
    }
}

fn update_shadows(query_fighter_position: Query<&FighterPosition>,
                mut query_fighter_shadows: Query<(&mut Transform, &mut ShadowData)>) {
    for (mut transform,
        shadow) in query_fighter_shadows.iter_mut() {
        if let Ok(position) = query_fighter_position.get(shadow.target_entity) {
            let uvw = project_xyz_2_uvw([position.x, position.y, FLOOR_Z + shadow.height_offset]);
            transform.translation = Vec3::new(uvw[0], uvw[1], shadow.z);
        }
    }
}

fn draw_fighters(time: Res<Time>,
                figher_movement_map_collection: Res<FighterMovementMapCollection>,
                fighters_movement_animation_indicies: Res<FightersMovementAnimationIndicies>,
                mut animation_timer: ResMut<AnimationTimer>,
                mut query: Query<(&Fighter,
                                &FighterMovementStack,
                                &FighterPosition,
                                Ref<FacingEast>,
                                &mut TextureAtlasSprite,
                                &mut Transform,)>) {
    
    animation_timer.tick(Duration::from_secs_f32(time.delta_seconds()));
    for (fighter,
        movement_stack,
        position,
        facing_right,
        mut sprite,
        mut transform) in query.iter_mut() {
        
        if let Some(last_durative_movement) = movement_stack.0.stack.last() {
            let movement_node = figher_movement_map_collection.0.get(&fighter).unwrap()
                                                                .get_node_by_movement(&last_durative_movement.value).unwrap();
            let sprite_name = movement_node.sprite_name();

            if animation_timer.just_finished() {
                let movement_indicies = fighters_movement_animation_indicies.0.get(&fighter).unwrap()
                                                                                    .hashmap.get(sprite_name).unwrap();
                if sprite.index < movement_indicies[0] || sprite.index > movement_indicies[1]-1 {
                    sprite.index = movement_indicies[0];
                } else {
                    sprite.index += 1;
                }
            }
            
            let uvw = project_xyz_2_uvw(position.into());
            transform.translation = Vec3::new(uvw[0], uvw[1], uvw[2]);
    
            if facing_right.is_changed() {
                sprite.flip_x = !facing_right.0;
            }
        }
    }
}