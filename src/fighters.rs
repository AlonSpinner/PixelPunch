use bevy::prelude::*;
use strum_macros::Display;
use crate::controls::KeyTargetSetStack;
use crate::utils::{TimeTaggedStack,TimeTaggedValue};

use super::controls::{KeyTargetSet,KeyTarget};
use std::collections::HashMap;
use std::hash::Hash;
use lazy_static::lazy_static;
use std::sync::Arc;

//movement
pub const WALKING_SPEED : f32 = 100.0;
pub const RUNNING_SPEED : f32 = 200.0;
pub const JUMPING_SPEED : f32 = 200.0;
pub const GRAVITY : f32 = -400.0;

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
    pub x : f32, //right
    pub y : f32, //in
    pub z : f32, //up
}

impl From<&FighterPosition> for [f32;3]{
    fn from(xyz : &FighterPosition) -> [f32;3] {
        [xyz.x,xyz.y,xyz.z]
    }
}

#[derive(Component)]
pub struct FighterVelocity {
    pub x : f32,
    pub y : f32,
    pub z :f32,
}

#[derive(Component)]
pub struct FighterMovementDuration(pub f32);

#[derive(Component)]
pub struct FighterMovementNodeName(pub String);

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, Hash)]
pub enum FighterMovement {
    Idle,
    Slashing,
    Jumping,
    RunningEast,
    RunningWest,
    WalkingEast,
    WalkingWest,
    WalkingNorth,
    WalkingSouth,
    WalkingNorthEast,
    WalkingNorthWest,
    WalkingSouthEast,
    WalkingSouthWest,
    Docking,
    InAir,
    JumpAttack,
}

#[derive(Component)]
pub struct FighterMovementStack(pub TimeTaggedStack<FighterMovement>);
impl FighterMovementStack {
    pub fn new(max_size : usize) -> Self {
        Self(TimeTaggedStack::new(max_size))
    }

    pub fn last_value(&self) -> Option<&TimeTaggedValue<FighterMovement>> {
        self.0.stack.last()
    }

    pub fn push(&mut self, value : FighterMovement) {
        self.0.push(value);
    }
}
pub struct HitBox;

pub struct FighterMovementNodeBase {
    pub movement: FighterMovement,
    pub sprite_name : String,
    pub state_update : fn(fighter_position : &mut FighterPosition,
                    fighter_velocity : &mut FighterVelocity,
                    delta_time : f32),
    pub state_enter : fn(fighter_position : &mut FighterPosition,
                   fighter_velocity : &mut FighterVelocity),
}

pub struct EventFighterMovementNode {
    pub base : FighterMovementNodeBase,
    pub player_can_enter : fn(floor_z : f32,
        position_z : f32,
        fighter_movement_stack : &FighterMovementStack,
        keyset : &mut KeyTargetSetStack) -> bool,
    pub player_can_exit : fn(floor_z : f32,
            position_z : f32,
            movement_duration : f32,
            movement_request : &FighterMovement) -> bool,
    pub channel : Option<fn (full_keyset : &KeyTargetSet, fighter_velocity : &mut FighterVelocity)>,
    pub duration : usize,
    pub hit_boxes : Vec<HitBox>,
    pub hurt_boxes : Vec<HitBox>,
}

pub struct PersistentFighterMovementNode {
    pub base : FighterMovementNodeBase,
    pub player_can_enter : fn(floor_z : f32, position_z : f32,) -> bool,
    pub player_can_exit : fn(floor_z : f32,
            position_z : f32,
            movement_duration : f32,
            movement_request : &FighterMovement) -> bool,
    pub hit_box : HitBox,
    pub hurt_box : HitBox,
}

pub struct UncontrollableFighterMovementNode {
    pub base : FighterMovementNodeBase,
    pub player_can_enter : fn(floor_z : f32, position_z : f32,) -> bool,
    pub hit_box : HitBox,
    pub hurt_box : HitBox,
}

pub enum FighterMovementNode {
    EventTriggered(Arc<EventFighterMovementNode>),
    Persistent(Arc<PersistentFighterMovementNode>),
    Uncontrollable(Arc<UncontrollableFighterMovementNode>),
}

