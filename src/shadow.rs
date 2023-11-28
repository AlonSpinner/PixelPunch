use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
pub struct ShadowData {
    pub target_entity : Entity,
}

#[derive(Bundle)]
pub struct ShadowBundle{
    shape_bundle : ShapeBundle,
    data : ShadowData,
}

impl ShadowBundle {
    pub fn new(
        color: Color,
        reverse: bool,
        hide: bool,
        z : f32,
        target_entity: Entity,
    ) -> Self
    {
        Self {
            sprite_bundle: ShapeBundle {
                shape: Shape {
                    color : color,
                    fill_type : FillType::Solid,
                    ..default()
                },
                transform: if reverse {
                             Transform::from_translation(Vec3::new(displacement.x, displacement.y, z))
                            } else {Transform::from_translation(Vec3::new(displacement.x, displacement.y, z))},
                visibility : if hide {Visibility::Hidden} else {Visibility::Visible},
                ..default()
            },
            data : ShadowData {
                target_entity : target_entity,
            }
        }
    }
}
