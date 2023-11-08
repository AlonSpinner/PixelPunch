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
    pub keytarget : KeyTarget, 
    pub last_released : f32
}

#[allow(dead_code)]
#[derive(Component)]
pub struct PlayerControls{
    up : PlayerKeyControl,
    down : PlayerKeyControl,
    left : PlayerKeyControl,
    right : PlayerKeyControl,
    attack : PlayerKeyControl,
    defend : PlayerKeyControl,
}

impl Default for PlayerControls {
    fn default() -> Self {
        Self{
            up : PlayerKeyControl{keycode: KeyCode::W, keytarget: KeyTarget::Up, last_released: 0.0},
            down : PlayerKeyControl{keycode: KeyCode::S, keytarget: KeyTarget::Down, last_released: 0.0},
            left : PlayerKeyControl{keycode: KeyCode::A, keytarget: KeyTarget::Left, last_released: 0.0},
            right : PlayerKeyControl{keycode: KeyCode::D, keytarget: KeyTarget::Right, last_released: 0.0},
            attack: PlayerKeyControl{keycode: KeyCode::F, keytarget: KeyTarget::Attack, last_released: 0.0},
            defend: PlayerKeyControl{keycode: KeyCode::G, keytarget: KeyTarget::Defend, last_released: 0.0},
        }
    }
}

impl PlayerControls {
    pub fn into_keytargetset(&self, keyboard_input : &Input<KeyCode>) -> KeyTargetSet {
        let mut keytargetset = KeyTargetSet::empty();
        if keyboard_input.pressed(self.up.keycode) {
            keytargetset = keytargetset + self.up.keytarget;
        }
        if keyboard_input.pressed(self.down.keycode) {
            keytargetset = keytargetset + self.down.keytarget;
        }
        if keyboard_input.pressed(self.left.keycode) {
            keytargetset = keytargetset + self.left.keytarget;
        }
        if keyboard_input.pressed(self.right.keycode) {
            keytargetset = keytargetset + self.right.keytarget;
        }
        if keyboard_input.pressed(self.attack.keycode) {
            keytargetset = keytargetset + self.attack.keytarget;
        }
        if keyboard_input.pressed(self.defend.keycode) {
            keytargetset = keytargetset + self.defend.keytarget;
        }
        keytargetset
    }
}