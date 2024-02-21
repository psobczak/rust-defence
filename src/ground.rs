use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use noise::utils::NoiseMapBuilder;
use noise::{utils::PlaneMapBuilder, Perlin};

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh_data = GroundMeshData::new(100, 100, 5.0);

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh_data.generate_mesh()),
        material: materials.add(Color::WHITE),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },

        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}

struct GroundMeshData {
    width: u32,
    depth: u32,
    extent: f64
}

impl GroundMeshData {
    pub fn new(width: u32, depth: u32, extent: f64) -> Self {
        Self { width, depth, extent }
    }

    fn vertices_count(&self) -> usize {
        ((self.width + 1) * (self.depth + 1)) as usize
    }

    fn triangle_count(&self) -> usize {
        (self.width * self.depth * 2 * 3) as usize
    }

    fn generate_mesh(&self) -> Mesh {
        let perlin = Perlin::new(2137);

        let extent = 10.0;

        let noisemap = PlaneMapBuilder::<Perlin, 2>::new(perlin)
            .set_size(self.width as usize, self.depth as usize)
            .set_x_bounds(-self.extent, self.extent)
            .set_y_bounds(-self.extent, self.extent)
            .build();


        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(self.vertices_count());
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(self.vertices_count());
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(self.vertices_count());

        let extent = extent as f32;

        for d in 0..=self.width {
            for w in 0..=self.depth {
                let (w, d) = (w as f32, d as f32);
                let (width, depth) = (self.width as f32, self.depth as f32);


                let height =  (noisemap.get_value(w as usize, d as usize) as f32) * 1.0;
                info!("{}", height);

                let pos = [
                    (w - width / 2.0) * extent / width,
                    height,
                    (d - depth / 2.0) * extent / depth,
                ];
                positions.push(pos);
                normals.push([0.0, 1.0, 0.0]);
                uvs.push([w / width, d / depth]);
            }
        }

        // Defining triangles.
        let mut triangles: Vec<u32> = Vec::with_capacity(self.triangle_count());

        for d in 0..self.depth {
            for w in 0..self.width {
                // First triangle
                triangles.push((d * (self.width + 1)) + w);
                triangles.push(((d + 1) * (self.width + 1)) + w);
                triangles.push(((d + 1) * (self.width + 1)) + w + 1);
                // Second triangle
                triangles.push((d * (self.width + 1)) + w);
                triangles.push(((d + 1) * (self.width + 1)) + w + 1);
                triangles.push((d * (self.width + 1)) + w + 1);
            }
        }

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(triangles))
    }
}
