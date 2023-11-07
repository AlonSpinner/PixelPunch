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

    pub fn update_position_velocity(&self, position : &mut FighterPosition, velocity : &mut FighterVelocity, delta_time : f32) {
        match self {
            FighterMovement::Idle => {},
            FighterMovement::Jumping{inital_velocity, gravity} => {
                velocity.y = *inital_velocity + *gravity * delta_time;
                position.y += velocity.y * delta_time;
            },
            FighterMovement::Docking => {},
            FighterMovement::Running{velocity} => {
                position.x += velocity * delta_time;
            },
            FighterMovement::Walking{velocity} => {
                position.x += velocity * delta_time;
            },
        }
    }
}

pub struct HitBox;

pub struct FighterMovementNode {
    movement: FighterMovement,
    player_enter_condition : fn(position_y : f32, previous_movement : FighterMovement) -> bool,
    player_leave_condition : fn(position_y : f32, movement_duration : usize) -> bool,
    enemy_enter_condition : fn() -> bool,
    enemy_leave_condition : fn() -> bool,
    hit_boxes : Vec<HitBox>,
    hurt_boxes : Vec<HitBox>,
}
impl Default for FighterMovementNode {
    fn default() -> Self {
        Self{
            movement: FighterMovement::Idle,
            player_enter_condition: |position_y, previous_movement| position_y == 0.0,
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
    nodes : HashMap<KeyTargetSet,FighterMovementNode>
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

