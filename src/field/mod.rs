use bevy::prelude::*;
use bevy::render::primitives::Plane;

mod field;
mod plugin;

pub use field::Field;
pub use plugin::FieldPlugin;

#[derive(Component)]
pub struct Wall;

pub const FIELD_DISTANCE: f32 = 5.0;
