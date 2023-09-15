// WINDOW

pub mod window {
    use bevy::window::PresentMode;

    pub const VIEWPORT_RESOLUTION: (usize, usize) = (160, 144);
    pub const SCALE_FACTOR: usize = 5;
    pub const RESOLUTION: (usize, usize) = (
        VIEWPORT_RESOLUTION.0 * SCALE_FACTOR,
        VIEWPORT_RESOLUTION.1 * SCALE_FACTOR,
    );
    pub const TITLE: &str = "GBJam #11";
    pub const PRESENT_MODE: PresentMode = PresentMode::Fifo;
    pub const RESIZABLE: bool = false;
}

// APP

pub mod app {
    pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
}
