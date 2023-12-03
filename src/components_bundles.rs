use super::utils::*;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use strum_macros::Display;
use std::collections::BTreeSet;
use std::ops::Add;
use std::fmt::Display;
use std::fmt;


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
pub struct FacingEast(pub bool);

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
    pub facing_east: FacingEast,
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

#[derive(Component)]
pub struct ShadowData {
    pub target_entity : Entity,
    pub height_offset : f32,
    pub z : f32,
}

#[derive(Bundle)]
pub struct ShadowBundle{
    shape_bundle : ShapeBundle,
    fill : Fill,
    stroke : Stroke,
    shadow : ShadowData,
}   

impl ShadowBundle {
    pub fn new(
        radii : Vec2,
        z : f32,
        hide: bool,
        fill_color : Color,
        stroke_color : Color,
        stroke_width : f32,
        target_entity : Entity,
        height_offset : f32,
    ) -> Self
    {
        Self {
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(
                    &shapes::Ellipse{
                        radii : radii,
                        center : Vec2::new(0.0, 0.0),
                    }),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, z)),
                visibility : if hide {Visibility::Hidden} else {Visibility::Visible},
                ..default()
                },
            fill : Fill::color(fill_color),
            stroke : Stroke::new(stroke_color, stroke_width),
            shadow : ShadowData {
                target_entity : target_entity,
                height_offset : height_offset,
                z : z,
            }
        }
    }
}

#[derive(Component)]
pub struct StatBarData {
    pub max_length: f32,
    pub thickness: f32,
    pub target_entity : Entity,
}

#[derive(Bundle)]
pub struct StatBarBundle
{
    sprite_bundle: SpriteBundle,
    data : StatBarData,
}

impl StatBarBundle
{
    pub fn new(
        color: Color,
        max_length: f32,
        thickness: f32,
        displacement: Vec2,
        reverse: bool,
        hide: bool,
        z : f32,
        target_entity: Entity,
    ) -> Self
    {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color : color,
                    flip_x : reverse,
                    flip_y : false,
                    rect : Some(Rect {min :  Vec2::new(0.0, 0.0), max : Vec2::new(max_length, thickness),}),
                    anchor : if reverse {bevy::sprite::Anchor::TopRight} else {bevy::sprite::Anchor::TopLeft},
                    ..default()
                },
                transform: if reverse {
                             Transform::from_translation(Vec3::new(displacement.x, displacement.y, z))
                            } else {Transform::from_translation(Vec3::new(displacement.x, displacement.y, z))},
                visibility : if hide {Visibility::Hidden} else {Visibility::Visible},
                ..default()
            },
            data : StatBarData {
                max_length : max_length,
                thickness : thickness,
                target_entity : target_entity,
            }
        }
    }

    pub fn new_with_emptycolor(
        bar_color: Color,
        empty_color : Color,
        max_length: f32,
        thickness: f32,
        displacement: Vec2,
        reverse: bool,
        hide: bool,
        target_entity: Entity,
        z : f32,
    ) -> (Self, StatBarEmptyBundle)
    {
        let bar = Self::new(bar_color,
            max_length,
            thickness,
            displacement,
            reverse,
            hide,
            z,
            target_entity);

        let empty = StatBarEmptyBundle::new(empty_color,
            reverse,
            max_length,
            thickness,
            z);

        (bar, empty)
    }
}

#[derive(Bundle)]
pub struct StatBarEmptyBundle
{
    sprite_bundle: SpriteBundle,
}

impl StatBarEmptyBundle {
    pub fn new(color : Color, bar_reverse : bool,  bar_max_length : f32, bar_thickness : f32, bar_z : f32) -> Self {
        let dz = -1.0;
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color : color,
                    flip_x : false,
                    flip_y : false,
                    rect : Some(Rect {min :  Vec2::new(0.0, 0.0),
                                    max : Vec2::new(bar_max_length, bar_thickness),}),
                    anchor : if bar_reverse {bevy::sprite::Anchor::TopRight} else {bevy::sprite::Anchor::TopLeft},
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, bar_z + dz)),
                ..default()
            },
    }
}
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Ord, PartialOrd, Debug)]
pub enum KeyTarget{
    Up,
    UpJustPressed,
    Down,
    DownJustPressed,
    Left,
    LeftJustPressed,
    Right,
    RightJustPressed,
    Attack,
    AttackJustPressed,
    Jump,
    JumpJustPressed,
    Defend,
    DefendJustPressed,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct KeyTargetSet(BTreeSet<KeyTarget>);

impl Display for KeyTargetSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for key in &self.0 {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", key)?;
        }
        Ok(())
    }
}

