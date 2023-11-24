use bevy::prelude::*;
use crate::fighters::*;
use crate::controls::*;

#[derive(Bundle)]
pub struct FighterBundle{
    player: Player,
    fighter: Fighter,
    health: FighterHealth,
    position: FighterPosition,
    velocity: FighterVelocity,
    movement_stack : FighterMovementStack,
    event_keytargetset_stack : KeyTargetSetStack,
    sprite: SpriteSheetBundle,
}

impl Default for FighterBundle {
    fn default() -> Self {
        let mut movement_stack = FighterMovementStack::new(10);
        movement_stack.0.push(FighterMovement::InAir);

        Self{
            player: Player::Player1,
            fighter: Fighter::IDF,
            health : FighterHealth{current : 100.0, max : 100.0},
            position : FighterPosition{x : 0.0, y :0.0, z : 0.0},
            velocity : FighterVelocity{x : 0.0, y :0.0, z :0.0},
            movement_stack : movement_stack,
            event_keytargetset_stack : KeyTargetSetStack::new(10, 0.5),
            sprite : SpriteSheetBundle::default(),
        }
    }
}


#[derive(Component)]
pub enum Player{
    Player1,
    Player2,
}

#[derive(Bundle)]
pub struct ControlledFighterBundle{
    pub fighter_bundle : FighterBundle,
    pub controls: PlayerControls,
}

impl Default for ControlledFighterBundle {
    fn default() -> Self {
        Self{
            fighter_bundle : FighterBundle::default(),
            controls : PlayerControls::default(),
        }
    }
}