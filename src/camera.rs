use macroquad::prelude::*;

pub const TILE_WIDTH: f32 = 64.0;
pub const TILE_HEIGHT: f32 = 32.0;

pub struct GameCamera {
    pub x: f32,
    pub y: f32,
    pub lerp_speed: f32,
}

impl GameCamera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            lerp_speed: 5.0,
        }
    }

    pub fn follow(&mut self, target_x: f32, target_y: f32, dt: f32) {
        let lerp = 1.0 - (-self.lerp_speed * dt).exp();
        self.x += (target_x - self.x) * lerp;
        self.y += (target_y - self.y) * lerp;
    }

    /// Convert world coordinates to isometric screen coordinates
    pub fn world_to_screen(&self, world_x: f32, world_y: f32) -> (f32, f32) {
        let rel_x = world_x - self.x;
        let rel_y = world_y - self.y;

        // Isometric projection
        let iso_x = (rel_x - rel_y) * (TILE_WIDTH / 2.0);
        let iso_y = (rel_x + rel_y) * (TILE_HEIGHT / 2.0);

        // Center on screen
        let screen_x = screen_width() / 2.0 + iso_x;
        let screen_y = screen_height() / 2.0 + iso_y;

        (screen_x, screen_y)
    }

    /// Convert screen coordinates to world coordinates
    pub fn screen_to_world(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        let rel_screen_x = screen_x - screen_width() / 2.0;
        let rel_screen_y = screen_y - screen_height() / 2.0;

        // Inverse isometric projection
        let world_x = (rel_screen_x / (TILE_WIDTH / 2.0) + rel_screen_y / (TILE_HEIGHT / 2.0)) / 2.0;
        let world_y = (rel_screen_y / (TILE_HEIGHT / 2.0) - rel_screen_x / (TILE_WIDTH / 2.0)) / 2.0;

        (world_x + self.x, world_y + self.y)
    }
}