impl KeyTargetSet {
    pub fn empty() -> Self {
        Self(BTreeSet::new())
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.is_subset(&other.0)
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        self.0.is_superset(&other.0)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        !self.0.is_disjoint(&other.0)
    }

    pub fn contains(&self, other: &KeyTarget) -> bool {
        self.0.contains(other)
    }
}

impl<const N: usize> From<[KeyTarget; N]> for KeyTargetSet {
    fn from(array: [KeyTarget; N]) -> Self {
        Self(array.iter().cloned().collect())
    }

}

impl Add for KeyTargetSet {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_set = self.0;
        new_set.extend(rhs.0);
        Self(new_set)
    }
}

impl Add <KeyTarget> for KeyTargetSet {
    type Output = Self;

    fn add(self, rhs: KeyTarget) -> Self::Output {
        let mut new_set = self.0;
        new_set.insert(rhs);
        Self(new_set)
    }
}

#[derive(Component)]
pub struct KeyTargetSetStack(pub DurativeStack<KeyTargetSet>);

impl KeyTargetSetStack{
    pub fn new(max_size : usize, max_duration : f32) -> Self {
        Self(DurativeStack::new(max_size, max_duration))
    }

    pub fn join(&self) -> KeyTargetSet {
        let mut joined = KeyTargetSet::empty();
        for time_tagged_keytargetset in self.0.stack.iter() {
            joined = joined + time_tagged_keytargetset.value.clone();
        }
        joined
    }
}

#[allow(dead_code)]
#[derive(Component)]
pub struct PlayerControls{
    pub up : KeyCode,
    pub down : KeyCode,
    pub left : KeyCode,
    pub right : KeyCode,
    pub attack : KeyCode,
    pub jump : KeyCode,
    pub defend : KeyCode,
}

impl Default for PlayerControls {
    fn default() -> Self {
        Self{
            up : KeyCode::W,
            down : KeyCode::S,
            left : KeyCode::A,
            right : KeyCode::D,
            attack: KeyCode::G,
            jump: KeyCode::H,
            defend: KeyCode::J,
        }
    }
}

impl PlayerControls {
    pub fn into_persistent_keytargetset(&self, keyboard_input : &Input<KeyCode>) -> KeyTargetSet {
        let mut pressed_keys = KeyTargetSet::empty();
        if keyboard_input.pressed(self.up) {
            pressed_keys = pressed_keys + KeyTarget::Up;
        }
        if keyboard_input.pressed(self.down) {
            pressed_keys = pressed_keys + KeyTarget::Down;
        }
        if keyboard_input.pressed(self.left) {
            pressed_keys = pressed_keys + KeyTarget::Left;
        }
        if keyboard_input.pressed(self.right) {
            pressed_keys = pressed_keys + KeyTarget::Right;
        }
        if keyboard_input.pressed(self.attack) {
            pressed_keys = pressed_keys + KeyTarget::Attack;
        }
        if keyboard_input.pressed(self.jump) {
            pressed_keys = pressed_keys + KeyTarget::Jump;
        }
        if keyboard_input.pressed(self.defend) {
            pressed_keys = pressed_keys + KeyTarget::Defend;
        }
        pressed_keys
    }

    pub fn into_event_keytargetset(&self, keyboard_input : &Input<KeyCode>) -> KeyTargetSet {
        let mut just_pressed_keys = KeyTargetSet::empty();
        if keyboard_input.just_pressed(self.up) {
            just_pressed_keys = just_pressed_keys + KeyTarget::UpJustPressed;
        }
        if keyboard_input.just_pressed(self.down) {
            just_pressed_keys = just_pressed_keys + KeyTarget::DownJustPressed;
        }
        if keyboard_input.just_pressed(self.left) {
            just_pressed_keys = just_pressed_keys + KeyTarget::LeftJustPressed;
        }
        if keyboard_input.just_pressed(self.right) {
            just_pressed_keys = just_pressed_keys + KeyTarget::RightJustPressed;
        }
        if keyboard_input.just_pressed(self.attack) {
            just_pressed_keys = just_pressed_keys + KeyTarget::AttackJustPressed;
        }
        if keyboard_input.just_pressed(self.jump) {
            just_pressed_keys = just_pressed_keys + KeyTarget::JumpJustPressed;
        }
        if keyboard_input.just_pressed(self.defend) {
            just_pressed_keys = just_pressed_keys + KeyTarget::DefendJustPressed;
        }
        just_pressed_keys
    }

    pub fn into_full_keytargetset(&self, keyboard_input : &Input<KeyCode>) -> KeyTargetSet {
        self.into_persistent_keytargetset(keyboard_input) + self.into_event_keytargetset(keyboard_input)
    }
}