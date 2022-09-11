use bevy::prelude::*;
use rand;

const NAMES: [&str; 8] = [
    "Tim", // $20
    "Ducksworth",
    "Antonio",
    "Bill",
    "Daisy",
    "Ernie",
    "Mallory",
    "Waddles",
];

#[derive(Component)]
struct Duckling {
    name: String,
}

#[derive(Component)]
struct Stressor;

fn main() {
    App::new()
        // .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "DUCK GAME".to_string(),
            position: WindowPosition::Centered(MonitorSelection::Primary),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(flee)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn flee(time: Res<Time>, mut query: Query<(&mut Transform, Option<&Stressor>)>) {
    let mut stressors: Vec<Transform> = vec![];

    for (stressor_transform, stressor_stressor) in query.iter() {
        if let Some(_stressor_stressor) = stressor_stressor {
            stressors.push(stressor_transform.clone());
        }
    }

    for stressor_transform in stressors.iter() {
        for (mut duckling_transform, duckling_stressor) in query.iter_mut() {
            if let None = duckling_stressor {
                let dir = (duckling_transform.translation - stressor_transform.translation)
                    .normalize_or_zero();
                let distance = duckling_transform
                    .translation
                    .distance(stressor_transform.translation);
                if distance < 5.0 {
                    duckling_transform.translation +=
                        (dir * 0.001 * time.delta().as_millis() as f32);
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for index in (0..3).rev() {
        if index < NAMES.len() {
            let name = NAMES[index];
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere{ radius: 0.2, ..default() })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::hex("ffffff").unwrap(),
                        // unlit: true,
                        ..default()
                    }),
                    transform: Transform::from_xyz(
                        rand::random::<f32>() * 3.0,
                        rand::random::<f32>() * 3.0,
                        rand::random::<f32>() * 3.0,
                    ),
                    ..default()
                })
                .insert(Duckling {
                    name: name.to_string(),
                });
        }
    }

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {..default()})),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("ff0000").unwrap(),
                // unlit: true,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Stressor);

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0),
        point_light: PointLight {
            intensity: 600000.,
            range: 100.,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(10.0, 10.0, 8.0).looking_at(Vec3::default(), Vec3::Y),
        ..default()
    });
}
