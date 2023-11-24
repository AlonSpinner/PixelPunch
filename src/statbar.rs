use bevy::prelude::*;

#[derive(Component)]
pub struct StatBarData {
    max_length: f32,
    thickness: f32,
    displacement: Vec2,
    reverse: bool,
    hide: bool,
    entity : Entity,
    z : f32,
}

#[derive(Bundle)]
pub struct StatBarBundle
{
    sprite_bundle: SpriteBundle,
    data : StatBarData,
}

impl StatBarBundle
{
    pub fn new(
        color: Color,
        // empty_color : Color,
        length: f32,
        thickness: f32,
        displacement: Vec2,
        reverse: bool,
        hide: bool,
        target_id: Entity,
        z : f32,
    ) -> Self
    {
        let fixed_x_displacement: f32;
        if reverse {fixed_x_displacement = displacement.x + length;
        } else {fixed_x_displacement = displacement.x;}

        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color : color,
                    flip_x : reverse,
                    flip_y : false,
                    rect : Some(Rect {min :  Vec2::new(0.0, 0.0), max : Vec2::new(length, thickness),}),
                    anchor : bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(fixed_x_displacement, displacement.y, z)),
                ..default()
            },
            data : StatBarData {
                max_length : length,
                thickness : thickness,
                displacement : displacement,
                reverse : reverse,
                hide : hide,
                entity : target_id,
                z : z,
            }
        }
    }

    pub fn set_value(&mut self, value: f32)
    {
        if value < 0.0 || value > 1.0 {
            panic!("Statbar value is {}, but must be between 0.0 and 1.0", value)
        }

        self.sprite_bundle.sprite.rect = Some(Rect {
             min :  Vec2::new(0.0, 0.0),
             max : Vec2::new(self.data.max_length * value, self.data.thickness),
        });
    }

    pub fn hide(&mut self, hide: bool)
    {
        self.data.hide = hide;
        self.sprite_bundle.visibility = if hide {Visibility::Hidden} else {Visibility::Visible};
    }
}