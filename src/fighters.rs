use bevy::prelude::*;
use strum_macros::{EnumString, Display};
use super::controls::PlayerControls;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Display)]
pub enum Fighter{
    IDF,
    HAMAS,
}
impl Fighter {
    pub fn movements(&self) -> Vec<FighterMovement> {
        match *self {
            Fighter::IDF => vec!(FighterMovement::Idle,
                                 FighterMovement::JumpLoop,
                                 FighterMovement::Docking,
                                 FighterMovement::Running,
                                 FighterMovement::Walking),
            Fighter::HAMAS => vec!(FighterMovement::Idle,
                                 FighterMovement::JumpLoop,
                                 FighterMovement::Docking,
                                 FighterMovement::Running,
                                 FighterMovement::Walking),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, EnumString, Display)]
pub enum FighterMovement {
    Idle,
    #[strum(serialize = "JumpLoop")]
    JumpLoop,
    #[strum(serialize = "Sliding")]
    Docking,
    Running,
    Walking,
}
impl FighterMovement {
    pub fn change_to(&mut self, new_movement: FighterMovement) {
        //only change if new movement is different to allow Bevy's change detection to work
        if &new_movement != self {
            *self = new_movement;
            info!("movement changed to {:?}", self)
        }
    }
}

struct FighterMovementEdge {
    to : FighterMovementNode,
    controls : PlayerControls,
    condition : fn() -> bool,
}

struct FighterMovementNode {
    movement : FighterMovement,
    edges : Vec<FighterMovementEdge>,
}

struct FighterMovementGraph {
    nodes : Vec<FighterMovementNode>,
    current_node : FighterMovementNode,
}

impl FighterMovementGraph {
    fn default() -> Self {
        let mut nodes = Vec::new();
        nodes.push(FighterMovementNode{
            movement : FighterMovement::Idle,
            edges : 
        });
        nodes.push(FighterMovementNode{
            movement : FighterMovement::JumpLoop,
            edges : Vec::new(),
        });
        nodes.push(FighterMovementNode{
            movement : FighterMovement::Docking,
            edges : Vec::new(),
        });
        nodes.push(FighterMovementNode{
            movement : FighterMovement::Running,
            edges : Vec::new(),
        });
        nodes.push(FighterMovementNode{
            movement : FighterMovement::Walking,
            edges : Vec::new(),
        });
        Self{
            nodes,
            current_node : nodes[0],
        }
    }
}

