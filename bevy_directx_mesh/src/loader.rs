use anyhow::Result;
use bevy::asset::io::Reader;
use bevy::asset::AsyncReadExt;
use bevy::asset::{AssetLoader, LoadContext};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::{
    mesh::{Indices, Mesh},
    render_resource::PrimitiveTopology,
};
use directx_mesh::read_directx_mesh;

#[derive(Default)]
pub struct DirectXMeshLoader;

impl AssetLoader for DirectXMeshLoader {
    type Asset = Mesh;
    type Settings = ();
    type Error = anyhow::Error;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut content = String::new();
        reader.read_to_string(&mut content).await?;
        load_x_mesh(&content)
    }

    fn extensions(&self) -> &[&str] {
        &["x"]
    }
}

/// Loads an entire x file.
fn load_x_mesh<'a>(content: &'a str) -> Result<Mesh> {
    let header = read_directx_mesh(content)?;

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    let positions: Vec<_> = header.vertices.iter().map(|v| [v.0, -v.1, v.2]).collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);

    let indices: Vec<u32> = header.faces.iter().flatten().cloned().collect();
    mesh.insert_indices(Indices::U32(indices));

    let normals: Vec<_> = header.normals.iter().map(|v| [v.0, v.1, v.2]).collect();
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    Ok(mesh)
}
