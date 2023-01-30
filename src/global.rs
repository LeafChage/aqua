use crate::field;
use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

const CAMERA_POSITION: Vec3 = Vec3::new(
    field::FIELD_DISTANCE * 2.,
    field::FIELD_DISTANCE * 1.5,
    field::FIELD_DISTANCE * 1.5,
);

const HALF_SIZE: f32 = 10.0;
pub fn setup_world(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(CAMERA_POSITION).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Camera);
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_translation(CAMERA_POSITION).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
