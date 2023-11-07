use bevy::prelude::*;
use std::collections::BTreeSet;
use std::ops::Add;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub enum KeyTarget{
    Up,
    Down,
    Left,
    Right,
    Attack,
    Defend,
}
#[derive(Hash, Eq, PartialEq, Clone)]
pub struct KeyTargetSet(BTreeSet<KeyTarget>);

impl KeyTargetSet {
    pub fn empty() -> Self {
        Self(BTreeSet::new())
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