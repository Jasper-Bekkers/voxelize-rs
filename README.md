voxelize-rs
========
[![travis-ci.com](https://travis-ci.com/Jasper-Bekkers/voxelize-rs.svg?branch=master)](https://travis-ci.com/Jasper-Bekkers/voxelize-rs)
[![Latest version](https://img.shields.io/crates/v/voxelize-rs.svg)](https://crates.io/crates/voxelize-rs)
[![Documentation](https://docs.rs/voxelize-rs/badge.svg)](https://docs.rs/voxelize-rs)
[![](https://tokei.rs/b1/github/Jasper-Bekkers/voxelize-rs)](https://github.com/Jasper-Bekkers/voxelize-rs)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

Native voxelization library

- [Documentation](https://docs.rs/voxelize-rs)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
voxelize-rs = "0.1.0"
```

```rust
let tri = Triangle::new(
    Vector3::new(0f32, 0f32, 0f32),
    Vector3::new(0f32, 1f32, 0f32),
    Vector3::new(0f32, 0f32, 1f32),
);

let coords = voxelize(&vec![tri], &Vector3::new(0.1, 0.1, 0.1));
```

## License

Licensed under MIT license (http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, shall be licensed as above, without any additional terms or conditions.

Contributions are always welcome; please look at the [issue tracker](https://github.com/Jasper-Bekkers/voxelize-rs/issues) to see what known improvements are documented.
