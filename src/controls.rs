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

pub struct PlayerKeyControl{
    pub keycode : KeyCode,
    pub last_released : f32
}

#[allow(dead_code)]
#[derive(Component)]
pub struct PlayerControls{
    pub up : PlayerKeyControl,
    pub down : PlayerKeyControl,
    pub left : PlayerKeyControl,
    pub right : PlayerKeyControl,
    pub attack : PlayerKeyControl,
    pub defend : PlayerKeyControl,
}

impl Default for PlayerControls {
    fn default() -> Self {
        Self{
            up : PlayerKeyControl{keycode: KeyCode::W, last_released: 0.0},
            down : PlayerKeyControl{keycode: KeyCode::S, last_released: 0.0},
            left : PlayerKeyControl{keycode: KeyCode::A, last_released: 0.0},
            right : PlayerKeyControl{keycode: KeyCode::D, last_released: 0.0},
            attack: PlayerKeyControl{keycode: KeyCode::F, last_released: 0.0},
            defend: PlayerKeyControl{keycode: KeyCode::G, last_released: 0.0},
        }
    }
}

impl PlayerControls {
    pub fn into_keytargetset(&self, keyboard_input : &Input<KeyCode>) -> KeyTargetSet {
        let mut keytargetset = KeyTargetSet::empty();
        if keyboard_input.pressed(self.up.keycode) {
            keytargetset = keytargetset + KeyTarget::Up;
        }
        if keyboard_input.pressed(self.down.keycode) {
            keytargetset = keytargetset + KeyTarget::Down;
        }
        if keyboard_input.pressed(self.left.keycode) {
            keytargetset = keytargetset + KeyTarget::Left;
        }
        if keyboard_input.pressed(self.right.keycode) {
            keytargetset = keytargetset + KeyTarget::Right;
        }
        if keyboard_input.pressed(self.attack.keycode) {
            keytargetset = keytargetset + KeyTarget::Attack;
        }
        if keyboard_input.pressed(self.defend.keycode) {
            keytargetset = keytargetset + KeyTarget::Defend;
        }
        keytargetset
    }
}