use bevy::prelude::*;
use strum_macros::Display;
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
pub struct FighterMovementDuration(pub usize);

#[derive(Component)]
pub struct FighterMovementNodeName(pub String);

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq)]
pub enum FighterMovement {
    Slashing,
    Jumping,
    Running,
    Idle,
    Walking,
    Docking,
    InAir,
}

impl FighterMovement {
    //don't change to the same movement
    pub fn change_to(&mut self, new_movement: Self) {
        if &new_movement != self {
            *self = new_movement;
            info!("Changed to {}", self.to_string());
        }
    }

}

pub struct HitBox;

pub struct FighterMovementNode {
    pub name : String,
    pub movement: FighterMovement,
    pub player_enter_condition : fn(floor_y : f32, position_y : f32, previous_movement : &FighterMovement) -> bool,
    pub player_leave_condition : fn(floor_y : f32, position_y : f32, movement_duration : f32) -> bool,
    pub hit_boxes : Vec<HitBox>,
    pub hurt_boxes : Vec<HitBox>,
    pub update : fn(fighter_position : &mut FighterPosition,
                    fighter_velocity : &mut FighterVelocity,
                    delta_time : f32),
    pub enter : fn(fighter_position : &mut FighterPosition,
                   fighter_velocity : &mut FighterVelocity),
    pub sprite_name : String,
}

impl FighterMovementNode {
    pub fn player_enter_condition(&self, floor_y : f32,  position_y : f32, previous_movement : &FighterMovement) -> bool {
        (self.player_enter_condition)(floor_y, position_y, previous_movement)
    }
    pub fn player_leave_condition(&self, floor_y :f32,  position_y : f32, movement_duration : f32) -> bool {
        (self.player_leave_condition)(floor_y, position_y, movement_duration)
    }
    pub fn update(&self, fighter_position : &mut FighterPosition,
                         fighter_velocity : &mut FighterVelocity,
                         delta_time : f32) {
        (self.update)(fighter_position, fighter_velocity, delta_time);
    }
    pub fn enter(&self, fighter_position : &mut FighterPosition,
                        fighter_velocity : &mut FighterVelocity) {
        (self.enter)(fighter_position, fighter_velocity);
    }
}

impl Default for FighterMovementNode {
    fn default() -> Self {
        Self{
            name : "Idle".to_string(),
            movement: FighterMovement::Idle,
            enter: |_,_| {},
            update: |_,_,_| {},
            player_enter_condition: |floor_y,position_y,_| position_y == floor_y,
            player_leave_condition: |floor_y,position_y,_| position_y == floor_y,
            hit_boxes: Vec::new(),
            hurt_boxes: Vec::new(),
            sprite_name: "Idle".to_string(),
        }
    }
}

//A static graph of all possible movements for a fighter. NO DYNAMIC DATA.
pub struct FighterMovementMap {
    pub keyset_map : HashMap<KeyTargetSet,Arc<FighterMovementNode>>,
    pub name_map : HashMap<String, Arc<FighterMovementNode>>,
}

impl FighterMovementMap {
    fn new() -> Self {
        Self{
            keyset_map : HashMap::new(),
            name_map : HashMap::new(),
        }
    }

    pub fn insert_to_maps(&mut self, keyset : KeyTargetSet, node : FighterMovementNode) {
        if self.name_map.contains_key(&node.name) {
            panic!("Keyset {} already exists in the map", &node.name);
        } else if self.keyset_map.contains_key(&keyset) {
            panic!("Keyset {:?} already exists in the map", &keyset);
        } else {
        let node_name = node.name.clone();
        let arc_movement_node = Arc::new(node);
        self.name_map.insert(node_name, arc_movement_node.clone());
        self.keyset_map.insert(keyset, arc_movement_node);
        }
    }

    pub fn insert_to_by_name(&mut self, node : FighterMovementNode) {
        if self.name_map.contains_key(&node.name) {
            panic!("Keyset {} already exists in the map", &node.name);
        } else {
        let node_name = node.name.clone();
        let arc_movement_node = Arc::new(node);
        self.name_map.insert(node_name, arc_movement_node.clone());
        }
    }
}

impl Default for FighterMovementMap {
    fn default() -> Self {
        let mut map = Self::new();
        map.insert_to_maps(KeyTargetSet::empty(),
        FighterMovementNode::default());

        map.insert_to_maps(KeyTargetSet::from([KeyTarget::Right]),
        FighterMovementNode{
            name : "WalkingRight".to_string(),
            movement: FighterMovement::Walking,
            enter: |_, fighter_velocity| {
                fighter_velocity.x = WALKING_SPEED;
            },
            update: |fighter_position, fighter_velocity, delta_time| {
                fighter_position.x += fighter_velocity.x * delta_time;
            },
            sprite_name: "Walking".to_string(),
            ..default()}
        );

        map.insert_to_maps(KeyTargetSet::from([KeyTarget::Left]),
        FighterMovementNode{
            name : "WalkingLeft".to_string(),
            movement: FighterMovement::Walking,
            enter: |_, fighter_velocity| {
                fighter_velocity.x = -WALKING_SPEED;
            },
            update: |fighter_position, fighter_velocity, delta_time| {
                fighter_position.x += fighter_velocity.x * delta_time;
            },
            sprite_name: "Walking".to_string(),
            ..default()}
        );

        map.insert_to_maps(KeyTargetSet::from([KeyTarget::Down]),
        FighterMovementNode{
            name : "Docking".to_string(),
            movement: FighterMovement::Docking,
            enter: |_, fighter_velocity| {
                fighter_velocity.x = -WALKING_SPEED;
            },
            update: |fighter_position, fighter_velocity, delta_time| {
                fighter_position.x += fighter_velocity.x * delta_time;
            },
            sprite_name: "Sliding".to_string(),
            ..default()}
        );

        map.insert_to_by_name(
        FighterMovementNode{
            name : "InAir".to_string(),
            movement: FighterMovement::InAir,
            enter: |_, fighter_velocity| {
                fighter_velocity.x = -WALKING_SPEED;
            },
            update: |fighter_position, fighter_velocity, delta_time| {
                fighter_position.x += fighter_velocity.x * delta_time;
            },
            sprite_name: "JumpLoop".to_string(),
            ..default()}
        );

        map
    }
}