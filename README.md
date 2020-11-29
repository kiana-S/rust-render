# rust-render
A raytracing 3D renderer written in Rust.

### Checklist of features

This list may be changed or extended in the future.

- [x] Raytracing camera
  - [x] Camera struct
  - [x] Ray creation code
- [x] Object code architecture
- [x] Sphere objects
  - [x] Sphere struct
  - [x] Sphere intersection test
  - [x] Sphere normal generation
  - [x] Color mapping on spheres
- [ ] Triangle objects
  - [ ] Triangle struct
  - [ ] Triangle intersection test
  - [ ] Triangle normal generation
  - [ ] Color mapping on triangles
  - [ ] Triangle meshes
- [ ] Bounding boxes
- [ ] Direct lighting
  - [ ] Point light sources
    - [ ] Point source struct
    - [ ] Point source illuminance test
    - [ ] Hard shadows
    - [ ] Soft shadows
  - [ ] Light-emitting surfaces
- [ ] Indirect lighting
  - [ ] Reflection
    - [ ] Perfectly reflective objects
    - [ ] Diffuse reflection
    - [ ] Roughness
  - [ ] Transparency
    - [ ] Simple transparency
    - [ ] Refraction
    
