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

#[derive(PartialEq, Clone)]
pub struct KeyControl{
    pub keycode : KeyCode,
    pub keytarget : KeyTarget, 
    pub last_released : f32
}

#[derive(Component)]
pub struct PlayerControls{
    up : KeyControl,
    down : KeyControl,
    left : KeyControl,
    right : KeyControl,
    attack : KeyControl,
    defend : KeyControl,
}

impl Default for PlayerControls {
    fn default() -> Self {
        Self{
            up : KeyControl{keycode: KeyCode::W, keytarget: KeyTarget::Up, last_released: 0.0},
            down : KeyControl{keycode: KeyCode::S, keytarget: KeyTarget::Down, last_released: 0.0},
            left : KeyControl{keycode: KeyCode::A, keytarget: KeyTarget::Left, last_released: 0.0},
            right : KeyControl{keycode: KeyCode::D, keytarget: KeyTarget::Right, last_released: 0.0},
            attack: KeyControl{keycode: KeyCode::F, keytarget: KeyTarget::Attack, last_released: 0.0},
            defend: KeyControl{keycode: KeyCode::G, keytarget: KeyTarget::Defend, last_released: 0.0},
        }
    }
}