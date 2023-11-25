use bevy::prelude::*;

#[derive(Component)]
struct StatBarData {
    max_length: f32,
    thickness: f32,
    target_id : Entity,

}

#[derive(Bundle)]
pub struct StatBarBundle
{
    sprite_bundle: SpriteBundle,
    pub data : StatBarData,
}

impl StatBarBundle
{
    pub fn new(
        color: Color,
        max_length: f32,
        thickness: f32,
        displacement: Vec2,
        reverse: bool,
        hide: bool,
        z : f32,
        target_id: Entity,
    ) -> Self
    {
        let fixed_x_displacement: f32;
        if reverse {fixed_x_displacement = displacement.x + max_length;
        } else {fixed_x_displacement = displacement.x;}

        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color : color,
                    flip_x : reverse,
                    flip_y : false,
                    rect : Some(Rect {min :  Vec2::new(0.0, 0.0), max : Vec2::new(max_length, thickness),}),
                    anchor : bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(fixed_x_displacement, displacement.y, z)),
                visibility : if hide {Visibility::Hidden} else {Visibility::Visible},
                ..default()
            },
            data : StatBarData {
                max_length : max_length,
                thickness : thickness,
                target_id : target_id,
            }
        }
    }

    pub fn new_with_emptycolor(
        bar_color: Color,
        empty_color : Color,
        max_length: f32,
        thickness: f32,
        displacement: Vec2,
        reverse: bool,
        hide: bool,
        target_id: Entity,
        z : f32,
    ) -> (Self, StatBarEmptyBundle)
    {
        let bar = Self::new(bar_color,
            max_length,
            thickness,
            displacement,
            reverse,
            hide,
            z,
            target_id);

        let empty = StatBarEmptyBundle::new(empty_color,
            max_length,
            thickness,
            z);

        (bar, empty)
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
        self.sprite_bundle.visibility = if hide {Visibility::Hidden} else {Visibility::Visible};
    }
}

#[derive(Bundle)]
pub struct StatBarEmptyBundle
{
    sprite_bundle: SpriteBundle,
}

impl StatBarEmptyBundle {
    pub fn new(color : Color, bar_max_length : f32, bar_thickness : f32, bar_z : f32) -> Self {
        let dz = -1.0;
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color : color,
                    flip_x : false,
                    flip_y : false,
                    rect : Some(Rect {min :  Vec2::new(0.0, 0.0),
                                    max : Vec2::new(bar_max_length, bar_thickness),}),
                    anchor : bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, bar_z + dz)),
                ..default()
            },
    }
}

    pub fn hide(&mut self, hide: bool)
    {
        self.sprite_bundle.visibility = if hide {Visibility::Hidden} else {Visibility::Visible};
    }
}

