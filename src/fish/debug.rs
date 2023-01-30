use super::*;
use crate::field;

pub struct FishDebugPlugin;

impl Plugin for FishDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_group)
            .add_startup_system(spawn_cell)
            .add_system(group_movement);
    }
}

#[derive(Component)]
struct Group;

fn spawn_group(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
            material: materials.add(Color::rgb(0.9, 0., 0.).into()),
            transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..default()
        })
        .insert(Group);
}

fn spawn_cell(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let d = field::FIELD_DISTANCE.round() as i32;
    for x in -d..d {
        for y in -d..d {
            for z in -d..d {
                commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
                    material: materials.add(Color::rgb(0., 0., 0.).into()),
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32)
                        .with_scale(Vec3::new(0.01, 0.01, 0.01)),
                    ..default()
                });
            }
        }
    }
}

fn group_movement(
    group_center: Res<GroupPosition>,
    mut transforms: Query<&mut Transform, With<Group>>,
) {
    for mut t in &mut transforms {
        t.translation = group_center.0;
    }
}
