use bevy::{
    input::mouse::MouseMotion,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
};

use fish::spawn_cubes;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_cubes, setup))
        .add_systems(Update, camera_movement);

    #[cfg(feature = "wireframes")]
    {
        setup_wireframes(&mut app);
    }

    app.run();
}

fn setup_wireframes(app: &mut App) {
    app.add_plugins((WireframePlugin,))
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: Color::WHITE,
        });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
    // flying camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });
}
fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    // rotate camera with mouse
    let mut rotation_move = Vec2::ZERO;
    for ev in motion_evr.read() {
        rotation_move += ev.delta;
    }
    if rotation_move.length() > 0.0 {
        let yaw = Quat::from_rotation_y(-rotation_move.x * time.delta_seconds());
        let pitch = Quat::from_rotation_x(-rotation_move.y * time.delta_seconds());
        for mut transform in query.iter_mut() {
            transform.rotation = yaw * transform.rotation;
            transform.rotation *= pitch;
        }
    }
    // wasd movement
    let move_factor = 4.;
    let mut translation_move = Vec3::ZERO;
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            translation_move += transform.forward() * move_factor * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::S) {
            translation_move -= transform.forward() * move_factor * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::A) {
            translation_move -= transform.right() * move_factor * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::D) {
            translation_move += transform.right() * move_factor * time.delta_seconds();
        }
        transform.translation += translation_move;
    }
}
