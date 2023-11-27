use super::controls::{KeyTargetSet,KeyTarget,KeyTargetSetStack};
use super::fighters::{Fighter,FighterPosition,FighterVelocity,FighterMovementStack,FighterMovement, HitBox};

use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;

//movement
pub const WALKING_SPEED : f32 = 100.0;
pub const RUNNING_SPEED : f32 = 200.0;
pub const JUMPING_SPEED : f32 = 200.0;
pub const GRAVITY : f32 = -400.0;

pub static FIGHTERS_MOVEMENT_GRAPH : Lazy<HashMap<Fighter, FighterMovementMap>> = Lazy::new(||{
    let mut hashmap = HashMap::new();
    hashmap.insert(Fighter::IDF, FighterMovementMap::default().ensure_must_exists_movements());
    hashmap.insert(Fighter::HAMAS, FighterMovementMap::default().ensure_must_exists_movements());
    hashmap
    });

pub struct DurationAndFallback {
    pub duration : f32,
    pub fallback : FighterMovement,
    pub apply_enter_state_fcn : bool
}

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
        movement_stack : &FighterMovementStack,
        keytargetset_stack : &mut KeyTargetSetStack,
        queried_by_joined_keytargetset : bool) -> bool,
    pub player_can_exit : fn(floor_z : f32,
            position_z : f32,
            movement_duration : f32,
            movement_request : &FighterMovement) -> bool,
    pub channel : Option<fn (full_keyset : &KeyTargetSet, fighter_velocity : &mut FighterVelocity)>,
    pub duration_and_fallback : Option<DurationAndFallback>,
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

pub trait FighterMovementNodeTrait {
    fn movement(&self) -> FighterMovement;
    fn state_enter(&self, fighter_position : &mut FighterPosition,
                        fighter_velocity : &mut FighterVelocity) -> ();
    fn state_update(&self, fighter_position : &mut FighterPosition,
                        fighter_velocity : &mut FighterVelocity,
                        delta_time : f32) -> ();
    fn sprite_name(&self) -> &String;
}

macro_rules! impl_fighter_movement_node_trait {
    ($type:ty) => {
        impl FighterMovementNodeTrait for $type {
            fn movement(&self) -> FighterMovement {
                self.base.movement
            }
            fn state_enter(&self, fighter_position: &mut FighterPosition, fighter_velocity: &mut FighterVelocity) {
                (self.base.state_enter)(fighter_position, fighter_velocity);
            }
            fn state_update(&self, fighter_position: &mut FighterPosition, fighter_velocity: &mut FighterVelocity, delta_time: f32) {
                (self.base.state_update)(fighter_position, fighter_velocity, delta_time);
            }
            fn sprite_name(&self) -> &String {
                &self.base.sprite_name
            }
        }
    };
}

impl_fighter_movement_node_trait!(EventFighterMovementNode);
impl_fighter_movement_node_trait!(PersistentFighterMovementNode);
impl_fighter_movement_node_trait!(UncontrollableFighterMovementNode);

pub enum FighterMovementNode {
    EventTriggered(Arc<EventFighterMovementNode>),
    Persistent(Arc<PersistentFighterMovementNode>),
    Uncontrollable(Arc<UncontrollableFighterMovementNode>),
}

impl FighterMovementNodeTrait for FighterMovementNode {
    fn movement(&self) -> FighterMovement {
        match self {
            FighterMovementNode::EventTriggered(node) => {node.movement()},
            FighterMovementNode::Persistent(node) => {node.movement()},
            FighterMovementNode::Uncontrollable(node) => {node.movement()},
        }
    }

    fn sprite_name(&self) -> &String {
        match self {
            FighterMovementNode::EventTriggered(node) => {node.sprite_name()},
            FighterMovementNode::Persistent(node) => {node.sprite_name()},
            FighterMovementNode::Uncontrollable(node) => {node.sprite_name()},
        }
    }

    fn state_update(&self, pos : &mut FighterPosition, vel : &mut FighterVelocity, dt : f32) {
        match self {
            FighterMovementNode::EventTriggered(node) => {node.state_update(pos,vel,dt)},
            FighterMovementNode::Persistent(node) => {node.state_update(pos,vel,dt)},
            FighterMovementNode::Uncontrollable(node) => {node.state_update(pos,vel,dt)},
        };
    }

    fn state_enter(&self, pos : &mut FighterPosition, vel : &mut FighterVelocity) {
        match self {
            FighterMovementNode::EventTriggered(node) => {node.state_enter(pos,vel)},
            FighterMovementNode::Persistent(node) => {node.state_enter(pos,vel)},
            FighterMovementNode::Uncontrollable(node) => {node.state_enter(pos,vel)}
        };
    }
}

