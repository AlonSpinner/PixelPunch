use super::controls::*;
use super::datatypes::*;

use bevy::prelude::*;
use strum_macros::Display;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Display)]
pub enum Fighter{
    IDF,
    HAMAS,
}

#[derive(Component)]
pub struct FighterHealth{
    pub current: f32,
    pub max: f32,
}

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

    pub fn last(&self) -> Option<&TimeTaggedValue<FighterMovement>> {
        self.0.stack.last()
    }

    pub fn push(&mut self, value : FighterMovement) {
        self.0.push(value);
    }
}

#[derive(Bundle)]
pub struct FighterBundle{
    pub fighter: Fighter,
    pub health: FighterHealth,
    pub position: FighterPosition,
    pub velocity: FighterVelocity,
    pub movement_stack : FighterMovementStack,
    pub event_keytargetset_stack : KeyTargetSetStack,
    pub sprite: SpriteSheetBundle,
}

#[derive(Component)]
pub enum Player{
    Player1,
    Player2,
}

#[derive(Bundle)]
pub struct ControlledFighterBundle{
    pub fighter_bundle : FighterBundle,
    pub player: Player,
    pub controls: PlayerControls,
}