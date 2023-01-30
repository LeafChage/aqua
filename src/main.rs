extern crate bevy;
extern crate bevy_prototype_debug_lines;
extern crate rand;

mod field;
mod fish;
mod global;
mod math;

use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
// use bevy::time::FixedTimestep;

const WINDOW_WIDTH: f32 = 1000.;
const WINDOW_HEIGHT: f32 = 1000.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .add_startup_system(global::setup_world)
        // .add_system(look_at_main)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Aqua".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(field::FieldPlugin)
        .add_plugin(fish::FishPlugin)
        // .add_plugin(fish::FishDebugPlugin)
        .run();
}

fn look_at_main(
    group_position: Res<fish::GroupPosition>,
    mut query: Query<&mut Transform, With<global::Camera>>,
) {
    for mut t in query.iter_mut() {
        let up = t.up().clone();
        t.look_at(group_position.0, up);
        t.translation = group_position.0 - t.forward() * 10.0
    }
}
