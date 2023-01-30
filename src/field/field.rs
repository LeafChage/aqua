use crate::math;
use bevy::prelude::*;
use bevy::render::primitives::Plane;

#[derive(Resource, Clone, Debug, Deref, DerefMut)]
pub struct Field {
    pub planes: Vec<Plane>,
}

impl Field {
    pub fn new(planes: Vec<Plane>) -> Self {
        Field { planes }
    }

    pub fn nearest_plane(&self, pos: Vec3) -> (f32, &Plane) {
        let mut nearest_plane = None;
        let mut nearest_distance = f32::MAX;

        for plane in self.planes.iter() {
            let distance = math::distance_position_to_plane(plane, &pos);
            if nearest_distance > distance {
                nearest_distance = distance;
                nearest_plane = Some(plane);
            }
        }
        (nearest_distance, nearest_plane.unwrap())
    }
}

impl Default for Field {
    fn default() -> Self {
        Field {
            planes: vec![
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
                Plane::default(),
            ],
        }
    }
}
