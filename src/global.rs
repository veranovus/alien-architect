// WINDOW

pub mod window {
    use bevy::window::PresentMode;

    pub const RESOLUTION: (usize, usize) = (800, 720);
    pub const TITLE: &str = "GBJam #11";
    pub const PRESENT_MODE: PresentMode = PresentMode::Fifo;
    pub const RESIZABLE: bool = false;
}

// APP

pub mod app {
    pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
}
