use bevy::prelude::*;

#[derive(Component)]
pub struct StatBarData {
    max_length: f32,
    thickness: f32,
    hide: bool,
    entity : Entity,
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
        max_length: f32,
        thickness: f32,
        displacement: Vec2,
        reverse: bool,
        hide: bool,
        z : f32,
        target_id: Entity,
        border_id: Option<Entity>,
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
                ..default()
            },
            data : StatBarData {
                max_length : max_length,
                thickness : thickness,
                hide : hide,
                entity : target_id,
            }
        }
    }

    pub fn new_with_border(
        bar_color: Color,
        border_color : Color,
        max_length: f32,
        thickness: f32,
        displacement: Vec2,
        reverse: bool,
        hide: bool,
        target_id: Entity,
        z : f32,
        border_size : f32,
    ) -> (Self, StatBarBorderBundle)
    {

        let border = StatBarBorderBundle::new(border_color,
            max_length,
           thickness,
           z,
           border_size);

        let bar = Self::new(bar_color,
            max_length,
            thickness,
            displacement,
            reverse,
            hide,
            z,
            target_id,
            None);

        (bar, border)
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

#[derive(Bundle)]
pub struct StatBarBorderBundle
{
    sprite_bundle: SpriteBundle,
}

impl StatBarBorderBundle {
    pub fn new(color : Color, bar_max_length : f32, bar_thickness : f32, bar_z : f32, border_size : f32) -> Self {
        let dz = 1.0;
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color : color,
                    flip_x : false,
                    flip_y : false,
                    rect : Some(Rect {min :  Vec2::new(-border_size,
                                                         -border_size),
                                    max : Vec2::new(bar_max_length + border_size,
                                         bar_thickness + border_size),}),
                    anchor : bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, bar_z - dz)),
                ..default()
            },
    }
}
}

