use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::{
    asset::Assets,
    ecs::system::{Commands, ResMut},
    pbr::{PbrBundle, StandardMaterial},
    render::mesh::Mesh,
    transform::components::Transform,
};
use noise::{NoiseFn, Perlin};

const TANK_SIZE: (usize, usize, usize) = (100, 100, 100);
const STARTING_POS: (usize, usize, usize) = (0, 0, 0);

pub fn spawn_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // generate voxels
    let voxels = generate_noise(TANK_SIZE);
    let mesh = generate_mesh(voxels, TANK_SIZE, STARTING_POS);

    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.7, 0.6),
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material,
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });

    // generate tank
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_length(1.)),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..Default::default()
        }),
        ..Default::default()
    });

    // Create walls
    // let width = 5.;
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Cuboid::from_size(Vec3::new(
    //         TANK_SIZE.0 as f32,
    //         TANK_SIZE.1 as f32,
    //         width,
    //     ))),
    //     material: materials.add(StandardMaterial {
    //         base_color: Color::WHITE,
    //         ..Default::default()
    //     }),
    //     transform: Transform {
    //         translation: Vec3::new(
    //             0. + (TANK_SIZE.1 as f32 / 2.),
    //             0. + (TANK_SIZE.1 as f32 / 2.),
    //             0. - (width / 2.),
    //         ),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });

    let tank_size = (TANK_SIZE.0 as f32, TANK_SIZE.1 as f32, TANK_SIZE.2 as f32);
    let wall_thickness = 1.;
    let start_pos = Vec3::new(
        STARTING_POS.0 as f32,
        STARTING_POS.1 as f32,
        STARTING_POS.2 as f32,
    ); // Define the starting corner position

    let positions = [
        Vec3::new(
            start_pos.x + tank_size.0 / 2.0,
            start_pos.y + tank_size.1 / 2.0,
            start_pos.z - wall_thickness / 2.0,
        ), // back
        Vec3::new(
            start_pos.x + tank_size.0 / 2.0,
            start_pos.y + tank_size.1 / 2.0,
            start_pos.z + tank_size.2 + wall_thickness / 2.0,
        ), // front
        Vec3::new(
            start_pos.x + tank_size.0 + wall_thickness / 2.0,
            start_pos.y + tank_size.1 / 2.0,
            start_pos.z + tank_size.2 / 2.0,
        ), // right
        Vec3::new(
            start_pos.x - wall_thickness / 2.0,
            start_pos.y + tank_size.1 / 2.0,
            start_pos.z + tank_size.2 / 2.0,
        ), // left
        Vec3::new(
            start_pos.x + tank_size.0 / 2.0,
            start_pos.y + tank_size.1 + wall_thickness / 2.0,
            start_pos.z + tank_size.2 / 2.0,
        ), // top
        Vec3::new(
            start_pos.x + tank_size.0 / 2.0,
            start_pos.y - wall_thickness / 2.0,
            start_pos.z + tank_size.2 / 2.0,
        ), // bottom
    ];

    let sizes = [
        Vec3::new(tank_size.0, tank_size.1, wall_thickness), // back
        Vec3::new(tank_size.0, tank_size.1, wall_thickness), // front
        Vec3::new(wall_thickness, tank_size.1, tank_size.2), // right
        Vec3::new(wall_thickness, tank_size.1, tank_size.2), // left
        Vec3::new(tank_size.0, wall_thickness, tank_size.2), // top
        Vec3::new(tank_size.0, wall_thickness, tank_size.2), // bottom
    ];

    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 1.0, 1.0, 0.5), // Semi-transparent
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    for (position, size) in positions.iter().zip(sizes.iter()) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(size.x, size.y, size.z))),
            material: material.clone(),
            transform: Transform {
                translation: *position,
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
fn generate_noise(size: (usize, usize, usize)) -> Vec<Vec<Vec<u8>>> {
    let perlin = Perlin::new(1);
    let mut voxels = vec![vec![vec![0; size.2]; size.1]; size.1];
    for x in 0..size.0 {
        for y in 0..size.1 {
            for z in 0..size.2 {
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
fn generate_mesh(
    voxels: Vec<Vec<Vec<u8>>>,
    size: (usize, usize, usize),
    start_pos: (usize, usize, usize),
) -> Mesh {
    let mut vertices: Vec<Vec3> = vec![];
    let mut indices: Vec<u32> = vec![];
    let block_size = 1.;
    let block_half = block_size / 2.;

    for x in start_pos.0..size.0 {
        for y in start_pos.1..size.1 {
            for z in start_pos.2..size.2 {
                let vertex_pos = [
                    [
                        -block_half + x as f32,
                        block_half + y as f32,
                        -block_half + z as f32,
                    ],
                    [
                        -block_half + x as f32,
                        block_half + y as f32,
                        block_half + z as f32,
                    ],
                    [
                        block_half + x as f32,
                        block_half + y as f32,
                        block_half + z as f32,
                    ],
                    [
                        block_half + x as f32,
                        block_half + y as f32,
                        -block_half + z as f32,
                    ],
                    [
                        -block_half + x as f32,
                        -block_half + y as f32,
                        -block_half + z as f32,
                    ],
                    [
                        -block_half + x as f32,
                        -block_half + y as f32,
                        block_half + z as f32,
                    ],
                    [
                        block_half + x as f32,
                        -block_half + y as f32,
                        block_half + z as f32,
                    ],
                    [
                        block_half + x as f32,
                        -block_half + y as f32,
                        -block_half + z as f32,
                    ],
                ];
                if voxels[x][y][z] == 1 {
                    if x == size.0 - 1 || voxels[x + 1][y][z] == 0 {
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
                    if x == start_pos.0 || voxels[x - 1][y][z] == 0 {
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
                    if y == size.1 - 1 || voxels[x][y + 1][z] == 0 {
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
                    if y == start_pos.1 || voxels[x][y - 1][z] == 0 {
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
                    if z == size.2 - 1 || voxels[x][y][z + 1] == 0 {
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
                    if z == start_pos.2 || voxels[x][y][z - 1] == 0 {
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
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(Indices::U32(indices));
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
