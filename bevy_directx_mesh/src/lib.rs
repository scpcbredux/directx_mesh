pub use directx_mesh;
pub use loader::*;

mod loader;

use bevy::prelude::*;

#[derive(Default)]
pub struct DirectXMeshPlugin;

impl Plugin for DirectXMeshPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<DirectXMeshLoader>();
    }
}
