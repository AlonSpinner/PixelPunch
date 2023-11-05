use bevy::prelude::*;

pub struct PlayerKeyControl{key : KeyCode, last_press : f64}

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
            up : PlayerKeyControl{key: KeyCode::W, last_press: 0.0},
            down : PlayerKeyControl{key: KeyCode::S, last_press: 0.0},
            left : PlayerKeyControl{key: KeyCode::A, last_press: 0.0},
            right : PlayerKeyControl{key: KeyCode::D, last_press: 0.0},
            attack: PlayerKeyControl{key: KeyCode::F, last_press: 0.0},
            defend: PlayerKeyControl{key: KeyCode::G, last_press: 0.0},
        }
    }
}