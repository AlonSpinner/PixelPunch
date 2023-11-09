use bevy::prelude::*;
use strum_macros::{EnumString, Display};
use super::controls::{KeyTargetSet,KeyTarget};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use lazy_static::lazy_static;
use std::sync::Arc;

//movement
pub const WALKING_SPEED : f32 = 100.0;
pub const RUNNING_SPEED : f32 = 200.0;
pub const JUMPING_SPEED : f32 = 100.0;
pub const GRAVITY : f32 = -100.0;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Display)]
pub enum Fighter{
    IDF,
    HAMAS,
}

lazy_static! {
pub static ref FIGHTERS_MOVEMENT_GRAPH : HashMap<Fighter, FighterMovementMap> = {
    let mut hashmap = HashMap::new();
    hashmap.insert(Fighter::IDF, FighterMovementMap::default());
    hashmap.insert(Fighter::HAMAS, FighterMovementMap::default());
    hashmap
    };
}

#[derive(Component)]
pub struct FighterHealth(pub f32);
#[derive(Component)]
pub struct FighterPosition {
    pub x : f32,
    pub y : f32,
}
#[derive(Component)]
pub struct FighterVelocity {
    pub x : f32,
    pub y : f32,
}

#[derive(Component)]
pub struct FighterMovementStack(pub Vec<(f32, FighterMovement)>);

impl FighterMovementStack {
    pub fn insert(&mut self, elapsed_timed : f32, movement: FighterMovement) {
        self.0.insert(0, (elapsed_timed, movement));
        if self.0.len() > 10 {
            self.0.truncate(10);
        }
    }
}

#[derive(Component)]
pub struct FighterMovementDuration(pub usize);

//All possible movements for a fighter
#[derive(Component, Clone, Copy, Debug, PartialEq, EnumString, Display)]
pub enum FighterMovement {
    Idle,
    #[strum(to_string = "JumpLoop")]
    Jumping{inital_velocity : f32, gravity : f32},
    #[strum(to_string = "Sliding")]
    Docking,
    Running{velocity : f32},
    Walking{velocity : f32},
}

impl FighterMovement {
    //don't change to the same movement
    pub fn change_to(&mut self, new_movement: Self) {
        if &new_movement != self {
            *self = new_movement;
            info!("Changed to {}", self.to_string());
        }
    }

    //helper function to get the name of the movement instead of to_string
    pub fn name(&self) -> String {
        self.to_string()
    }

    pub fn enter_position_velocity(&self, _fighter_position : &mut FighterPosition,
                                          fighter_velocity : &mut FighterVelocity) {
        match self {
            FighterMovement::Idle => {
                fighter_velocity.x = 0.0;
                fighter_velocity.y = 0.0;
            },
            FighterMovement::Jumping{inital_velocity, gravity: _} => {
                fighter_velocity.y = *inital_velocity;
            },
            FighterMovement::Docking => {},
            FighterMovement::Running{velocity} => {
                fighter_velocity.x = *velocity;
            },
            FighterMovement::Walking{velocity} => {
                fighter_velocity.x = *velocity;
            },
        }
    }

    pub fn update_position_velocity(&self, figther_position : &mut FighterPosition,
                                           fighter_velocity : &mut FighterVelocity,
                                           delta_time : f32) {
        match self {
            FighterMovement::Idle => {},
            FighterMovement::Jumping{inital_velocity: _, gravity} => {
                figther_position.y += fighter_velocity.y * delta_time;
                fighter_velocity.y += (*gravity) * delta_time;
                figther_position.x += fighter_velocity.x * delta_time;
            },
            FighterMovement::Docking => {},
            FighterMovement::Running{velocity: _} => {
                figther_position.x += fighter_velocity.x * delta_time;
            },
            FighterMovement::Walking{velocity: _} => {
                figther_position.x += fighter_velocity.x * delta_time;
            },
        }
    }
}

pub struct HitBox;

//datatype that expands FigherMovement with static data building the FighterMovementGraph
#[allow(dead_code)]
pub struct FighterMovementNode {
    pub movement: FighterMovement,
    pub player_enter_condition : fn(floor_y : f32, position_y : f32, previous_movement : &FighterMovement) -> bool,
    pub player_leave_condition : fn(floor_y : f32, position_y : f32, movement_duration : usize) -> bool,
    pub hit_boxes : Vec<HitBox>,
    pub hurt_boxes : Vec<HitBox>,
    pub index : usize,
}

