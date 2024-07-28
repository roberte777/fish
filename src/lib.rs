use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    pbr::{PbrBundle, StandardMaterial},
    render::{
        color::Color,
        mesh::{shape, Mesh},
    },
    transform::components::Transform,
};
use noise::{NoiseFn, Perlin};

pub fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // basic bevy cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0., 0., 10.),
        ..default()
    });
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
    let mut voxels = [[[0_u8; 32]; 32]; 32];
    (0..32).for_each(|x| {
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
    });
    voxels
}
fn generate_mesh(voxels: [[[u8; 32]; 32]; 32]) -> Mesh {
    let mut vertices: Vec<Vec3> = vec![];
    let mut indices: Vec<u32> = vec![];

    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                let vertex_pos = [
                    [-0.5 + x as f32, 0.5 + y as f32, -0.5 + z as f32],
                    [-0.5 + x as f32, 0.5 + y as f32, 0.5 + z as f32],
                    [0.5 + x as f32, 0.5 + y as f32, 0.5 + z as f32],
                    [0.5 + x as f32, 0.5 + y as f32, -0.5 + z as f32],
                    [-0.5 + x as f32, -0.5 + y as f32, -0.5 + z as f32],
                    [-0.5 + x as f32, -0.5 + y as f32, 0.5 + z as f32],
                    [0.5 + x as f32, -0.5 + y as f32, 0.5 + z as f32],
                    [0.5 + x as f32, -0.5 + y as f32, -0.5 + z as f32],
                ];
                if voxels[x][y][z] == 1 {
                    if x == 31 || voxels[x + 1][y][z] == 0 {
                        // front face
                        // positive x
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
                    if x == 0 || voxels[x - 1][y][z] == 0 {
                        // back face
                        // negative x
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
                    if y == 31 || voxels[x][y + 1][z] == 0 {
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
                    if y == 0 || voxels[x][y - 1][z] == 0 {
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
                    if z == 31 || voxels[x][y][z + 1] == 0 {
                        // right face
                        // positive z
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
                    if z == 0 || voxels[x][y][z - 1] == 0 {
                        // left face
                        // neagtive z
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
                }
            }
        }
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();
    mesh
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
    indices.push(i as u32);
    indices.push((i + 1) as u32);
    indices.push((i + 2) as u32);
    indices.push(i as u32);
    indices.push((i + 2) as u32);
    indices.push((i + 3) as u32);
}
