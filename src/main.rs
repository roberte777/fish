use bevy::{
    pbr::wireframe::{WireframeConfig, WireframePlugin},
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use noise::{NoiseFn, Perlin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            // You need to add this plugin to enable wireframe rendering
            WireframePlugin,
        ))
        // Wireframes can be configured with this resource. This can be changed at runtime.
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: true,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: Color::WHITE,
        })
        .add_systems(Startup, (spawn_cubes, setup))
        .add_systems(Update, camera_movement)
        .run();
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
    mut query: Query<&mut Transform, With<Camera3d>>,
) {
    let mut camera = query.single_mut();
    let mut forward = camera.forward();
    forward.y = 0.;
    forward = forward.normalize();
    let speed = 3.;
    let rotate_speed = 3.;
    if keyboard_input.pressed(KeyCode::W) {
        camera.translation += forward * speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::S) {
        camera.translation -= forward * speed * time.delta_seconds();
    }
    let right = camera.rotation * Vec3::X;
    if keyboard_input.pressed(KeyCode::D) {
        camera.translation += right * speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::A) {
        camera.translation -= right * speed * time.delta_seconds();
    }
    let up = camera.rotation * Vec3::Y;
    if keyboard_input.pressed(KeyCode::Space) {
        camera.translation += up * speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        camera.translation -= up * speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds());
    }
    if keyboard_input.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds());
    }
}

fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let voxels = generate_noise();
    let mesh = generate_mesh(voxels);

    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material,
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}
fn generate_noise() -> [[[u8; 32]; 32]; 32] {
    let perlin = Perlin::new(1);
    let mut voxels = [[[0 as u8; 32]; 32]; 32];
    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                let scale = 0.1;
                voxels[x][y][z] =
                    if perlin.get([x as f64 * scale, y as f64 * scale, z as f64 * scale]) > 0.0 {
                        1
                    } else {
                        0
                    };
            }
        }
    }
    voxels
}
fn generate_mesh(voxels: [[[u8; 32]; 32]; 32]) -> Mesh {
    let mut vertices: Vec<Vec3> = vec![];
    let mut indices: Vec<u32> = vec![];
    for x in 1..31 {
        for y in 1..31 {
            for z in 1..31 {
                let vertex_pos = [
                    [-1. + x as f32, 1. + y as f32, -1. + z as f32],
                    [-1. + x as f32, 1. + y as f32, 1. + z as f32],
                    [1. + x as f32, 1. + y as f32, 1. + z as f32],
                    [1. + x as f32, 1. + y as f32, -1. + z as f32],
                    [-1. + x as f32, -1. + y as f32, -1. + z as f32],
                    [-1. + x as f32, -1. + y as f32, 1. + z as f32],
                    [1. + x as f32, -1. + y as f32, 1. + z as f32],
                    [1. + x as f32, -1. + y as f32, -1. + z as f32],
                ]; // println!("voxels: {}", voxels[x][y][z]);
                if voxels[x][y][z] == 1 {
                    if voxels[x + 1][y][z] == 0 {
                        // front face
                        add_quad(
                            2,
                            1,
                            5,
                            6,
                            vertices.len(),
                            vertex_pos,
                            &mut vertices,
                            &mut indices,
                        );
                    }
                    if voxels[x - 1][y][z] == 0 {
                        // back face
                        add_quad(
                            0,
                            3,
                            7,
                            4,
                            vertices.len(),
                            vertex_pos,
                            &mut vertices,
                            &mut indices,
                        );
                    }
                    if voxels[x][y + 1][z] == 0 {
                        // top face
                        add_quad(
                            0,
                            1,
                            2,
                            3,
                            vertices.len(),
                            vertex_pos,
                            &mut vertices,
                            &mut indices,
                        );
                    }
                    if voxels[x][y - 1][z] == 0 {
                        // bottom face
                        add_quad(
                            7,
                            6,
                            5,
                            4,
                            vertices.len(),
                            vertex_pos,
                            &mut vertices,
                            &mut indices,
                        );
                    }
                    if voxels[x][y][z + 1] == 0 {
                        // right face
                        add_quad(
                            3,
                            2,
                            6,
                            7,
                            vertices.len(),
                            vertex_pos,
                            &mut vertices,
                            &mut indices,
                        );
                    }
                    if voxels[x][y][z - 1] == 0 {
                        // left face
                        add_quad(
                            1,
                            0,
                            4,
                            5,
                            vertices.len(),
                            vertex_pos,
                            &mut vertices,
                            &mut indices,
                        );
                    }
                }
                fn add_quad(
                    ai: usize,
                    bi: usize,
                    ci: usize,
                    di: usize,
                    i: usize,
                    vertex_pos: [[f32; 3]; 8],
                    vertices: &mut Vec<Vec3>,
                    indices: &mut Vec<u32>,
                ) {
                    let a = vertex_pos[ai];
                    let b = vertex_pos[bi];
                    let c = vertex_pos[ci];
                    let d = vertex_pos[di];
                    vertices.push(Vec3::new(a[0], a[1], a[2]));
                    vertices.push(Vec3::new(b[0], b[1], b[2]));
                    vertices.push(Vec3::new(c[0], c[1], c[2]));
                    vertices.push(Vec3::new(d[0], d[1], d[2]));
                    // println!("vertices: {}", vertices.len());
                    indices.push(i as u32);
                    indices.push((i + 1) as u32);
                    indices.push((i + 2) as u32);
                    indices.push(i as u32);
                    indices.push((i + 2) as u32);
                    indices.push((i + 3) as u32);
                }
            }
        }
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    println!("vertices: {:?}", vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();
    mesh
}
