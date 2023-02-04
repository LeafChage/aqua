use super::*;
use crate::field;
use crate::math;
use bevy::time::FixedTimestep;
use bevy_prototype_debug_lines::*;

pub struct FishPlugin;

impl Plugin for FishPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_fish)
            .add_system_set(
                SystemSet::new()
                    .with_system(update_group_position)
                    .with_system(update_group_direction)
                    .with_system(
                        caliculate_separation_from_others_force.after(update_group_position),
                    )
                    .with_system(caliculate_separation_from_wall_force.after(update_group_position))
                    .with_system(caliculate_alignment_force.after(update_group_position))
                    .with_system(caliculate_cohesion_force.after(update_group_position))
                    .with_system(
                        caliculate_compounded_force
                            .after(caliculate_separation_from_others_force)
                            .after(caliculate_separation_from_wall_force)
                            .after(caliculate_alignment_force)
                            .after(caliculate_cohesion_force),
                    )
                    // .with_system(debug.after(caliculate_compounded_force))
                    .with_system(fish_movement.after(caliculate_compounded_force)),
            )
            // .add_system_set(
            //     SystemSet::new()
            //         .with_run_criteria(FixedTimestep::step(0.1))
            //         .with_system(caliculate_compounded_force),
            // )
            .insert_resource(GroupPosition::default())
            .insert_resource(GroupDirection::default());
    }
}

fn spawn_fish(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in 0..FISH_COUNT {
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::default())),
                material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
                transform: Transform::from_translation(math::random_vec3(
                    -(field::FIELD_DISTANCE - 1.)..(field::FIELD_DISTANCE - 1.),
                ))
                .with_scale(Vec3::new(0.1, 0.1, 0.3)),
                ..default()
            })
            .insert(Fish)
            .insert(SeparationFromOthersForce(Vec3::default()))
            .insert(SeparationFromWallForce(Vec3::default()))
            .insert(AlignmentForce(Vec3::default()))
            .insert(CohesionForce(Vec3::default()))
            .insert(CompoundedForce(Vec3::default()))
            .insert(Speed(math::random_range(0.5..0.75)));
    }
}

fn update_group_position(
    mut group_center: ResMut<GroupPosition>,
    transforms: Query<&Transform, With<Fish>>,
) {
    let count = transforms.iter().count().clone();
    *group_center = GroupPosition(
        transforms
            .iter()
            .fold(Vec3::ZERO, |sum, b| sum + b.translation)
            / count as f32,
    );
}

fn update_group_direction(
    mut group_direction: ResMut<GroupDirection>,
    forces: Query<&CompoundedForce, With<Fish>>,
) {
    let count = forces.iter().count().clone();
    *group_direction = GroupDirection(
        forces
            .iter()
            .fold(Vec3::ZERO, |sum, b| sum + b.0.normalize_or_zero())
            / count as f32,
    );
}

fn caliculate_separation_from_others_force(
    mut targets: Query<(Entity, &mut SeparationFromOthersForce, &Transform), With<Fish>>,
    group: Query<(Entity, &Transform), With<Fish>>,
) {
    for (me, mut force, me_transform) in &mut targets {
        let mut near_position: Option<Vec3> = None;
        let mut closest_distance = MOST_CLOSEST_DISTANCE;
        // get closest transform.
        for (other, other_transform) in &group {
            if other == me {
                continue;
            }

            let distance = me_transform
                .translation
                .distance(other_transform.translation);

            if closest_distance > distance {
                closest_distance = distance;
                near_position = Some(other_transform.translation);
            }
        }

        force.0 = if let Some(position) = near_position {
            (me_transform.translation - position).normalize()
                * 2.0
                * ((MOST_CLOSEST_DISTANCE - closest_distance) / MOST_CLOSEST_DISTANCE)
        } else {
            Vec3::ZERO
        }
    }
}

fn caliculate_separation_from_wall_force(
    field: Res<field::Field>,
    mut targets: Query<(&mut SeparationFromWallForce, &Transform), With<Fish>>,
) {
    for (mut force, transform) in &mut targets {
        let (distance, nearest_plane) = field.nearest_plane(transform.translation);

        force.0 = if distance < MOST_CLOSEST_DISTANCE {
            Vec3::from(nearest_plane.normal())
        } else {
            Vec3::ZERO
        };
    }
}

fn caliculate_alignment_force(
    group_direction: Res<GroupDirection>,
    mut targets: Query<&mut AlignmentForce, With<Fish>>,
) {
    for mut force in &mut targets {
        force.0 = group_direction.0.normalize_or_zero();
    }
}

fn caliculate_cohesion_force(
    group_center: Res<GroupPosition>,
    mut targets: Query<(&Transform, &mut CohesionForce), With<Fish>>,
) {
    for (t, mut force) in &mut targets {
        let len = (group_center.0 - t.translation).length();
        force.0 = (group_center.0 - t.translation).normalize_or_zero() * 2.0 * (len / GROUP_SCALE);
    }
}

fn debug(
    queries: Query<
        (
            &SeparationFromWallForce,
            &SeparationFromOthersForce,
            &AlignmentForce,
            &CohesionForce,
            &mut CompoundedForce,
            &Transform,
        ),
        With<Fish>,
    >,
    mut lines: ResMut<DebugLines>,
) {
    for (
        SeparationFromWallForce(separation_from_wall),
        SeparationFromOthersForce(separation_from_other),
        AlignmentForce(alighment),
        CohesionForce(cohesion),
        CompoundedForce(force),
        t,
    ) in &queries
    {
        let duration = 0.03;
        let position = t.translation;
        lines.line_colored(
            position,
            position + *separation_from_wall,
            duration,
            Color::rgb(1., 1., 0.),
        );
        lines.line_colored(
            position,
            position + *separation_from_other,
            duration,
            Color::rgba(0., 1., 0., 0.5),
        );
        lines.line_colored(
            position,
            position + *alighment,
            duration,
            Color::rgba(0., 0., 1., 0.5),
        );
        lines.line_colored(
            position,
            position + *cohesion,
            duration,
            Color::rgba(1., 0., 0., 0.5),
        );
        lines.line_colored(
            position,
            position + *force * 5.,
            duration,
            Color::rgb(0., 0., 0.),
        );
    }
}

fn caliculate_compounded_force(
    mut queries: Query<
        (
            &SeparationFromWallForce,
            &SeparationFromOthersForce,
            &AlignmentForce,
            &CohesionForce,
            &mut CompoundedForce,
        ),
        With<Fish>,
    >,
) {
    for (
        SeparationFromWallForce(separation_from_wall),
        SeparationFromOthersForce(separation_from_other),
        AlignmentForce(alighment),
        CohesionForce(cohesion),
        mut compounded,
    ) in &mut queries
    {
        compounded.0 = (compounded.0 * 10.
            + *separation_from_wall
            + *separation_from_other
            + *alighment
            + *cohesion)
            / 14.;
    }
}

fn fish_movement(mut queries: Query<(&CompoundedForce, &Speed, &mut Transform), With<Fish>>) {
    for (CompoundedForce(force), Speed(speed), mut t) in &mut queries {
        let position = t.translation;
        t.look_at(position + *force, Vec3::Y);
        t.translation += *force * *speed;
    }
}
