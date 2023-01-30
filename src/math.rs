use bevy::prelude::*;
use bevy::render::primitives::Plane;
use rand::random;

pub fn random_range(range: std::ops::Range<f32>) -> f32 {
    random::<f32>() * (range.end - range.start) + range.start
}

#[test]
fn it_random_range() {
    for _ in 0..1000000 {
        let result = random_range(-10.0..5.0);
        assert!(-10. <= result && result <= 5.);
        let result = random_range(1.0..100.0);
        assert!(1. <= result && result <= 100.);
        let result = random_range(-100.0..-5.0);
        assert!(-100. <= result && result <= -5.);
    }
}

pub fn random_vec3(range: std::ops::Range<f32>) -> Vec3 {
    Vec3::new(
        random_range(range.clone()),
        random_range(range.clone()),
        random_range(range.clone()),
    )
}

pub fn distance_position_to_plane(plane: &Plane, v: &Vec3) -> f32 {
    (v.dot(plane.normal().into()) + plane.d()).abs()
}

#[test]
fn it_distance_position_to_plane() {
    assert_eq!(
        distance_position_to_plane(
            &Plane::new(Vec4::new(0., 0., 1., -3.)),
            &Vec3::new(0., 0., 1.)
        ),
        2.
    );
}
