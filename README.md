# Alien Architect - GB Jam #11

Super cool puzzle games about managing space and aliens!

> Version 0.3.1

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
- `v0.2.7`: Implemented `AppState` and scene transitions.
- `v0.2.7`: Added `Splash` scene.
- `v0.2.7`: Small bug fixes, and quality improvements.
- `v0.2.8`: Implemented `GameUI`.
- `v0.2.8`: `SpawnWarningEvent` is now send also when player tries to place an object to invalid tile.
- `v0.2.9`: Fixed some bugs related to `GameUI`.
- `v0.2.9`: Implemented `ObjectsActTurnsEvent` and some `Object`'s now perform actions at the end of their turns.
- `v0.2.9`: Implemented pathfinding for `ObjectID::King`.
- `v0.2.9`: Implemented winb condition, and `PlayerWinEvent`.
- `v0.2.9`: Implemented `WinAnimation`.
- `v0.3.0`: Fixed some bugs related to validation of Object drop locations.
- `v0.3.0`: Implementd the system for Villager and Cow's special drop actions.
- `v0.3.1`: Implemented `EndScreen`.
- `v0.3.1`: Implemented `UFOCancalEvent` and set up the `Cancel` button.
- `v0.3.1`: Impelemnted `Restart` button, and temporary introduced new state `GameState`.

# Update

- Change `Grid`'s `cell_to_world` function from `u32` to `i32` to account for negative positions.

