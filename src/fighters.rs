use bevy::prelude::*;
use strum_macros::Display;
use crate::utils::DurativeStack;

use super::controls::{KeyTargetSet,KeyTarget};
use std::collections::HashMap;
use std::hash::Hash;
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
    hashmap.insert(Fighter::IDF, FighterMovementMap::default().ensure_must_exists_movements());
    hashmap.insert(Fighter::HAMAS, FighterMovementMap::default().ensure_must_exists_movements());
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
pub struct FighterMovementDuration(pub f32);

#[derive(Component)]
pub struct FighterMovementNodeName(pub String);

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash)]
pub enum FighterMovement {
    Slashing,
    Jumping,
    RunningRight,
    RunningLeft,
    Idle,
    WalkingRight,
    WalkingLeft,
    Docking,
    InAir,
}

impl FighterMovement {
    //don't change to the same movement
    pub fn change_to(&mut self, new_movement: Self) {
        if &new_movement != self {
            *self = new_movement;
        }
    }

}

#[derive(Component)]
pub struct FighterMovementStack(pub DurativeStack<FighterMovement>);
impl FighterMovementStack {
    pub fn new(max_size : usize, max_duration : f32) -> Self {
        Self(DurativeStack::new(max_size, max_duration))
    }
}
pub struct HitBox;

pub struct FighterMovementNode {
    pub movement: FighterMovement,
    pub player_enter_condition : fn(floor_y : f32,
                                    position_y : f32,
                                    previous_node_name : &String,) -> bool,
    pub player_exit_condition : fn(floor_y : f32,
                                    position_y : f32,
                                    movement_duration : f32,
                                    keyset : &KeyTargetSet) -> bool,
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
    pub fn player_enter_condition(&self, floor_y : f32,  position_y : f32, previous_node_name : &String) -> bool {
        (self.player_enter_condition)(floor_y, position_y, previous_node_name)
    }
    pub fn player_exit_condition(&self, floor_y :f32,  position_y : f32, movement_duration : f32, keyset : &KeyTargetSet) -> bool {
        (self.player_exit_condition)(floor_y, position_y, movement_duration, keyset)
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
            movement: FighterMovement::Idle,
            enter: |_fighter_position, fighter_velocity| {
                fighter_velocity.x = 0.0;
                fighter_velocity.y = 0.0;
            },
            update: |_,_,_| {},
            player_enter_condition: |floor_y,position_y,_| position_y == floor_y,
            player_exit_condition: |floor_y,position_y,_,_| position_y == floor_y,
            hit_boxes: Vec::new(),
            hurt_boxes: Vec::new(),
            sprite_name: "Idle".to_string(),
        }
    }
}

//A static graph of all possible movements for a fighter. NO DYNAMIC DATA.
pub struct FighterMovementMap {
    pub event_map : HashMap<KeyTargetSet,Arc<FighterMovementNode>>,
    pub persistent_map : HashMap<KeyTargetSet,Arc<FighterMovementNode>>,
    pub name_map : HashMap<FighterMovement, Arc<FighterMovementNode>>,
}

impl FighterMovementMap {
    fn new() -> Self {
        Self{
            event_map : HashMap::new(),
            persistent_map : HashMap::new(),
            name_map : HashMap::new(),
        }
    }

    fn ensure_must_exists_movements(self) -> Self{
        let must_exist_movements = [FighterMovement::Idle];
        for movement in must_exist_movements.iter() {
            if !self.name_map.contains_key(movement) {
                panic!("Movement {} must exist in the FighterMovementMap", movement);
            }
        }
    self
    }

    fn check_if_can_insert_node(&mut self, keyset : &KeyTargetSet, node : &FighterMovementNode) {
        if self.event_map.contains_key(&keyset) {
            panic!("Keyset {:?} already contained in event_map", keyset);
        } else if self.persistent_map.contains_key(keyset) {
            panic!("Keyset {:?} already contained in the persistent_map", keyset);
        } else if {
            self.name_map.contains_key(&node.movement)
        } {
            panic!("Node with fighter movment {} already contained in the name_map", node.movement);
        }
    }

    pub fn insert_to_event_map(&mut self, keyset : KeyTargetSet, node : FighterMovementNode) {
        self.check_if_can_insert_node(&keyset, &node);
        let node_name = node.movement.clone();
        let arc_movement_node = Arc::new(node);
        self.name_map.insert(node_name, arc_movement_node.clone());
        self.event_map.insert(keyset, arc_movement_node);
    }

