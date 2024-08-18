use thiserror::Error as ThisError;

#[non_exhaustive]
#[derive(ThisError, Debug)]
pub enum DirectXMeshError {
    #[error(transparent)]
    FloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    IntError(#[from] std::num::ParseIntError),
}

#[derive(Debug, Default)]
pub struct DirectXMesh {
    pub vertices: Vec<(f32, f32, f32)>,
    pub faces: Vec<Vec<u32>>,
    pub normals: Vec<(f32, f32, f32)>,
}

pub fn read_directx_mesh(contents: &str) -> Result<DirectXMesh, DirectXMeshError> {
    let mut vertices_len = 0;
    let mut faces_len = 0;
    let mut normals_len = 0;
    let mut in_mesh = false;
    let mut in_normals = false;

    let mut mesh = DirectXMesh::default();

    for line in contents.lines() {
        let line = line.trim();

        // Determine mesh header
        if line.starts_with("Mesh") && line.ends_with("{") && !in_mesh {
            in_mesh = true;
            continue;
        }

        if in_mesh {
            // Handle vertices
            if vertices_len == 0 {
                vertices_len = line.trim_end_matches(";").parse()?;
                continue;
            }

            if mesh.vertices.len() != vertices_len {
                let parts: Vec<&str> = line
                    .trim_end_matches(",;")
                    .split(";")
                    .map(|p| p.trim_matches(','))
                    .filter(|p| !p.is_empty())
                    .collect();
                if parts.len() == 3 {
                    let vertex: (f32, f32, f32) = (
                        parts[0].parse()?,
                        parts[1].parse()?,
                        parts[2].parse()?,
                    );
                    mesh.vertices.push(vertex);
                }
                continue;
            }

            // Handle faces
            if faces_len == 0 {
                faces_len = line.trim_end_matches(";").parse()?;
                continue;
            }

            if mesh.faces.len() != faces_len {
                let parts: Vec<&str> = line
                    .trim_end_matches(",;")
                    .split(";")
                    .map(|p| p.trim_matches(','))
                    .filter(|p| !p.is_empty())
                    .collect();
                let indices_len: usize = parts[0].parse()?;
                let indices: Vec<&str> = parts[1].split(",").collect();
                if indices.len() == indices_len {
                    let face: Vec<u32> = indices.iter().map(|i| i.parse().unwrap()).collect();
                    mesh.faces.push(face);
                }
                continue;
            }

            // Determine mesh normals header
            if line.starts_with("MeshNormals") && line.ends_with("{") && !in_normals {
                in_normals = true;
                continue;
            }

            // Handle normals
            if in_normals && normals_len == 0 {
                normals_len = line.trim_end_matches(";").parse()?;
                continue;
            }

            if mesh.normals.len() != normals_len {
                let parts: Vec<&str> = line
                    .trim_end_matches(",;")
                    .split(";")
                    .map(|p| p.trim_matches(','))
                    .filter(|p| !p.is_empty())
                    .collect();
                if parts.len() == 3 {
                    let normal: (f32, f32, f32) = (
                        parts[0].parse()?,
                        parts[1].parse()?,
                        parts[2].parse()?,
                    );
                    mesh.normals.push(normal);
                }
                continue;
            }

            if line == "}" {
                continue;
            }
        }
    }

    Ok(mesh)
}
