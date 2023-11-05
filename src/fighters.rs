use bevy::prelude::*;
use strum_macros::{EnumString, Display};
use super::controls::{KeyControl,KeyTarget};
use std::collections::{HashMap,HashSet};
use std::ops::Add;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Display)]
pub enum Fighter{
    IDF,
    HAMAS,
}

pub const IDF_MOVEMENT_GRAPH : FighterMovementGraph = FighterMovementGraph::default();
pub const HAMAS_MOVEMENT_GRAPH : FighterMovementGraph = FighterMovementGraph::default();

#[derive(Component)]
pub struct FighterHealth(f32);
#[derive(Component)]
pub struct FighterPosition {
    x : f32,
    y : f32,
}
#[derive(Component)]
pub struct FighterVelocity {
    x : f32,
    y : f32,
}

//All possible movements for a fighter
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum FighterMovement {
    Idle,
    JumpLoop,
    #[strum(serialize = "Sliding")]
    Docking,
    Running,
    Walking,
}
impl FighterMovement {
    //only change if new movement is different to allow Bevy's change detection to work
    pub fn change_to(&mut self, new_movement: FighterMovement) {
        if &new_movement != self {
            *self = new_movement;
            info!("movement changed to {:?}", self)
        }
    }
}

#[derive(Clone)]
struct FighterMovementNodeTransition {
    enter_controls : HashSet<KeyControl>, //combination of controls pressed to enter this node
    enter_condition : fn(position : FighterPosition) -> bool,
    leave_condition : fn(movement_duration : f32) -> bool,
}
impl Default for FighterMovementNodeTransition {
    fn default() -> Self {
        Self{
            enter_controls: HashSet::new(),
            enter_condition: |position| position.y == 0.0,
            leave_condition: |movement_duration| true,
        }
    }
}
impl FighterMovementNodeTransition {
    fn can_enter(&self, controls : &HashSet<KeyControl>) -> bool {
        self.enter_controls.is_subset(controls)
    }
    fn can_leave(&self, movement_duration : f32) -> bool {
        (self.leave_condition)(movement_duration)
    }
}

#[derive(Clone)]
struct FighterMovementGraph {
    nodes : HashMap<FighterMovement,FighterMovementNodeTransition>,
    current_node : FighterMovement,
}

impl FighterMovementGraph {
    fn default() -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(FighterMovement::Idle, FighterMovementNodeTransition::default());
        nodes.insert(FighterMovement::JumpLoop, FighterMovementNodeTransition{
            enter_controls : HashSet::from([KeyControl{keytarget: KeyTarget::Up, tapped_amount: 1}]),
            ..default()});
        nodes.insert(FighterMovement::Docking, FighterMovementNodeTransition{
            enter_controls : HashSet::from([KeyControl{keytarget: KeyTarget::Down, tapped_amount: 1}]),
            ..default()});
        nodes.insert(FighterMovement::Walking, FighterMovementNodeTransition{
            enter_controls : HashSet::from([KeyControl{keytarget: KeyTarget::Left, tapped_amount: 1},
                                            KeyControl{keytarget: KeyTarget::Right, tapped_amount: 1}]),
            ..default()});
        nodes.insert(FighterMovement::Running, FighterMovementNodeTransition{
            enter_controls : HashSet::from([KeyControl{keytarget: KeyTarget::Left, tapped_amount: 2},
                                            KeyControl{keytarget: KeyTarget::Right, tapped_amount: 2}]),
            ..default()});
        Self{
            nodes,
            current_node : FighterMovement::Idle,
        }
    }
    fn movements(&self) -> Vec<FighterMovement> {
        self.nodes.keys().map(|k| *k).collect()
    }
}

impl Add for FighterMovementGraph {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut nodes = self.nodes;
        nodes.extend(other.nodes);
        Self{
            nodes,
            current_node : self.current_node,
        }
    }
}

