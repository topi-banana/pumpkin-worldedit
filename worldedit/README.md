# WorldEdit for Pumpkin

This crate provides a partial implementation of the WorldEdit plugin for the [Pumpkin](https://github.com/PumpkinMC/Pumpkin) Minecraft server.

## Implemented Commands

The following WorldEdit commands are currently implemented:

- `//wand`: Gives you the selection wand.
- `//pos1`: Sets your first selection position.
- `//pos2`: Sets your second selection position.
- `//set <block>`: Sets all blocks in the selection to a specific block type.
- `//replace <from_block> <to_block>`: Replaces all blocks of a certain type with another type in the selection.
- `//copy`: Copies the selected region.
- `//paste`: Pastes the copied region.
- `//undo`: Undoes your last action.
- `//redo`: Redoes your last undone action.
- `//sel`: Allows for changing the selection shape (currently a placeholder).

## Usage

1.  Build the project using `cargo build --release`.
2.  Copy the generated library file (e.g., `libworldedit.so`, `worldedit.dll`) from the `target/release` directory to your Pumpkin server's `plugins` directory.
3.  Start the Pumpkin server. You can then use the commands in-game.

## Future Development

We plan to implement more commands and features from the original WorldEdit plugin. Contributions are welcome!