pub struct FighterMovementMap {
    pub event_map : HashMap<KeyTargetSet,Vec<Arc<EventFighterMovementNode>>>,
    pub persistent_map : HashMap<KeyTargetSet,Vec<Arc<PersistentFighterMovementNode>>>,
    pub uncontrollable_map : HashMap<FighterMovement,Arc<UncontrollableFighterMovementNode>>,
    pub movement_map : HashMap<FighterMovement, FighterMovementNode>,
}

impl FighterMovementNode {
    pub fn sprite_name(&self) -> &String {
        match self {
            FighterMovementNode::EventTriggered(node) => {&node.base.sprite_name},
            FighterMovementNode::Persistent(node) => {&node.base.sprite_name},
            FighterMovementNode::Uncontrollable(node) => {&node.base.sprite_name},
        }
    }

    pub fn state_update(&self, pos : &mut FighterPosition, vel : &mut FighterVelocity, dt : f32) {
        let state_update_fn = match self {
            FighterMovementNode::EventTriggered(node) => {&node.base.state_update},
            FighterMovementNode::Persistent(node) => {&node.base.state_update},
            FighterMovementNode::Uncontrollable(node) => {&node.base.state_update},
        };
        state_update_fn(pos,vel,dt);
    }

    pub fn state_enter(&self, pos : &mut FighterPosition, vel : &mut FighterVelocity) {
        let state_update_fn = match self {
            FighterMovementNode::EventTriggered(node) => {&node.base.state_enter},
            FighterMovementNode::Persistent(node) => {&node.base.state_enter},
            FighterMovementNode::Uncontrollable(node) => {&node.base.state_enter},
        };
        state_update_fn(pos,vel);
    }
}

#[derive(Debug)]
pub enum FighterMovementError {
    MovementNotFound(FighterMovement),
}

impl FighterMovementMap {
    fn new() -> Self {
        Self{
            event_map : HashMap::new(),
            persistent_map : HashMap::new(),
            uncontrollable_map : HashMap::new(),
            movement_map : HashMap::new(),
        }
    }

    pub fn get_node_by_movement(&self, movement: &FighterMovement) -> Result<&FighterMovementNode,FighterMovementError> {
        //don't know which type of node? your gonna neee to match! HAHAH
        self.movement_map.get(movement).ok_or(FighterMovementError::MovementNotFound(movement.clone()))
    }

    pub fn get_uncontrollable_node(&self, movement: &FighterMovement) -> Result<Arc<UncontrollableFighterMovementNode>,FighterMovementError> {
        match self.movement_map.get(movement) {
            Some(FighterMovementNode::Uncontrollable(arc)) => Ok(Arc::clone(arc)),
            _ => Err(FighterMovementError::MovementNotFound(movement.clone())),
        }
    }

    pub fn get_event_node_by_movement(&self, movement: &FighterMovement) -> Result<Arc<EventFighterMovementNode>,FighterMovementError> {
        match self.movement_map.get(movement) {
            Some(FighterMovementNode::EventTriggered(arc)) => Ok(Arc::clone(arc)),
            _ => Err(FighterMovementError::MovementNotFound(movement.clone())),
        }
    }

    pub fn get_persistent_node_by_movement(&self, movement: &FighterMovement) -> Result<Arc<PersistentFighterMovementNode>,FighterMovementError> {
        match self.movement_map.get(movement) {
            Some(FighterMovementNode::Persistent(arc)) => Ok(Arc::clone(arc)),
            _ => Err(FighterMovementError::MovementNotFound(movement.clone())),
        }
    }

    fn ensure_must_exists_movements(self) -> Self{
        let must_exist_movements = [FighterMovement::Idle];
        for movement in must_exist_movements.iter() {
            if !self.movement_map.contains_key(movement) {
                panic!("Movement {} must exist in the FighterMovementMap", movement);
            }
        }
    self
    }

    fn check_if_can_insert_node(&mut self, movement : &FighterMovement) {
        if self.movement_map.contains_key(&movement) {
            panic!("Node with fighter movement {} already contained in the movement_map", movement);
        }
    }

    fn insert_to_event_map(&mut self, keyset : KeyTargetSet, node : EventFighterMovementNode) {
        self.check_if_can_insert_node(&node.base.movement);
        let node_movement = node.base.movement.clone();
        let arc_movement_node = Arc::new(node);
        self.movement_map.insert(node_movement, FighterMovementNode::EventTriggered(arc_movement_node.clone()));
        if self.event_map.contains_key(&keyset) {
            self.event_map.get_mut(&keyset).unwrap().push(arc_movement_node);
        } else {
            self.event_map.insert(keyset, vec![arc_movement_node]);
        }
    }

