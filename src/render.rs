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
                self.validate_render_layer_order(*order);

                *order
            }
            RenderLayer::Entity(order) => {
                self.validate_render_layer_order(*order);

                100 + *order
            }
        };
    }

    #[inline(always)]
    fn validate_render_layer_order(self, order: usize) {
        if !(order > 99) {
            return;
        }
        panic!("Invalid render order `{}` for `{:?}`.", order, self);
    }
}

/************************************************************
 * - System Functions
 */

fn setup_render_layer(mut query: Query<(&RenderLayer, &mut Transform), Added<RenderLayer>>) {
    for (layer, mut transform) in &mut query {
        let order = layer.get_render_order();

        transform.translation.z = order as f32;
    }
}