    pub fn insert_to_peristent_map(&mut self, keyset : KeyTargetSet, node : FighterMovementNode) {
        self.check_if_can_insert_node(&keyset, &node);
        let node_name = node.movement.clone();
        let arc_movement_node = Arc::new(node);
        self.name_map.insert(node_name, arc_movement_node.clone());
        self.persistent_map.insert(keyset, arc_movement_node);
    }

    //by_name map may contain nodes that are not in the keyset_map
    pub fn insert_to_name_map(&mut self, node : FighterMovementNode) {
        if self.name_map.contains_key(&node.movement) {
            panic!("node with that name {} already exists in the map", &node.movement);
        } else {
        let node_name = node.movement.clone();
        let arc_movement_node = Arc::new(node);
        self.name_map.insert(node_name, arc_movement_node.clone());
        }
    }
}

impl Default for FighterMovementMap {
    fn default() -> Self {
        let mut map = Self::new();
        map.insert_to_name_map(FighterMovementNode::default());
                    
        map.insert_to_peristent_map(KeyTargetSet::from([KeyTarget::Right]),
        FighterMovementNode{
            movement: FighterMovement::WalkingRight,
            player_exit_condition: |floor_y, position_y, movement_duration,_| 
                position_y == floor_y && movement_duration > 0.1,
            enter: |_, fighter_velocity| {
                fighter_velocity.x = WALKING_SPEED;
            },
            update: |fighter_position, fighter_velocity, delta_time| {
                fighter_position.x += fighter_velocity.x * delta_time;
            },
            sprite_name: "Walking".to_string(),
            ..default()}
        );

        map.insert_to_event_map(KeyTargetSet::from([KeyTarget::RightJustPressed]),
        FighterMovementNode{
            movement: FighterMovement::RunningRight,
            player_enter_condition: |floor_y,position_y, previous_node_name| 
                position_y == floor_y && 
                previous_node_name == "WalkingRight",
            player_exit_condition: |_, _, _, keyset| {keyset != &KeyTargetSet::empty()},
            enter: |_, fighter_velocity| {fighter_velocity.x = RUNNING_SPEED;},
            update: |fighter_position, fighter_velocity, delta_time| {
                fighter_position.x += fighter_velocity.x * delta_time
            },
            sprite_name: "Running".to_string(),
            ..default()}
        );

        map.insert_to_peristent_map(KeyTargetSet::from([KeyTarget::Left]),
        FighterMovementNode{
            movement: FighterMovement::WalkingLeft,
            enter: |_, fighter_velocity| {
                fighter_velocity.x = -WALKING_SPEED;
            },
            update: |fighter_position, fighter_velocity, delta_time| {
                fighter_position.x += fighter_velocity.x * delta_time;
            },
            sprite_name: "Walking".to_string(),
            ..default()}
        );

        map.insert_to_peristent_map(KeyTargetSet::from([KeyTarget::Down]),
        FighterMovementNode{
            movement: FighterMovement::Docking,
            enter: |_, fighter_velocity| {
                fighter_velocity.x = 0.0;
                fighter_velocity.y = 0.0;
            },
            sprite_name: "Sliding".to_string(),
            ..default()}
        );

        map.insert_to_event_map(KeyTargetSet::from([KeyTarget::UpJustPressed]),
        FighterMovementNode{
            movement: FighterMovement::Jumping,
            enter: |_, fighter_velocity| {
                fighter_velocity.y = JUMPING_SPEED;
            },
            update: |fighter_position, fighter_velocity, delta_time| {
                fighter_position.x += fighter_velocity.x * delta_time;
                fighter_position.y += fighter_velocity.y * delta_time;
                fighter_velocity.y += GRAVITY * delta_time;
            },
            sprite_name: "JumpLoop".to_string(),
            ..default()}
        );

        map.insert_to_event_map(KeyTargetSet::from([KeyTarget::AttackJustPressed, KeyTarget::DefendJustPressed]),
        FighterMovementNode{
            movement: FighterMovement::Slashing,
            enter: |_, fighter_velocity| {
                fighter_velocity.x = 0.0;
                fighter_velocity.y = 0.0;
            },
            player_exit_condition: |floor_y, position_y,movement_duration,_| 
                floor_y == position_y && movement_duration > 0.5,
            sprite_name: "Slashing".to_string(),
            ..default()}
        );

        map.insert_to_name_map(
        FighterMovementNode{
            movement: FighterMovement::InAir,
            update: |fighter_position, fighter_velocity, delta_time| {
                fighter_position.x += fighter_velocity.x * delta_time;
                fighter_position.y += fighter_velocity.y * delta_time;
                fighter_velocity.y += GRAVITY * delta_time;
            },
            sprite_name: "JumpLoop".to_string(),
            ..default()}
        );

        map
    }
}