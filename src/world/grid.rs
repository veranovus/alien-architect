use bevy::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_grid);
    }
}

/************************************************************
 * - Constants
 */

const GRID_SIZE: (u32, u32) = (5, 9);
const CELL_SIZE: (u32, u32) = (30, 18);
const CELL_OFFSET: (u32, u32) = (2, 4);

/************************************************************
 * - Types
 */

#[derive(Debug, Resource)]
pub struct Grid {
    cell_size: (u32, u32),
    pub cell_offset: (u32, u32),
    pub size: (u32, u32),
    pub grid: Vec<usize>,
}

impl Grid {
    fn new(size: (u32, u32), cell_size: (u32, u32), cell_offset: (u32, u32)) -> Self {
        Self {
            cell_size,
            cell_offset,
            size,
            grid: Vec::new(),
        }
    }

    pub fn grid_to_world(&self, pos: UVec2) -> Vec2 {
        return Vec2::new(
            (pos.x * (self.cell_size.0 - self.cell_offset.0)) as f32
                + ((pos.y % 2) * ((self.cell_size.0 - self.cell_offset.0) / 2)) as f32,
            (pos.y * ((self.cell_size.1 - self.cell_offset.1) / 2)) as f32,
        );
    }
}

/************************************************************
 * - System Functions
 */

fn setup_grid(mut commands: Commands) {
    commands.insert_resource(Grid::new(GRID_SIZE, CELL_SIZE, CELL_OFFSET));
}

/************************************************************
 * - Notes
 * W: usize,
 * H: usize,
 *
 * X => (i / W) + (i % H)
 * Y => (i / W)
 */
