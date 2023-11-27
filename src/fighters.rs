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

#[derive(Component)]
pub struct LookingRight(pub bool);

pub struct HitBox {
    pub center : [f32;2], //x,y : center of hitbox
    pub theta : f32, //rotation of hitbox (around axis outside the screen)
    pub dx : f32, //width of hitbox
    pub dy : f32, //height of hitbox
    pub dz : f32, //depth of hitbox
}

impl HitBox {
    pub fn bubble_intersection(self : &Self, other : &Self) -> bool {
        let r1 = ((self.dx/2.0).powi(2) + (self.dy/2.0).powi(2)).sqrt();
        let r2 = ((other.dx/2.0).powi(2) + (other.dy/2.0).powi(2)).sqrt();

        let d = ((self.center[0] - other.center[0]).powi(2) +
                         (self.center[1] - other.center[1]).powi(2)).sqrt();
        
        if d < r1 + r2 {
            return true
        } else {
            return false
        }
    }

    pub fn intersection(self : &Self, other : &Self) -> bool {
        //consider 2D intersection: transform other into self's coordinate system,
        // then check if any of the corners of other are inside self. We use the SE(2) representation
        //self_SE(2)_other = self_SE(2)_world * world_SE(2)_other
        //that is:  T = T1_inv * T2
        let mut xy_overlap = false;
        
        let (s1,c1) = self.theta.sin_cos();
        let r1_inv = Mat2::from_cols(
            Vec2::new(c1, s1),
            Vec2::new(-s1, c1),
        );
        let t1 = Vec2::new(self.center[0], self.center[1]);
        let t1_inv = -r1_inv * t1;

        let (s2,c2) = other.theta.sin_cos();
        let r2 = Mat2::from_cols(
            Vec2::new(c2, s2),
            Vec2::new(-s2, c2),
        );
        let t2 = Vec2::new(other.center[0], other.center[1]);

        let r = r1_inv * r2;
        let t = r1_inv * t2 + t1_inv;

        let p1 = Vec2::new(other.dx/2.0, other.dy/2.0);
        let p2 = Vec2::new(other.dx/2.0, -other.dy/2.0);
        let p3 = Vec2::new(-other.dx/2.0, other.dy/2.0);
        let p4 = Vec2::new(-other.dx/2.0, -other.dy/2.0);
        for p in [p1,p2,p3,p4].into_iter() {
            let p = r * p + t;
            let in_x = p.x > -self.dx/2.0 && p.x < self.dx/2.0;
            let in_y = p.y > -self.dy/2.0 && p.y < self.dy/2.0;
            if in_x && in_y {
                xy_overlap = true
            }
        }

        //now check z
        if xy_overlap {
            let z_overlap = other.dz/2.0 + self.dz/2.0 > (other.center[1] - self.center[1]).abs();
            if z_overlap {
                return true
            }
        }
        false
    }
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
    pub looking_right: LookingRight,
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