pub struct FighterMovementMap {
    pub event_map : HashMap<KeyTargetSet,Vec<Arc<EventFighterMovementNode>>>,
    pub persistent_map : HashMap<KeyTargetSet,Vec<Arc<PersistentFighterMovementNode>>>,
    pub uncontrollable_map : HashMap<FighterMovement,Arc<UncontrollableFighterMovementNode>>,
    pub movement_map : HashMap<FighterMovement, FighterMovementNode>,
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
            hit_box: HitBox::default(),
            hurt_box: HitBox::default()
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
             hit_box: HitBox::default(),
             hurt_box: HitBox::default()
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
            hit_box: HitBox::default(),
            hurt_box: HitBox::default(), 
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
            hit_box: HitBox::default(),
            hurt_box: HitBox::default(), 
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
            hit_box: HitBox::default(),
            hurt_box: HitBox::default(), 
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
            hit_box: HitBox::default(),
            hurt_box: HitBox::default(), 
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
            hit_box: HitBox::default(),
            hurt_box: HitBox::default(), 
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
            hit_box: HitBox::default(),
            hurt_box: HitBox::default(), 
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
            hit_box: HitBox::default(),
            hurt_box: HitBox::default(), 
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
            hit_box: HitBox::default(),
            hurt_box: HitBox::default(), 
        });

        map.insert_to_event_map(KeyTargetSet::from([KeyTarget::JumpJustPressed]),
         EventFighterMovementNode { 
            base: FighterMovementNodeBase {
                movement: FighterMovement::Jumping,
                sprite_name: "JumpLoop".to_string(),
                state_update: |pos,vel,dt| {
                    pos.x += vel.x * dt;
                    pos.y += vel.y * dt;
                    pos.z += vel.z * dt;
                    vel.z += GRAVITY * dt;
                },
                state_enter: |_,vel| {vel.z = JUMPING_SPEED;},
            }, 
            player_can_enter: |floor_z,pos_z,_,_,_| floor_z == pos_z,
            player_can_exit: |floor_z,pos_z,_,movement_request| 
                {
                    if movement_request == &FighterMovement::JumpAttack {
                        return true
                    } else if pos_z == floor_z {
                        return true}
                    else {
                        return false};
                },
            channel: None,
            duration_and_fallback: None,
            hit_boxes: Vec::new(),
            hurt_boxes: Vec::new(),
         });

         map.insert_to_event_map(KeyTargetSet::from([KeyTarget::RightJustPressed]),
         EventFighterMovementNode { 
            base: FighterMovementNodeBase {
                movement: FighterMovement::RunningEast,
                sprite_name: "Running".to_string(),
                state_update: |pos,vel,dt| {
                    pos.x += vel.x * dt;
                    pos.y += vel.y * dt;
                },
                state_enter: |_,vel| {vel.x = RUNNING_SPEED;},
            }, 
            player_can_enter: |floor_z,pos_z, fighter_movement_stack,event_keytargetset_stack,is_joined_keytargetset| {
                if !is_joined_keytargetset {return false};
                let window_time = 0.3;

                let cond1 = pos_z == floor_z;
                if !cond1 {return false};
                
                //check for consecutive keypresses
                let mut pressed = 0;
                let mut elements = 0;
                for timed_keyset in event_keytargetset_stack.0.stack.iter().rev() {
                    if timed_keyset.duration > window_time || pressed > 1 {break};
                    if timed_keyset.value.contains(&KeyTarget::RightJustPressed) {pressed += 1};
                    elements += 1;
                }
                let cond2 = pressed > 1;
                if !cond2 {return false};

                //make sure last movement is idle/walking
                let mut cond3 = true;
                if let Some(last_movement) = fighter_movement_stack.0.stack.last() {
                    if last_movement.value != FighterMovement::Idle && last_movement.value != FighterMovement::WalkingEast {
                       cond3 = false;
                    }
                }
                if !cond3 {return false};

                //returning true!
                //remove acted upon events from stack
                for _ in 0..elements {
                    event_keytargetset_stack.0.pop();
                }
                true
                },
            player_can_exit: |_,_,_,movement_request| 
                {
                let unallowed_transitions = [
                    FighterMovement::WalkingEast,
                    FighterMovement::WalkingNorth,
                    FighterMovement::WalkingSouth,
                    FighterMovement::WalkingNorthEast,
                    FighterMovement::WalkingSouthEast,
                    FighterMovement::Idle,
                ];

                for movement in unallowed_transitions {
                    if movement_request == &movement {return false};
                }
                return true;
                },
            channel: Some(|full_keytargetset, vel| {
                    if KeyTargetSet::from([KeyTarget::Up]).is_subset(full_keytargetset) {
                        vel.y = WALKING_SPEED;
                    }
                    if KeyTargetSet::from([KeyTarget::Down]).is_subset(full_keytargetset) {
                        vel.y = -WALKING_SPEED;
                    }
                }),
            duration_and_fallback: None,
            hit_boxes: Vec::new(),
            hurt_boxes: Vec::new(),
         });

         map.insert_to_event_map(KeyTargetSet::from([KeyTarget::LeftJustPressed]),
         EventFighterMovementNode { 
            base: FighterMovementNodeBase {
                movement: FighterMovement::RunningWest,
                sprite_name: "Running".to_string(),
                state_update: |pos,vel,dt| {
                    pos.x += vel.x * dt;
                    pos.y += vel.y * dt;
                },
                state_enter: |_,vel| {vel.x = -RUNNING_SPEED;},
            }, 
            player_can_enter: |floor_z,pos_z, fighter_movement_stack,event_keytargetset_stack,is_joined_keytargetset| {
                if !is_joined_keytargetset {return false};
                let window_time = 0.3;

                let cond1 = pos_z == floor_z;
                if !cond1 {return false};
                
                //check for consecutive keypresses
                let mut pressed = 0;
                let mut elements = 0;
                for timed_keyset in event_keytargetset_stack.0.stack.iter().rev() {
                    if timed_keyset.duration > window_time || pressed > 1 {break};
                    if timed_keyset.value.contains(&KeyTarget::LeftJustPressed) {pressed += 1};
                    elements += 1;
                }
                let cond2 = pressed > 1;
                if !cond2 {return false};

                //make sure last movement is idle/walking
                let mut cond3 = true;
                if let Some(last_movement) = fighter_movement_stack.last() {
                    if last_movement.value != FighterMovement::Idle && last_movement.value != FighterMovement::WalkingWest {
                       cond3 = false;
                    }
                }
                if !cond3 {return false};

                //returning true!
                //remove acted upon events from stack
                for _ in 0..elements {
                    event_keytargetset_stack.0.pop();
                }
                true
                },
            player_can_exit: |_,_,_,movement_request| 
                {
                let unallowed_transitions = [
                    FighterMovement::WalkingWest,
                    FighterMovement::WalkingNorth,
                    FighterMovement::WalkingSouth,
                    FighterMovement::WalkingNorthWest,
                    FighterMovement::WalkingSouthWest,
                    FighterMovement::Idle,
                ];

                for movement in unallowed_transitions {
                    if movement_request == &movement {return false};
                }
                return true;
                },
            channel: Some(|full_keytargetset, vel| {
                    if KeyTargetSet::from([KeyTarget::Up]).is_subset(full_keytargetset) {
                        vel.y = WALKING_SPEED;
                    }
                    if KeyTargetSet::from([KeyTarget::Down]).is_subset(full_keytargetset) {
                        vel.y = -WALKING_SPEED;
                    }
                }),
            duration_and_fallback: None,
            hit_boxes: Vec::new(),
            hurt_boxes: Vec::new(),
         });

         map.insert_to_persistent_map(KeyTargetSet::from([KeyTarget::Defend]),
         PersistentFighterMovementNode { 
             base: FighterMovementNodeBase { 
                 movement: FighterMovement::Docking,
                 sprite_name: "Sliding".to_string(),
                 state_update: |_,_,_| {},
                 state_enter: |_,vel| {
                     vel.x = 0.0;
                     vel.y = 0.0;
                     vel.z = 0.0;
                     }, 
             },
             player_can_enter: |floor_z, position_z| floor_z == position_z,
             player_can_exit: |_,_,_,_| true,
             hit_box: HitBox::default(),
             hurt_box: HitBox::default(), 
         });

         map.insert_to_event_map(KeyTargetSet::from([KeyTarget::AttackJustPressed]),
         EventFighterMovementNode { 
            base: FighterMovementNodeBase {
                movement: FighterMovement::Slashing,
                sprite_name: "Slashing".to_string(),
                state_update: |_,_,_| {},
                state_enter: |_,vel| {
                    vel.x = 0.0;
                    vel.y = 0.0;
                },
            }, 
            player_can_enter: |floor_z,pos_z,_,_,joined_keytargetset| {
                if joined_keytargetset {return false};
                floor_z == pos_z 
            },
            player_can_exit: |_,_,prev_movement_duration ,_| {
                prev_movement_duration > 0.5
            },
            channel: None,
            duration_and_fallback: None,
            hit_boxes: Vec::new(),
            hurt_boxes: Vec::new(),
         });

         map.insert_to_event_map(KeyTargetSet::from([KeyTarget::AttackJustPressed]),
         EventFighterMovementNode { 
            base: FighterMovementNodeBase {
                movement: FighterMovement::JumpAttack,
                sprite_name: "AirSlashing".to_string(),
                state_update: |pos,vel,dt| {
                    pos.x += vel.x * dt;
                    pos.y += vel.y * dt;
                    pos.z += vel.z * dt;
                    vel.z += GRAVITY * dt;
                },
                state_enter: |_,_| {},
            }, 
            player_can_enter: |floor_z,pos_z,fighter_movement_stack,_,_| {
                if let Some(durative_movement) = fighter_movement_stack.last() {
                    if floor_z != pos_z && durative_movement.value == FighterMovement::Jumping{
                        return true
                    }
                }
                false
            },
            player_can_exit: |floor_z,pos_z,_ ,_| {
                floor_z == pos_z
            },
            channel: None,
            duration_and_fallback: Some(DurationAndFallback {
                duration: 0.5,
                fallback: FighterMovement::Jumping,
                apply_enter_state_fcn: false, 
            }),
            hit_boxes: Vec::new(),
            hurt_boxes: Vec::new(),
         });

        map
    }
}