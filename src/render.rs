/************************************************************
 * - Constants
 */

pub const RENDER_LAYER: [u32; 4] = [0, 100, 200, 300];

/************************************************************
 * - Types
 */

#[derive(Debug, Clone, Copy)]
pub enum RenderLayer {
    Tile = 0,
    Entity,
    UFO,
    UI,
}
