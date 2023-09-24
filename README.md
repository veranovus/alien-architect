# Alien Architect - GB Jam #11

Super cool puzzle games about managing space and aliens!

> Version 0.2.6

## Credits

- Programming : `veranovus`
- Art & Game Design : `st_levi`
- SFX & Music: `danielsbk`

## Changelog

- `v0.1.0`: Initial commit.
- `v0.1.1`: Implemented Window creation.
- `v0.1.1`: Implemented a barebones version og Tile generation.
- `v0.1.2`: Map abstraction.
- `v0.1.2`: Tile abstraction.
- `v0.1.2`: Refactored Tile generation.
- `v0.1.3`: Implemented loading levels from file.
- `v0.1.4`: Implemented loading objects from file.
- `v0.1.4`: Object asset loading is automatized and controlled via `asset-conf.ron`.
- `v0.1.5`: Refactored Grid, and switched from Map to World.
- `v0.1.6`: Refactored loading levels.
- `v0.1.7`: Camera module.
- `v0.1.7`: Refactored object asset loading, and added `ObjectConf`, which are pre-made configurations stored
  at `object-conf.ron`.
- `v0.1.7`: Added `offset` property to `Grid`.
- `v0.1.8`: Re-implemented level loading.
- `v0.1.9`: Implemented UFO movement.
- `v0.1.9`: UFO modifies the `TileState` of tile its hovering.
- `v0.2.0`: Implemented `Object` selection.
- `v0.2.1`: Refactored `UFO` movement, and `UFO` object selection.
- `v0.2.1`: Implemented `UFOLiftEvent`, which is used to make UFO carry what its hovering.
- `v0.2.2`: Implemented `UFODropEvent`, and tile visualization for valid drop locations.
- `v0.2.3`: Fixed a bug causing `Object`'s to only register their initial cell to world.
- `v0.2.3`: `Object`'s now update their posiitons in World when they are moved.
- `v0.2.3`: Fixed a bug which occured when a Selectable object had less than 2 sprites.
- `v0.2.4`: Implemented movement rules, for every `Object`, except `Villager` and `Cow`.
- `v0.2.5`: Implemented rest of the movement rules.
- `v0.2.6`: Implemented animated `Objects`, and `AnimationPlugin`.
- `v0.2.6`: Implemented an animated version of `UFO`.
- `v0.2.6`: Implemented `WarningIndicator` for `UFO` to show when player tries to lift an immobile object.

# Update
- Change `Grid`'s `cell_to_world` function from `u32` to `i32` to account for negative positions.

