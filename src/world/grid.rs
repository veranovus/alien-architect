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

const GRID_OFFET: (i32, i32) = (-5, 35);
const CELL_SIZE: (u32, u32) = (30, 18);
const CELL_OFFSET: (u32, u32) = (2, 4);

/************************************************************
 * - Types
 */

#[derive(Debug, Resource)]
pub struct Grid {
    pub cell_offset: (u32, u32),
    pub size: (u32, u32),
    pub grid: Vec<usize>,
    cell_size: (u32, u32),
    offset: (i32, i32),
}

impl Grid {
    fn new(
        size: (u32, u32),
        offset: (i32, i32),
        cell_size: (u32, u32),
        cell_offset: (u32, u32),
    ) -> Self {
        Self {
            cell_size,
            cell_offset,
            size,
            offset,
            grid: vec![1; (size.0 * size.1) as usize],
        }
    }

    pub fn cell_order(&self, pos: UVec2) -> u32 {
        let index = (pos.y * self.size.0) + pos.x;

        return (self.size.0 * self.size.1)
            - (index % self.size.0 + (index / self.size.0 * self.size.0));
    }

    pub fn cell_to_world(&self, pos: UVec2) -> Vec2 {
        return Vec2::new(
            (pos.x * (self.cell_size.0 - self.cell_offset.0)) as f32
                + ((pos.y % 2) * ((self.cell_size.0 - self.cell_offset.0) / 2)) as f32
                + self.offset.0 as f32,
            (pos.y * ((self.cell_size.1 - self.cell_offset.1) / 2)) as f32 + self.offset.1 as f32,
        );
    }

    pub fn cell_center_offset(&self) -> Vec2 {
        return Vec2::new(
            (self.cell_offset.0 + ((self.cell_size.0 - self.cell_offset.1) / 2)) as f32,
            (self.cell_offset.1 + ((self.cell_size.1 - self.cell_offset.1) / 2)) as f32,
        );
    }
}

/************************************************************
 * - System Functions
 */

fn setup_grid(mut commands: Commands) {
    let mut g = Grid::new(GRID_SIZE, GRID_OFFET, CELL_SIZE, CELL_OFFSET);

    for i in 0..g.grid.len() {
        if (((i / g.size.0 as usize) % 2) == 0) && (i % g.size.0 as usize == 0) {
            g.grid[i] = 0;
        }
    }

    commands.insert_resource(g);
}

/************************************************************
 * - Notes
 * W: usize,
 * H: usize,
 *
 * X => (i / W) + (i % H)
 * Y => (i / W)
 *
 * Tiles -> cell_to_world
 * Objects -> cell_to_world + cell_offset
 */
