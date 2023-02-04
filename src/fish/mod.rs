use bevy::prelude::*;

mod debug;
mod plugin;

pub use debug::*;
pub use plugin::*;

pub const FISH_COUNT: isize = 400;

#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct GroupPosition(pub Vec3);

#[derive(Resource, Default, Deref, DerefMut, Debug)]
pub struct GroupDirection(pub Vec3);

pub const MOST_CLOSEST_DISTANCE: f32 = 1.;

pub const GROUP_SCALE: f32 = 10.;

/// Separate
/// the force to keep distance other fish.
#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub struct SeparationFromOthersForce(pub Vec3);

/// the force to keep distance field wall
#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub struct SeparationFromWallForce(pub Vec3);

/// Alignment
/// the force to move to same direction with other fish.
#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub struct AlignmentForce(pub Vec3);

/// Cohesion
/// the force to get together with other bird
#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub struct CohesionForce(pub Vec3);

/// Compounded above three force.
#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub struct CompoundedForce(pub Vec3);

#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub struct Speed(pub f32);

/// most closest distance from other bird
#[derive(Component, PartialEq, Clone, Copy, Debug)]
pub struct ClosestDistance(pub f32);

#[derive(Component, Debug)]
pub struct Fish;
