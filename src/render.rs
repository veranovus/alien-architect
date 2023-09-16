use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, setup_render_layer);
    }
}

/************************************************************
 * - Types
 */

#[derive(Debug, Clone, Copy, Component)]
pub enum RenderLayer {
    Tile(usize),
    Entity(usize),
}

impl RenderLayer {
    pub fn get_render_order(&self) -> usize {
        return match self {
            RenderLayer::Tile(order) => {
                if !validate_render_layer_order(*order) {
                    panic!("Invalid render order `{}` for `{:?}`.", *order, self);
                }

                *order
            }
            RenderLayer::Entity(order) => {
                if !validate_render_layer_order(*order) {
                    panic!("Invalid render order `{}` for `{:?}`.", *order, self);
                }

                100 + *order
            }
        };
    }
}

/************************************************************
 * - System Functions
 */

fn setup_render_layer(mut query: Query<(&mut Transform, &RenderLayer), Added<RenderLayer>>) {
    for (mut transform, layer) in &mut query {
        let order = layer.get_render_order();

        transform.translation.z = order as f32;
    }
}

/************************************************************
 * - Helper Functions
 */

#[inline(always)]
fn validate_render_layer_order(order: usize) -> bool {
    return !(order < 0 || order > 99);
}
