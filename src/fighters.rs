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
    Slashing,
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
            FighterMovement::Slashing => {
                fighter_velocity.x = 0.0;
                fighter_velocity.y = 0.0;
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
            FighterMovement::Slashing => {}
        }
    }
}

pub struct HitBox;

pub struct PersistentFighterMovementNode {
    name: String,
    pub movement : FighterMovement,
    pub hit_box : HitBox,
    pub hurt_box : HitBox,
    pub player_enter_condition : fn(floor_y : f32, position_y : f32) -> bool,
}

impl Default for PersistentFighterMovementNode {
    fn default() -> Self {
        Self{
            name: "Idle".to_string(),
            movement: FighterMovement::Idle,
            player_enter_condition: |floor_y, position_y|
                                        position_y == floor_y,
            hit_box: HitBox,
            hurt_box: HitBox,
        }
    }
}

pub struct EventFighterMovementNode {
    pub name: String,
    pub movement: FighterMovement,
    pub player_enter_condition : fn(floor_y : f32, position_y : f32, previous_movement : &FighterMovement) -> bool,
    pub player_leave_condition : fn(floor_y : f32, position_y : f32, movement_duration : f32) -> bool,
    pub hit_boxes : Vec<HitBox>,
    pub hurt_boxes : Vec<HitBox>,
}

impl EventFighterMovementNode {
    pub fn player_enter_condition(&self, floor_y : f32,  position_y : f32, previous_movement : &FighterMovement) -> bool {
        (self.player_enter_condition)(floor_y, position_y, previous_movement)
    }
    pub fn player_leave_condition(&self, floor_y :f32,  position_y : f32, movement_duration : f32) -> bool {
        (self.player_leave_condition)(floor_y, position_y, movement_duration)
    }
}

//A static graph of all possible movements for a fighter. NO DYNAMIC DATA.
pub struct FighterMovementMap {
    pub event_map : HashMap<KeyTargetSet,Arc<EventFighterMovementNode>>,
    pub persistent_map : HashMap<KeyTargetSet,Arc<PersistentFighterMovementNode>>,
}

impl FighterMovementMap {
    fn new() -> Self {
        Self{
            persistent_map : HashMap::new(),
            event_map : HashMap::new(),
        }
    }

    pub fn insert_event(&mut self, keyset : KeyTargetSet, movement_node : EventFighterMovementNode) {
        let arc_movement_node = Arc::new(movement_node);
        self.event_map.insert(keyset, arc_movement_node);
    }

    pub fn insert_persistent(&mut self, keyset : KeyTargetSet, movement_node : PersistentFighterMovementNode) {
        let arc_movement_node = Arc::new(movement_node);
        self.persistent_map.insert(keyset, arc_movement_node);
    }

    pub fn movement_names(&self) -> Vec<String> {
        let acc = self.persistent_map.values().map(|node| node.name.clone())
        .fold(Vec::new(), |mut acc, name| {
            if !acc.contains(&name) {
                acc.push(name);
            }
            acc
        });
        let acc = self.event_map.values().map(|node| node.name.clone())
        .fold(acc, |mut acc, name| {
            if !acc.contains(&name) {
                acc.push(name);
            }
            acc
        });

        acc
    }
}

impl Default for FighterMovementMap {
    fn default() -> Self {
        let mut map = Self::new();
        map.insert_persistent(KeyTargetSet::empty(),
        PersistentFighterMovementNode{name : "Idle".to_string(),
                                                    movement: FighterMovement::Idle,..default()});
        map.insert_persistent(KeyTargetSet::from([KeyTarget::Down]), 
        PersistentFighterMovementNode{name : "Docking".to_string(),
                                                    movement: FighterMovement::Docking,..default()});
        map.insert_persistent(KeyTargetSet::from([KeyTarget::Left]),
        PersistentFighterMovementNode{name : "WalkingLeft".to_string(),
                                                    movement: FighterMovement::Walking{velocity: -WALKING_SPEED},..default()});
        map.insert_persistent(KeyTargetSet::from([KeyTarget::Right]),
        PersistentFighterMovementNode{name : "WalkingRight".to_string(),
                                                    movement: FighterMovement::Walking{velocity: WALKING_SPEED},..default()});
    
        map.insert_event(KeyTargetSet::from([KeyTarget::Up]),
            EventFighterMovementNode{name: "Jump".to_string(),
                                              movement: FighterMovement::Jumping{inital_velocity: JUMPING_SPEED, gravity: GRAVITY}
                                            ,..default()});
        // map.insert_event(KeyTargetSet::from([KeyTarget::LeftJustPressed]), 
        // FighterMovementNode{movement: FighterMovement::Running{velocity: -RUNNING_SPEED},
        //         ..default()});
        // map.insert_event(KeyTargetSet::from([KeyTarget::RightJustPressed]), 
        // FighterMovementNode{movement: FighterMovement::Running{velocity: RUNNING_SPEED},
        //         player_enter_condition: |floor_y, position_y, previous_movement| position_y == floor_y && previous_movement.name() == "Walking",
        //         ..default()});
        // map.insert_event(KeyTargetSet::from([KeyTarget::AttackJustPressed]),
        // FighterMovementNode{movement: FighterMovement::Slashing,
        //         player_leave_condition: |floor_y, position_y, movement_duration| position_y == floor_y && movement_duration > 1.0,
        //         ..default()});
        map
    }
}