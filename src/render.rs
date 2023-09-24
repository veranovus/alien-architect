/************************************************************
 * - Constants
 */

pub const RENDER_LAYER: [u32; 5] = [0, 100, 200, 300, 400];

/************************************************************
 * - Types
 */

#[derive(Debug, Clone, Copy)]
pub enum RenderLayer {
    Tile = 0,
    Entity,
    UFO,
    UI,
    Overlay,
}
