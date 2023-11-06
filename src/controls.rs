use bevy::prelude::*;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum KeyTarget{
    Up,
    Down,
    Left,
    Right,
    Attack,
    Defend,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct KeyControl{
    pub keytarget : KeyTarget,
    pub tapped_amount : usize,
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