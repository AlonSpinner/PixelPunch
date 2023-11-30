use bevy::prelude::*;

#[derive(Component)]
pub struct StatBarData {
    pub max_length: f32,
    pub thickness: f32,
    pub target_entity : Entity,
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
        target_entity: Entity,
    ) -> Self
    {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color : color,
                    flip_x : reverse,
                    flip_y : false,
                    rect : Some(Rect {min :  Vec2::new(0.0, 0.0), max : Vec2::new(max_length, thickness),}),
                    anchor : if reverse {bevy::sprite::Anchor::TopRight} else {bevy::sprite::Anchor::TopLeft},
                    ..default()
                },
                transform: if reverse {
                             Transform::from_translation(Vec3::new(displacement.x, displacement.y, z))
                            } else {Transform::from_translation(Vec3::new(displacement.x, displacement.y, z))},
                visibility : if hide {Visibility::Hidden} else {Visibility::Visible},
                ..default()
            },
            data : StatBarData {
                max_length : max_length,
                thickness : thickness,
                target_entity : target_entity,
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
        target_entity: Entity,
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
            target_entity);

        let empty = StatBarEmptyBundle::new(empty_color,
            reverse,
            max_length,
            thickness,
            z);

        (bar, empty)
    }
}

#[derive(Bundle)]
pub struct StatBarEmptyBundle
{
    sprite_bundle: SpriteBundle,
}

impl StatBarEmptyBundle {
    pub fn new(color : Color, bar_reverse : bool,  bar_max_length : f32, bar_thickness : f32, bar_z : f32) -> Self {
        let dz = -1.0;
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color : color,
                    flip_x : false,
                    flip_y : false,
                    rect : Some(Rect {min :  Vec2::new(0.0, 0.0),
                                    max : Vec2::new(bar_max_length, bar_thickness),}),
                    anchor : if bar_reverse {bevy::sprite::Anchor::TopRight} else {bevy::sprite::Anchor::TopLeft},
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, bar_z + dz)),
                ..default()
            },
    }
}
}