    fn insert_to_persistent_map(&mut self, keyset : KeyTargetSet, node : PersistentFighterMovementNode) {
        self.check_if_can_insert_node(&node.base.movement);
        let node_movement = node.base.movement.clone();
        let arc_movement_node = Arc::new(node);
        self.movement_map.insert(node_movement, FighterMovementNode::Persistent(arc_movement_node.clone()));
        if self.persistent_map.contains_key(&keyset) {
            self.persistent_map.get_mut(&keyset).unwrap().push(arc_movement_node);
        } else {
            self.persistent_map.insert(keyset, vec![arc_movement_node]);
        }
    }

    fn insert_to_uncontrollable_map(&mut self, node : UncontrollableFighterMovementNode) {
        self.check_if_can_insert_node(&node.base.movement);
        let node_movement = node.base.movement.clone();
        let arc_movement_node = Arc::new(node);
        self.movement_map.insert(node_movement, FighterMovementNode::Uncontrollable(arc_movement_node.clone()));
        self.uncontrollable_map.insert(node_movement.clone(), arc_movement_node);
    }
}

impl Default for FighterMovementMap {
    fn default() -> Self {
        let mut map = Self::new();
        map.insert_to_uncontrollable_map(UncontrollableFighterMovementNode {
            base: FighterMovementNodeBase { 
                movement: FighterMovement::Idle,
                sprite_name: "Idle".to_string(),
                state_update: |_,_,_| {},
                state_enter: |_,vel| {vel.x = 0.0; vel.y = 0.0}, 
            },
            player_can_enter: |floor_z,z| floor_z == z,
            hit_box: HitBox,
            hurt_box: HitBox
        });

         map.insert_to_uncontrollable_map(UncontrollableFighterMovementNode {
             base: FighterMovementNodeBase { 
                 movement: FighterMovement::InAir,
                 sprite_name: "JumpLoop".to_string(),
                 state_update: |pos,vel,dt| {
                    pos.x += vel.x*dt;
                    pos.y += vel.y*dt;
                    pos.z += vel.z*dt;
                    vel.z += GRAVITY*dt;
                 },
                 state_enter: |_,_| {}, 
             },
             player_can_enter: |floor_z,z| floor_z != z,
             hit_box: HitBox,
             hurt_box: HitBox
        });
                    
        map.insert_to_persistent_map(KeyTargetSet::from([KeyTarget::Right]),
        PersistentFighterMovementNode { 
            base: FighterMovementNodeBase { 
                movement: FighterMovement::WalkingEast,
                sprite_name: "Walking".to_string(),
                state_update: |pos,vel,dt| {
                    pos.x += vel.x*dt;
                },
                state_enter: |_,vel| {
                    vel.x = WALKING_SPEED;
                    vel.y = 0.0;
                    }, 
            },
            player_can_enter: |floor_z, position_z| floor_z == position_z,
            player_can_exit: |_,_,_,_| true,
            hit_box: HitBox,
            hurt_box: HitBox, 
        });

        map.insert_to_persistent_map(KeyTargetSet::from([KeyTarget::Left]),
        PersistentFighterMovementNode { 
            base: FighterMovementNodeBase { 
                movement: FighterMovement::WalkingWest,
                sprite_name: "Walking".to_string(),
                state_update: |pos,vel,dt| {
                    pos.x += vel.x*dt;
                },
                state_enter: |_,vel| {
                    vel.x = -WALKING_SPEED;
                    vel.y = 0.0;
                    }, 
            },
            player_can_enter: |floor_z, position_z| floor_z == position_z,
            player_can_exit: |_,_,_,_| true,
            hit_box: HitBox,
            hurt_box: HitBox, 
        });

        map.insert_to_persistent_map(KeyTargetSet::from([KeyTarget::Up]),
        PersistentFighterMovementNode { 
            base: FighterMovementNodeBase { 
                movement: FighterMovement::WalkingNorth,
                sprite_name: "Walking".to_string(),
                state_update: |pos,vel,dt| {
                    pos.y += vel.y*dt;
                },
                state_enter: |_,vel| {
                    vel.x = 0.0;
                    vel.y = WALKING_SPEED;
                    }, 
            },
            player_can_enter: |floor_z, position_z| floor_z == position_z,
            player_can_exit: |_,_,_,_| true,
            hit_box: HitBox,
            hurt_box: HitBox, 
        });

        map.insert_to_persistent_map(KeyTargetSet::from([KeyTarget::Down]),
        PersistentFighterMovementNode { 
            base: FighterMovementNodeBase { 
                movement: FighterMovement::WalkingSouth,
                sprite_name: "Walking".to_string(),
                state_update: |pos,vel,dt| {
                    pos.y += vel.y*dt;
                },
                state_enter: |_,vel| {
                    vel.x = 0.0;
                    vel.y = -WALKING_SPEED;
                    }, 
            },
            player_can_enter: |floor_z, position_z| floor_z == position_z,
            player_can_exit: |_,_,_,_| true,
            hit_box: HitBox,
            hurt_box: HitBox, 
        });

        map.insert_to_persistent_map(KeyTargetSet::from([KeyTarget::Up,KeyTarget::Right]),
        PersistentFighterMovementNode { 
            base: FighterMovementNodeBase { 
                movement: FighterMovement::WalkingNorthEast,
                sprite_name: "Walking".to_string(),
                state_update: |pos,vel,dt| {
                    pos.x += vel.x*dt;
                    pos.y += vel.y*dt;
                },
                state_enter: |_,vel| {
                    vel.x = WALKING_SPEED/1.41;
                    vel.y = WALKING_SPEED/1.41;
                    }, 
            },
            player_can_enter: |floor_z, position_z| floor_z == position_z,
            player_can_exit: |_,_,_,_| true,
            hit_box: HitBox,
            hurt_box: HitBox, 
        });

        map.insert_to_persistent_map(KeyTargetSet::from([KeyTarget::Up,KeyTarget::Left]),
        PersistentFighterMovementNode { 
            base: FighterMovementNodeBase { 
                movement: FighterMovement::WalkingNorthWest,
                sprite_name: "Walking".to_string(),
                state_update: |pos,vel,dt| {
                    pos.x += vel.x*dt;
                    pos.y += vel.y*dt;
                },
                state_enter: |_,vel| {
                    vel.x = -WALKING_SPEED/1.41;
                    vel.y = WALKING_SPEED/1.41;
                    }, 
            },
            player_can_enter: |floor_z, position_z| floor_z == position_z,
            player_can_exit: |_,_,_,_| true,
            hit_box: HitBox,
            hurt_box: HitBox, 
        });

        map.insert_to_persistent_map(KeyTargetSet::from([KeyTarget::Down,KeyTarget::Right]),
        PersistentFighterMovementNode { 
            base: FighterMovementNodeBase { 
                movement: FighterMovement::WalkingSouthEast,
                sprite_name: "Walking".to_string(),
                state_update: |pos,vel,dt| {
                    pos.x += vel.x*dt;
                    pos.y += vel.y*dt;
                },
                state_enter: |_,vel| {
                    vel.x = WALKING_SPEED/1.41;
                    vel.y = -WALKING_SPEED/1.41;
                    }, 
            },
            player_can_enter: |floor_z, position_z| floor_z == position_z,
            player_can_exit: |_,_,_,_| true,
            hit_box: HitBox,
            hurt_box: HitBox, 
        });

        map.insert_to_persistent_map(KeyTargetSet::from([KeyTarget::Down,KeyTarget::Left]),
        PersistentFighterMovementNode { 
            base: FighterMovementNodeBase { 
                movement: FighterMovement::WalkingSouthWest,
                sprite_name: "Walking".to_string(),
                state_update: |pos,vel,dt| {
                    pos.x += vel.x*dt;
                    pos.y += vel.y*dt;
                },
                state_enter: |_,vel| {
                    vel.x = -WALKING_SPEED/1.41;
                    vel.y = -WALKING_SPEED/1.41;
                    }, 
            },
            player_can_enter: |floor_z, position_z| floor_z == position_z,
            player_can_exit: |_,_,_,_| true,
            hit_box: HitBox,
            hurt_box: HitBox, 
        });

        // map.insert_to_event_map(KeyTargetSet::from([KeyTarget::RightJustPressed]),
        // FighterMovementNode{
        //     movement: FighterMovement::RunningEast,
        //     player_can_enter: |floor_z,position_z,
        //                             fighter_movement_stack,
        //                             event_keyset_stack| {
        //         let window_time = 0.3;
        //         let cond1 = position_z == floor_z;

        //         //search for double pressed in window
        //         let mut pressed = 0;
        //         let mut elements = 0;
        //         for timed_keyset in event_keyset_stack.0.stack.iter().rev() {
        //             if timed_keyset.duration > window_time || pressed > 1 {break};
        //             if timed_keyset.value.contains(&KeyTarget::RightJustPressed) {pressed += 1};
        //             elements += 1;
        //         }
        //         let cond2 = pressed > 1;

        //         //make sure last movement is idle/walking
        //         let mut cond3 = true;
        //         if let Some(last_movement) = fighter_movement_stack.0.stack.last() {
        //             if last_movement.value != FighterMovement::Idle && last_movement.value != FighterMovement::WalkingEast {
        //                cond3 = false;
        //             }
        //         }

        //         let cond =cond1 & cond2 & cond3;

        //         //remove acted upon events from stack
        //         if cond == true {
        //             for _ in 0..elements {
        //                 event_keyset_stack.0.pop();
        //             }
        //         }

        //         cond
        //         },
        //     player_can_exit: |_, _, _, request| {
        //         let unallowed_transitions = [
        //             FighterMovement::WalkingEast,
        //             FighterMovement::WalkingNorth,
        //             FighterMovement::WalkingSouth,
        //             FighterMovement::WalkingNorthEast,
        //             FighterMovement::WalkingSouthEast,
        //             FighterMovement::Idle,
        //         ];

        //         for movement in unallowed_transitions {
        //             if request == &movement {return false};
        //         }
        //         return true;
        //     },
        //     enter: |_, fighter_velocity| {fighter_velocity.x = RUNNING_SPEED;},
        //     update: |fighter_position, fighter_velocity, delta_time| {
        //         fighter_position.x += fighter_velocity.x * delta_time;
        //         fighter_position.y += fighter_velocity.y * delta_time;
        //     },
        //     channel: Some(|full_keyset, fighter_velocity| {
        //         if KeyTargetSet::from([KeyTarget::Up]).is_subset(full_keyset) {
        //             fighter_velocity.y = WALKING_SPEED;
        //         }
        //         if KeyTargetSet::from([KeyTarget::Down]).is_subset(full_keyset) {
        //             fighter_velocity.y = -WALKING_SPEED;
        //         }
        //     }),
        //     sprite_name: "Running".to_string(),
        //     ..default()}
        // );

        // map.insert_to_event_map(KeyTargetSet::from([KeyTarget::AttackJustPressed]),
        // FighterMovementNode{
        //     movement: FighterMovement::JumpAttack,
        //     enter: |_,_| {},
        //     update: |fighter_position, fighter_velocity, delta_time| {
        //         fighter_position.x += fighter_velocity.x * delta_time;
        //         fighter_position.y += fighter_velocity.y * delta_time;
        //         fighter_position.z += fighter_velocity.z * delta_time;
        //         fighter_velocity.z += GRAVITY * delta_time;
        //     },
        //     player_can_enter: |floor_z,position_z,
        //                             fighter_movement_stack,
        //                             _| {
                                        
        //         if floor_z == position_z {return false};
        //         if let Some(last_durative_movement) = fighter_movement_stack.0.stack.last() {
        //             if last_durative_movement.value == FighterMovement::Jumping {return true};
        //         }
        //         return false;
        //     },
        //     sprite_name: "AirSlashing".to_string(),
        //     ..default()}
        // );

        // map.insert_to_event_map(KeyTargetSet::from([KeyTarget::LeftJustPressed]),
        // FighterMovementNode{
        //     movement: FighterMovement::RunningWest,
        //     player_can_enter: |floor_z,position_z,
        //                              fighter_movement_stack,
        //                              event_keyset_stack| {
        //         let window_time = 0.3;
        //         let cond1 = position_z == floor_z;

        //         //search for double pressed in window
        //         let mut pressed = 0;
        //         let mut elements = 0;
        //         for timed_keyset in event_keyset_stack.0.stack.iter().rev() {
        //             if timed_keyset.duration > window_time || pressed > 1 {break};
        //             if timed_keyset.value.contains(&KeyTarget::LeftJustPressed) {
        //                 pressed += 1;
        //             }
        //             elements += 1;
        //         }
        //         let cond2 = pressed > 1;

        //         //make sure last movement is idle/walking
        //         let mut cond3 = true;
        //         if let Some(last_movement) = fighter_movement_stack.0.stack.last() {
        //             if last_movement.value != FighterMovement::Idle && last_movement.value != FighterMovement::WalkingWest {
        //                 cond3 = false;
        //             }
        //         }

        //         let cond =cond1 & cond2 & cond3;

        //         //remove acted upon events from stack
        //         if cond == true {
        //             for _ in 0..elements {
        //                 event_keyset_stack.0.pop();
        //             }
        //         }

        //         cond
        //     },
        //     player_can_exit: |_, _, _, request| {
        //         let unallowed_transitions = [
        //             FighterMovement::WalkingWest,
        //             FighterMovement::WalkingNorth,
        //             FighterMovement::WalkingSouth,
        //             FighterMovement::WalkingNorthWest,
        //             FighterMovement::WalkingSouthWest,
        //             FighterMovement::Idle,
        //         ];

        //         for movement in unallowed_transitions {
        //             if request == &movement {return false};
        //         }
        //         return true;
        //     },
        //     enter: |_, fighter_velocity| {fighter_velocity.x = -RUNNING_SPEED;},
        //     update: |fighter_position, fighter_velocity, delta_time| {
        //         fighter_position.x += fighter_velocity.x * delta_time;
        //         fighter_position.y += fighter_velocity.y * delta_time;
        //     },
        //     channel: Some(|full_keyset, fighter_velocity| {
        //         if KeyTargetSet::from([KeyTarget::Up]).is_subset(full_keyset) {
        //             fighter_velocity.y = WALKING_SPEED/2.0;
        //         }
        //         if KeyTargetSet::from([KeyTarget::Down]).is_subset(full_keyset) {
        //             fighter_velocity.y = -WALKING_SPEED/2.0;
        //         }
        //     }),
        //     sprite_name: "Running".to_string(),
        //     ..default()}
        // );

        // map.insert_to_peristent_map(KeyTargetSet::from([KeyTarget::Defend]),
        // FighterMovementNode{
        //     movement: FighterMovement::Docking,
        //     enter: |_, fighter_velocity| {
        //         fighter_velocity.x = 0.0;
        //         fighter_velocity.y = 0.0;
        //     },
        //     sprite_name: "Sliding".to_string(),
        //     ..default()}
        // );

        // map.insert_to_event_map(KeyTargetSet::from([KeyTarget::JumpJustPressed]),
        // FighterMovementNode{
        //     movement: FighterMovement::Jumping,
        //     enter: |_, fighter_velocity| {
        //         fighter_velocity.z = JUMPING_SPEED;
        //     },
        //     update: |fighter_position, fighter_velocity, delta_time| {
        //         fighter_position.x += fighter_velocity.x * delta_time;
        //         fighter_position.y += fighter_velocity.y * delta_time;
        //         fighter_position.z += fighter_velocity.z * delta_time;
        //         fighter_velocity.z += GRAVITY * delta_time;
        //     },
        //     player_can_exit: |floor_z,position_z,_,request| 
        //         {
        //             if request == &FighterMovement::JumpAttack {
        //                 return true
        //             } else if position_z == floor_z {
        //                 return true}
        //             else {
        //                 return false};
        //         },
                
        //     sprite_name: "JumpLoop".to_string(),
        //     ..default()}
        // );

        // map.insert_to_event_map(KeyTargetSet::from([KeyTarget::AttackJustPressed, KeyTarget::DefendJustPressed]),
        // FighterMovementNode{
        //     movement: FighterMovement::Slashing,
        //     enter: |_, fighter_velocity| {
        //         fighter_velocity.x = 0.0;
        //         fighter_velocity.y = 0.0;
        //     },
        //     player_can_exit: |floor_z, position_z,movement_duration,_| 
        //         floor_z == position_z && movement_duration > 0.5,
        //     sprite_name: "Slashing".to_string(),
        //     ..default()}
        // );

        // map.insert_to_movement_map(
        // FighterMovementNode{
        //     movement: FighterMovement::InAir,
        //     update: |fighter_position, fighter_velocity, delta_time| {
        //         fighter_position.x += fighter_velocity.x * delta_time;
        //         fighter_position.y += fighter_velocity.y * delta_time;
        //         fighter_position.z += fighter_velocity.z * delta_time;
        //         fighter_velocity.z += GRAVITY * delta_time;
        //     },
        //     sprite_name: "JumpLoop".to_string(),
        //     ..default()}
        // );

        map
    }
}