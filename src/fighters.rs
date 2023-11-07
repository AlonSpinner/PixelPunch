use bevy::prelude::*;
use strum_macros::{EnumString, Display};
use super::controls::{KeyTargetSet,KeyTarget};
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use lazy_static::lazy_static;

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
        }
    }

    pub fn enter_position_velocity(&self, _fighter_position : &mut FighterPosition,
                                          fighter_velocity : &mut FighterVelocity) {
        match self {
            FighterMovement::Idle => {},
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

#[allow(dead_code)]
pub struct FighterMovementNode {
    pub movement: FighterMovement,
    pub player_enter_condition : fn(position_y : f32, previous_movement : &FighterMovement) -> bool,
    pub player_leave_condition : fn(position_y : f32, movement_duration : usize) -> bool,
    pub enemy_enter_condition : fn() -> bool,
    pub enemy_leave_condition : fn() -> bool,
    pub hit_boxes : Vec<HitBox>,
    pub hurt_boxes : Vec<HitBox>,
}

impl FighterMovementNode {
    pub fn player_enter_condition(&self, position_y : f32, previous_movement : &FighterMovement) -> bool {
        (self.player_enter_condition)(position_y, previous_movement)
    }
    pub fn player_leave_condition(&self, position_y : f32, movement_duration : usize) -> bool {
        (self.player_leave_condition)(position_y, movement_duration)
    }
    pub fn enemy_enter_condition(&self) -> bool {
        (self.enemy_enter_condition)()
    }
    pub fn enemy_leave_condition(&self) -> bool {
        (self.enemy_leave_condition)()
    }
}

#[allow(unused_variables)]
impl Default for FighterMovementNode {
    fn default() -> Self {
        Self{
            movement: FighterMovement::Idle,
            player_enter_condition: |position_y, previous_movement| (position_y).abs()<1e-10,
            player_leave_condition: |position_y, movement_duration| position_y == 0.0,
            enemy_enter_condition: || false,
            enemy_leave_condition: || false,
            hit_boxes: Vec::new(),
            hurt_boxes: Vec::new(),
        }
    }
}

//A static graph of all possible movements for a fighter. NO DYNAMIC DATA.
pub struct FighterMovementMap{
    pub nodes : HashMap<KeyTargetSet,FighterMovementNode>
}

impl FighterMovementMap {
    fn default() -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(KeyTargetSet::empty(), FighterMovementNode{
            movement: FighterMovement::Idle,
            ..default()});
        nodes.insert(KeyTargetSet::from([KeyTarget::Up]), FighterMovementNode{
            movement: FighterMovement::Jumping{inital_velocity: JUMPING_SPEED, gravity: GRAVITY},
            ..default()});
        nodes.insert(KeyTargetSet::from([KeyTarget::Down]), FighterMovementNode{
            movement: FighterMovement::Docking,
            ..default()});
        nodes.insert(KeyTargetSet::from([KeyTarget::Left]), FighterMovementNode{
            movement: FighterMovement::Walking{velocity: -WALKING_SPEED},
            ..default()});
        nodes.insert(KeyTargetSet::from([KeyTarget::Right]), FighterMovementNode{
            movement: FighterMovement::Walking{velocity: WALKING_SPEED},
            ..default()});
        nodes.insert(KeyTargetSet::from([KeyTarget::Left]), FighterMovementNode{
            movement: FighterMovement::Running{velocity: -RUNNING_SPEED},
            ..default()});
        nodes.insert(KeyTargetSet::from([KeyTarget::Right]), FighterMovementNode{
            movement: FighterMovement::Running{velocity: RUNNING_SPEED},
            ..default()});
        Self{ nodes :nodes}
    }
    pub fn movements(&self) -> Vec<String> {
        self.nodes.values().map(|v| v.movement.to_string()).collect()
    }
}

impl Add for FighterMovementMap {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut nodes = self.nodes;
        nodes.extend(other.nodes);
        Self{
            nodes,
        }
    }
}