impl FighterMovementNode {
    pub fn player_enter_condition(&self, floor_y : f32,  position_y : f32, previous_movement : &FighterMovement) -> bool {
        (self.player_enter_condition)(floor_y, position_y, previous_movement)
    }
    pub fn player_leave_condition(&self, floor_y :f32,  position_y : f32, movement_duration : usize) -> bool {
        (self.player_leave_condition)(floor_y, position_y, movement_duration)
    }
}

#[allow(unused_variables)]
impl Default for FighterMovementNode {
    fn default() -> Self {
        Self{
            movement: FighterMovement::Idle,
            player_enter_condition: |floor_y, position_y, previous_movement|
                                        position_y == floor_y,
            player_leave_condition: |floor_y, position_y, movement_duration|
                                        position_y == floor_y && movement_duration > 5,
            hit_boxes: Vec::new(),
            hurt_boxes: Vec::new(),
            index : 0,
        }
    }
}

//A static graph of all possible movements for a fighter. NO DYNAMIC DATA.
pub struct FighterMovementMap {
    pub keyset_map : HashMap<KeyTargetSet,Vec<Arc<FighterMovementNode>>>,
    pub index_map : HashMap<usize, Arc<FighterMovementNode>>,
}

impl FighterMovementMap {
    fn new() -> Self {
        Self{
            keyset_map : HashMap::new(),
            index_map : HashMap::new()
        }
    }

    fn insert(&mut self, keyset : KeyTargetSet, mut movement_node : FighterMovementNode) {
        let index = self.index_map.len();
        movement_node.index = index;
        let arc_node = Arc::new(movement_node);
        self.index_map.insert(index, Arc::clone(&arc_node));
        if let Some(existing_nodes) = self.keyset_map.get_mut(&keyset) {
            existing_nodes.push(arc_node);
        } else {
            self.keyset_map.insert(keyset, vec![arc_node]);
        }
    }

    pub fn movement_names(&self) -> Vec<String> {
        self.index_map.values()
            .map(|node| node.movement.name())
            .fold(Vec::new(), |mut acc, name| {
            if !acc.contains(&name) {
                acc.push(name);
            }
            acc
        })
    }
}

impl Default for FighterMovementMap {
    fn default() -> Self {
        let mut map = Self::new();
        map.insert(KeyTargetSet::empty(), 
        FighterMovementNode{movement: FighterMovement::Idle,..default()});
        map.insert(KeyTargetSet::from([KeyTarget::Up]), 
        FighterMovementNode{movement: FighterMovement::Jumping{inital_velocity: JUMPING_SPEED, gravity: GRAVITY}
                            ,..default()});
        map.insert(KeyTargetSet::from([KeyTarget::Down]), 
        FighterMovementNode{movement: FighterMovement::Docking,..default()});
        map.insert(KeyTargetSet::from([KeyTarget::Left]), 
        FighterMovementNode{movement: FighterMovement::Walking{velocity: -WALKING_SPEED},..default()});
        map.insert(KeyTargetSet::from([KeyTarget::Right]), 
        FighterMovementNode{movement: FighterMovement::Walking{velocity: WALKING_SPEED},
                player_enter_condition: |floor_y, position_y, previous_movement| position_y == floor_y && previous_movement.name() != "Running",
                ..default()});
        
        map.insert(KeyTargetSet::from([KeyTarget::LeftJustPressed]), 
        FighterMovementNode{movement: FighterMovement::Running{velocity: -RUNNING_SPEED},..default()});
        map.insert(KeyTargetSet::from([KeyTarget::RightJustPressed]), 
        FighterMovementNode{movement: FighterMovement::Running{velocity: RUNNING_SPEED},
                player_enter_condition: |floor_y, position_y, previous_movement| position_y == floor_y && previous_movement.name() == "Walking",
                ..default()});

        map
    }
}

impl Add for FighterMovementMap {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_map = self;
        for (keyset, movement_nodes) in rhs.keyset_map {
            for arc_movement_node in movement_nodes {
                let movement_node = Arc::into_inner(arc_movement_node).unwrap();
                new_map.insert(keyset.clone(), movement_node);
            }
        }
        new_map
    }
}

#[derive(Component)]
pub struct FighterMovementNodeIndex(pub usize);