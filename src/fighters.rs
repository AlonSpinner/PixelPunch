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

pub struct HitBox {
    pub center : [f32;2], //x,y : center of hitbox
    pub theta : f32, //rotation of hitbox (around axis outside the screen)
    pub dx : f32, //width of hitbox
    pub dy : f32, //height of hitbox
    pub dz : f32, //depth of hitbox
}

impl Default for HitBox {
    fn default() -> Self {
        Self{
            center : [0.0,0.0],
            theta : 0.0,
            dx : 0.0,
            dy : 0.0,
            dz : 0.0,
        }
    }
}

#[derive(Component)]
pub struct FighterHitBox {
    pub hitbox : HitBox,
    pub damage : f32,
    pub knockback : f32,
    pub stun : f32,
}

impl Default for FighterHitBox {
    fn default() -> Self {
        Self{
            hitbox : HitBox::default(),
            damage : 0.0,
            knockback : 0.0,
            stun : 0.0,
        }
    }
}

#[derive(Component)]
pub struct FighterHurtBox {
    pub hitbox : HitBox,
    pub armor : bool,
    pub invunerable : bool,
}

impl Default for FighterHurtBox {
    fn default() -> Self {
        Self{
            hitbox : HitBox::default(),
            armor : false,
            invunerable : false,
        }
    }
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
    pub hitbox: FighterHitBox,
    pub hurtbox: FighterHurtBox,
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