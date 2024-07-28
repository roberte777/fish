use bevy::{
    input::mouse::MouseMotion,
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use fish::spawn_cubes;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_cubes, setup))
        .add_systems(Startup, cursor_grab)
        .add_systems(Update, camera_movement);

    #[cfg(feature = "wireframes")]
    {
        setup_wireframes(&mut app);
    }

    app.run();
}

fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    primary_window.cursor.grab_mode = CursorGrabMode::Confined;

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor.visible = false;
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

fn setup(mut commands: Commands) {
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
        transform: Transform::from_xyz(0., 100., 0.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
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
    let move_factor = 5.;
    let mut translation_move = Vec3::ZERO;
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            translation_move += transform.forward() * move_factor * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            translation_move -= transform.forward() * move_factor * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            translation_move -= transform.right() * move_factor * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            translation_move += transform.right() * move_factor * time.delta_seconds();
        }
        transform.translation += translation_move;
    }
}
