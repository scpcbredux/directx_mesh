## DirectX Mesh

[![crates.io](https://img.shields.io/crates/v/directx_mesh.svg)](https://crates.io/crates/directx_mesh) [![docs.rs](https://docs.rs/directx_mesh/badge.svg)](https://docs.rs/directx_mesh)

Rust parser for **Legacy Microsoft DirectX Mesh** file extension.

### Usage

```rust
let bytes = std::fs::read_to_string("GFX/map.x").unwrap();
let x_mesh = read_directx_mesh(&bytes).unwrap();
assert_eq!(x_mesh.vertices.len(), 301);
```

### Examples

- [read](rmesh/examples/read.rs)

### Task list

- [ ] Write documentation
- [ ] Create a writer
- [ ] Read materials
- [ ] Read animations, bones, etc
