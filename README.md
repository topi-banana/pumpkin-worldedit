# Pumpkin WorldEdit

This project is a plugin for [Pumpkin](https://github.com/PumpkinMC/Pumpkin), a Rust implementation of Minecraft, which aims to implement WorldEdit and VoxelSniper.

## Status

- **WorldEdit**: Partially implemented. See the [WorldEdit README](./worldedit/README.md) for more details.
- **VoxelSniper**: Not yet implemented. See the [VoxelSniper README](./voxelsniper/README.md) for future plans.

## Crates

- [`worldedit`](./worldedit): The WorldEdit implementation.
- [`voxelsniper`](./voxelsniper): The (planned) VoxelSniper implementation.

## Building

To build the project, run the following command:

```bash
cargo build --release
```

This will create the plugin library files in the `target/release` directory.

## Running

1.  After building, locate the generated library file in the `target/release` directory.
    - On Linux, this will be `libworldedit.so` and/or `libvoxelsniper.so`.
    - On Windows, this will be `worldedit.dll` and/or `voxelsniper.dll`.
    - On macOS, this will be `libworldedit.dylib` and/or `libvoxelsniper.dylib`.
2.  Copy the desired plugin library file(s) into the `plugins` directory of your Pumpkin server instance.
3.  Run the Pumpkin server binary. The plugin(s) will be loaded automatically.
