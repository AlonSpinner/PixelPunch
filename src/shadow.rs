use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
pub struct ShadowData {
    pub target_entity : Entity,
    pub height_offset : f32,
    pub z : f32,
}

#[derive(Bundle)]
pub struct ShadowBundle{
    shape_bundle : ShapeBundle,
    fill : Fill,
    stroke : Stroke,
    shadow : ShadowData,
}   

impl ShadowBundle {
    pub fn new(
        radii : Vec2,
        z : f32,
        hide: bool,
        fill_color : Color,
        stroke_color : Color,
        stroke_width : f32,
        target_entity : Entity,
        height_offset : f32,
    ) -> Self
    {
        Self {
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(
                    &shapes::Ellipse{
                        radii : radii,
                        center : Vec2::new(0.0, 0.0),
                    }),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, z)),
                visibility : if hide {Visibility::Hidden} else {Visibility::Visible},
                ..default()
                },
            fill : Fill::color(fill_color),
            stroke : Stroke::new(stroke_color, stroke_width),
            shadow : ShadowData {
                target_entity : target_entity,
                height_offset : height_offset,
                z : z,
            }
        }
    }
}
