use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

use crate::camera::{GameCamera, TILE_HEIGHT, TILE_WIDTH};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Terrain {
    Grass,
    Desert,
    Snow,
}

impl Terrain {
    pub fn base_color(&self) -> Color {
        match self {
            Terrain::Grass => Color::from_rgba(80, 160, 80, 255),
            Terrain::Desert => Color::from_rgba(210, 180, 140, 255),
            Terrain::Snow => Color::from_rgba(240, 245, 255, 255),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Decoration {
    // Grass decorations
    Rock,
    Tree,
    // Desert decorations
    Cactus,
    Bones,
    // Snow decorations
    SnowyRock,
    SnowyTree,
}

impl Decoration {
    pub fn draw(&self, screen_x: f32, screen_y: f32) {
        match self {
            Decoration::Rock => {
                draw_poly(screen_x, screen_y - 5.0, 5, 8.0, 0.0, GRAY);
            }
            Decoration::Tree => {
                // Trunk
                draw_rectangle(screen_x - 3.0, screen_y - 20.0, 6.0, 20.0, Color::from_rgba(101, 67, 33, 255));
                // Foliage
                draw_poly(screen_x, screen_y - 35.0, 3, 15.0, 180.0, Color::from_rgba(34, 139, 34, 255));
            }
            Decoration::Cactus => {
                // Main body
                draw_rectangle(screen_x - 4.0, screen_y - 25.0, 8.0, 25.0, Color::from_rgba(60, 140, 60, 255));
                // Arms
                draw_rectangle(screen_x - 12.0, screen_y - 20.0, 8.0, 5.0, Color::from_rgba(60, 140, 60, 255));
                draw_rectangle(screen_x + 4.0, screen_y - 15.0, 8.0, 5.0, Color::from_rgba(60, 140, 60, 255));
            }
            Decoration::Bones => {
                draw_line(screen_x - 8.0, screen_y - 2.0, screen_x + 8.0, screen_y - 2.0, 3.0, Color::from_rgba(230, 230, 210, 255));
                draw_line(screen_x - 5.0, screen_y - 6.0, screen_x + 5.0, screen_y + 2.0, 2.0, Color::from_rgba(230, 230, 210, 255));
            }
            Decoration::SnowyRock => {
                draw_poly(screen_x, screen_y - 5.0, 5, 8.0, 0.0, Color::from_rgba(180, 180, 190, 255));
                // Snow cap
                draw_poly(screen_x, screen_y - 8.0, 5, 5.0, 0.0, WHITE);
            }
            Decoration::SnowyTree => {
                // Trunk
                draw_rectangle(screen_x - 3.0, screen_y - 20.0, 6.0, 20.0, Color::from_rgba(101, 67, 33, 255));
                // Snow-covered foliage
                draw_poly(screen_x, screen_y - 35.0, 3, 15.0, 180.0, Color::from_rgba(220, 240, 220, 255));
            }
        }
    }
}

pub struct WorldTile {
    pub terrain: Terrain,
    pub decoration: Option<Decoration>,
}

pub struct World {
    noise: Perlin,
    decoration_noise: Perlin,
    seed: u32,
}

impl World {
    pub fn new(seed: u32) -> Self {
        Self {
            noise: Perlin::new(seed),
            decoration_noise: Perlin::new(seed.wrapping_add(1000)),
            seed,
        }
    }

    pub fn get_terrain_at(&self, x: f32, y: f32) -> Terrain {
        let scale = 0.05; // Controls biome size
        let noise_val = self.noise.get([x as f64 * scale, y as f64 * scale]);

        if noise_val < -0.33 {
            Terrain::Snow
        } else if noise_val < 0.33 {
            Terrain::Grass
        } else {
            Terrain::Desert
        }
    }

    fn get_blended_color(&self, x: f32, y: f32) -> Color {
        let scale = 0.05;
        let noise_val = self.noise.get([x as f64 * scale, y as f64 * scale]) as f32;

        // Get base colors
        let snow_color = Terrain::Snow.base_color();
        let grass_color = Terrain::Grass.base_color();
        let desert_color = Terrain::Desert.base_color();

        // Blend based on noise value with smooth transitions
        let blend_width = 0.15; // Width of transition zone

        if noise_val < -0.33 - blend_width {
            snow_color
        } else if noise_val < -0.33 + blend_width {
            // Snow to grass blend
            let t = (noise_val - (-0.33 - blend_width)) / (2.0 * blend_width);
            lerp_color(snow_color, grass_color, t)
        } else if noise_val < 0.33 - blend_width {
            grass_color
        } else if noise_val < 0.33 + blend_width {
            // Grass to desert blend
            let t = (noise_val - (0.33 - blend_width)) / (2.0 * blend_width);
            lerp_color(grass_color, desert_color, t)
        } else {
            desert_color
        }
    }

    fn get_decoration_at(&self, x: i32, y: i32) -> Option<Decoration> {
        let dec_noise = self.decoration_noise.get([x as f64 * 0.5, y as f64 * 0.5]);

        // Only spawn decorations ~10% of tiles
        if dec_noise < 0.7 {
            return None;
        }

        let terrain = self.get_terrain_at(x as f32, y as f32);
        let hash = ((x.wrapping_mul(374761393) ^ y.wrapping_mul(668265263)) as u32).wrapping_add(self.seed);

        match terrain {
            Terrain::Grass => {
                if hash % 2 == 0 {
                    Some(Decoration::Rock)
                } else {
                    Some(Decoration::Tree)
                }
            }
            Terrain::Desert => {
                if hash % 2 == 0 {
                    Some(Decoration::Cactus)
                } else {
                    Some(Decoration::Bones)
                }
            }
            Terrain::Snow => {
                if hash % 2 == 0 {
                    Some(Decoration::SnowyRock)
                } else {
                    Some(Decoration::SnowyTree)
                }
            }
        }
    }

    pub fn draw(&self, camera: &GameCamera) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Calculate visible tile range
        let tiles_x = (screen_w / TILE_WIDTH) as i32 + 4;
        let tiles_y = (screen_h / TILE_HEIGHT) as i32 + 4;

        let cam_tile_x = camera.x as i32;
        let cam_tile_y = camera.y as i32;

        // Draw tiles
        for dy in -tiles_y..=tiles_y {
            for dx in -tiles_x..=tiles_x {
                let world_x = cam_tile_x + dx;
                let world_y = cam_tile_y + dy;

                let (screen_x, screen_y) = camera.world_to_screen(world_x as f32, world_y as f32);

                // Skip if off screen
                if screen_x < -TILE_WIDTH || screen_x > screen_w + TILE_WIDTH
                    || screen_y < -TILE_HEIGHT || screen_y > screen_h + TILE_HEIGHT
                {
                    continue;
                }

                // Get blended terrain color
                let color = self.get_blended_color(world_x as f32, world_y as f32);

                // Draw isometric diamond tile
                draw_isometric_tile(screen_x, screen_y, color);
            }
        }

        // Draw decorations (second pass for proper layering)
        for dy in -tiles_y..=tiles_y {
            for dx in -tiles_x..=tiles_x {
                let world_x = cam_tile_x + dx;
                let world_y = cam_tile_y + dy;

                let (screen_x, screen_y) = camera.world_to_screen(world_x as f32, world_y as f32);

                if screen_x < -TILE_WIDTH * 2.0 || screen_x > screen_w + TILE_WIDTH * 2.0
                    || screen_y < -TILE_HEIGHT * 2.0 || screen_y > screen_h + TILE_HEIGHT * 2.0
                {
                    continue;
                }

                if let Some(decoration) = self.get_decoration_at(world_x, world_y) {
                    decoration.draw(screen_x, screen_y);
                }
            }
        }
    }
}

fn draw_isometric_tile(x: f32, y: f32, color: Color) {
    let hw = TILE_WIDTH / 2.0;
    let hh = TILE_HEIGHT / 2.0;

    // Draw diamond shape
    draw_triangle(
        Vec2::new(x, y - hh),      // Top
        Vec2::new(x - hw, y),      // Left
        Vec2::new(x, y + hh),      // Bottom
        color,
    );
    draw_triangle(
        Vec2::new(x, y - hh),      // Top
        Vec2::new(x + hw, y),      // Right
        Vec2::new(x, y + hh),      // Bottom
        color,
    );

    // Subtle grid lines
    let line_color = Color::from_rgba(0, 0, 0, 20);
    draw_line(x, y - hh, x - hw, y, 1.0, line_color);
    draw_line(x, y - hh, x + hw, y, 1.0, line_color);
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::from_rgba(
        (a.r * 255.0 * (1.0 - t) + b.r * 255.0 * t) as u8,
        (a.g * 255.0 * (1.0 - t) + b.g * 255.0 * t) as u8,
        (a.b * 255.0 * (1.0 - t) + b.b * 255.0 * t) as u8,
        255,
    )
}
