use super::*;

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_aqua_field)
            .insert_resource(Field::default());
    }
}

fn spawn_aqua_field(
    mut commands: Commands,
    mut field_planes: ResMut<Field>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pole_transforms = [
        (
            Vec3::new(FIELD_DISTANCE, 0., FIELD_DISTANCE),
            Vec3::new(0.1, FIELD_DISTANCE * 2., 0.1),
        ),
        (
            Vec3::new(-FIELD_DISTANCE, 0., FIELD_DISTANCE),
            Vec3::new(0.1, FIELD_DISTANCE * 2., 0.1),
        ),
        (
            Vec3::new(FIELD_DISTANCE, 0., -FIELD_DISTANCE),
            Vec3::new(0.1, FIELD_DISTANCE * 2., 0.1),
        ),
        (
            Vec3::new(-FIELD_DISTANCE, 0., -FIELD_DISTANCE),
            Vec3::new(0.1, FIELD_DISTANCE * 2., 0.1),
        ),
        (
            Vec3::new(0., FIELD_DISTANCE, FIELD_DISTANCE),
            Vec3::new(FIELD_DISTANCE * 2., 0.1, 0.1),
        ),
        (
            Vec3::new(0., FIELD_DISTANCE, -FIELD_DISTANCE),
            Vec3::new(FIELD_DISTANCE * 2., 0.1, 0.1),
        ),
        (
            Vec3::new(0., -FIELD_DISTANCE, FIELD_DISTANCE),
            Vec3::new(FIELD_DISTANCE * 2., 0.1, 0.1),
        ),
        (
            Vec3::new(0., -FIELD_DISTANCE, -FIELD_DISTANCE),
            Vec3::new(FIELD_DISTANCE * 2., 0.1, 0.1),
        ),
        (
            Vec3::new(FIELD_DISTANCE, FIELD_DISTANCE, 0.),
            Vec3::new(0.1, 0.1, FIELD_DISTANCE * 2.),
        ),
        (
            Vec3::new(-FIELD_DISTANCE, FIELD_DISTANCE, 0.),
            Vec3::new(0.1, 0.1, FIELD_DISTANCE * 2.),
        ),
        (
            Vec3::new(FIELD_DISTANCE, -FIELD_DISTANCE, 0.),
            Vec3::new(0.1, 0.1, FIELD_DISTANCE * 2.),
        ),
        (
            Vec3::new(-FIELD_DISTANCE, -FIELD_DISTANCE, 0.),
            Vec3::new(0.1, 0.1, FIELD_DISTANCE * 2.),
        ),
    ];

    for (position, scale) in pole_transforms.into_iter() {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::default())),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            transform: Transform::from_translation(position).with_scale(scale),
            ..default()
        });
    }

    let positions = [
        Vec3::new(0., FIELD_DISTANCE, 0.),
        Vec3::new(0., -FIELD_DISTANCE, 0.),
        Vec3::new(FIELD_DISTANCE, 0., 0.),
        Vec3::new(-FIELD_DISTANCE, 0., 0.),
        Vec3::new(0., 0., FIELD_DISTANCE),
        Vec3::new(0., 0., -FIELD_DISTANCE),
    ];

    *field_planes = Field::new(
        positions
            .iter()
            .map(|p| {
                let v = p.normalize();
                Plane::new(Vec4::new(v.x, v.y, v.z, v.dot(p.clone())))
            })
            .collect::<Vec<_>>(),
    );
}
