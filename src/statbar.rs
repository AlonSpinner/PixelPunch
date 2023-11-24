use bevy::prelude::*;

#[derive(Bundle)]
pub struct StatBar
{
    sprite_bundle: SpriteBundle,
    target_id: PlayerBundle,
}

impl StatBar
{
    pub fn new(
        color: Color,
        empty_color : Color,
        length: f32,
        thickness: f32,
        displacement: Vec2,
        reverse: bool,
        hide: bool,
        value : f32,
        target_id: Entity,
    ) -> Self
    {
        // let fixed_x_displacement: f32;
        // if reverse {fixed_x_displacement = displacement.x + length;
        // } else {fixed_x_displacement = displacement.x;}
        
        Self {
            // sprite_bundle: SpriteBundle {
            //     sprite: Sprite {
            //         color : Color::rgb(1.0,0.0,0.0),
            //         flip_x : reverse,
            //         flip_y : false,
            //         // rect : Some(Rect {min :  Vec2::new(0.0, 0.0), max : Vec2::new(length * value, thickness),}),

            //         anchor : bevy::sprite::Anchor::Center,
            //         custom_size: Some(Vec2::new(50.0, 1000.0)),
            //         ..default()
            //     },
            //     transform: Transform::from_translation(Vec3::new(displacement.x, displacement.y, 0.0)),
            //     // visibility: if hide {Visibility::Hidden} else {Visibility::Visible},
            //     ..default()
            // },

            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color : Color::rgb(1.0,0.0,0.0),
                    flip_x : false,
                    flip_y : false,
                    // rect : Some(Rect {min :  Vec2::new(0.0, 0.0), max : Vec2::new(length * value, thickness),}),
        
                    anchor : bevy::sprite::Anchor::Center,
                    custom_size: Some(Vec2::new(50.0, 1000.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                // visibility: if hide {Visibility::Hidden} else {Visibility::Visible},
                ..default()
            },
            target_id,
        }
    }

    pub fn update_value(&mut self, value: f32)
    {
        if value < 0.0 || value > 1.0 {
            panic!("Statbar value is {}, but must be between 0.0 and 1.0", value)
        }

        // self.sprite_bundle.sprite.rect = Some(Rect {
        //      min :  Vec2::new(0.0, 0.0),
        //      max : Vec2::new(self.length * self.value, self.thickness),
        // });
    }

    pub fn update_hidden(&mut self, hide: bool)
    {
        self.sprite_bundle.visibility = if hide {Visibility::Hidden} else {Visibility::Visible};
    }
}