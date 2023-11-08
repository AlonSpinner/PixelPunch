use bevy::prelude::*;
use std::collections::BTreeSet;
use std::ops::Add;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Ord, PartialOrd, Debug)]
pub enum KeyTarget{
    Up,
    Down,
    Left,
    Right,
    Attack,
    Defend,
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
            attack: KeyCode::F,
            defend: KeyCode::G,
        }
    }
}

impl PlayerControls {
    pub fn into_keytargetset(&self, keyboard_input : &Input<KeyCode>) -> KeyTargetSet {
        let mut keytargetset = KeyTargetSet::empty();
        if keyboard_input.pressed(self.up) {
            keytargetset = keytargetset + KeyTarget::Up;
        }
        if keyboard_input.pressed(self.down) {
            keytargetset = keytargetset + KeyTarget::Down;
        }
        if keyboard_input.pressed(self.left) {
            keytargetset = keytargetset + KeyTarget::Left;
        }
        if keyboard_input.pressed(self.right) {
            keytargetset = keytargetset + KeyTarget::Right;
        }
        if keyboard_input.pressed(self.attack) {
            keytargetset = keytargetset + KeyTarget::Attack;
        }
        if keyboard_input.pressed(self.defend) {
            keytargetset = keytargetset + KeyTarget::Defend;
        }
        keytargetset
    }
}