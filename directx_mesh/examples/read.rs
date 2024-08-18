use directx_mesh::{read_directx_mesh, DirectXMeshError};

fn main() -> Result<(), DirectXMeshError> {
    let mut args = std::env::args();
    let _ = args.next();
    let contents = std::fs::read_to_string(args.next().expect("No x file provided")).unwrap();
    let x_mesh = read_directx_mesh(&contents)?;
    println!("{:#?}", x_mesh);
    Ok(())
}
