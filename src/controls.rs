use bevy::prelude::*;
use std::collections::BTreeSet;
use std::ops::Add;

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
    Defend,
    DefendJustPressed,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct KeyTargetSet(BTreeSet<KeyTarget>);

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
pub struct KeyTargetSetStack{
    pub stack : Vec<(KeyTargetSet,f32)>,
    pub max_size : usize,
    pub max_duration : f32,
}

impl KeyTargetSetStack{
    pub fn new(max_size : usize, max_duration : f32) -> Self {
        Self{
            stack : Vec::with_capacity(max_size),
            max_size,
            max_duration,
        }
    }

    pub fn join(&self) -> KeyTargetSet {
        let mut joined = KeyTargetSet::empty();
        for (keytargetset, _) in self.stack.iter() {
            joined = joined + keytargetset.clone();
        }
        joined
    }
    
    pub fn push(&mut self, keytargetset : KeyTargetSet) {
        if self.stack.len() == self.max_size {
            self.stack.remove(0);
        }
        self.stack.push((keytargetset, 0.0));
    }

    pub fn update(&mut self, delta_time : f32) {
        //remove expired keytargetsets
        self.stack.retain(|(_, duration)| *duration < self.max_duration);
        //increment duration of all keytargetsets
        for (_, duration) in self.stack.iter_mut() {
            *duration += delta_time;
        }
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
            defend: KeyCode::H,
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
        if keyboard_input.just_pressed(self.defend) {
            just_pressed_keys = just_pressed_keys + KeyTarget::DefendJustPressed;
        }
        just_pressed_keys
    }

    pub fn into_full_keytargetset(&self, keyboard_input : &Input<KeyCode>) -> KeyTargetSet {
        self.into_persistent_keytargetset(keyboard_input) + self.into_event_keytargetset(keyboard_input)
    }
}