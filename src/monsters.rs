use macroquad::prelude::*;

use crate::camera::GameCamera;
use crate::combat::{calculate_damage, Item};
use crate::player::Player;
use crate::world::Terrain;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MonsterType {
    Goblin,
    Ogre,
    Orc,
    Wyrm,
    SnowGoblin,
    Yeti, // Renamed from SnowOgre
}

impl MonsterType {
    pub fn max_health(&self) -> i32 {
        match self {
            MonsterType::Goblin => 10,
            MonsterType::Ogre => 30,
            MonsterType::Orc => 20,
            MonsterType::Wyrm => 50,
            MonsterType::SnowGoblin => 10,
            MonsterType::Yeti => 30,
        }
    }

    pub fn base_damage(&self) -> i32 {
        match self {
            MonsterType::Goblin => 5,
            MonsterType::Ogre => 8,
            MonsterType::Orc => 6,
            MonsterType::Wyrm => 10,
            MonsterType::SnowGoblin => 5,
            MonsterType::Yeti => 8,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            MonsterType::Goblin => Color::from_rgba(80, 180, 80, 255),      // Green
            MonsterType::Ogre => Color::from_rgba(160, 120, 80, 255),       // Brown/tan
            MonsterType::Orc => Color::from_rgba(100, 120, 90, 255),        // Gray-green
            MonsterType::Wyrm => Color::from_rgba(200, 100, 50, 255),       // Red/orange
            MonsterType::SnowGoblin => Color::from_rgba(240, 240, 250, 255), // White
            MonsterType::Yeti => Color::from_rgba(245, 245, 255, 255),       // White
        }
    }

    pub fn size(&self) -> f32 {
        match self {
            MonsterType::Goblin | MonsterType::SnowGoblin => 12.0,
            MonsterType::Orc => 16.0,
            MonsterType::Ogre | MonsterType::Yeti | MonsterType::Wyrm => 22.0, // Wyrm same as Ogre
        }
    }

    pub fn for_terrain(terrain: Terrain) -> Vec<MonsterType> {
        match terrain {
            Terrain::Grass => vec![MonsterType::Goblin, MonsterType::Ogre],
            Terrain::Desert => vec![MonsterType::Orc, MonsterType::Wyrm],
            Terrain::Snow => vec![MonsterType::SnowGoblin, MonsterType::Yeti],
        }
    }

    pub fn random_for_terrain(terrain: Terrain) -> MonsterType {
        let types = Self::for_terrain(terrain);
        let idx = rand::gen_range(0, types.len());
        types[idx]
    }
}

pub struct Monster {
    pub x: f32,
    pub y: f32,
    pub health: i32,
    pub max_health: i32,
    pub monster_type: MonsterType,
    pub attack_cooldown: f32,
    pub speed: f32,
}

impl Monster {
    pub fn new(x: f32, y: f32, monster_type: MonsterType) -> Self {
        let max_health = monster_type.max_health();
        Self {
            x,
            y,
            health: max_health,
            max_health,
            monster_type,
            attack_cooldown: 0.0,
            speed: 4.0, // Slightly slower than player (5 tiles/sec)
        }
    }

    pub fn update(&mut self, dt: f32, player_x: f32, player_y: f32) {
        // Attack cooldown
        if self.attack_cooldown > 0.0 {
            self.attack_cooldown -= dt;
        }

        // Chase player if within detection range (10 tiles)
        let dx = player_x - self.x;
        let dy = player_y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist <= 10.0 && dist > 0.5 {
            // Move toward player
            let move_x = (dx / dist) * self.speed * dt;
            let move_y = (dy / dist) * self.speed * dt;
            self.x += move_x;
            self.y += move_y;
        }
    }

    pub fn can_attack(&self) -> bool {
        self.attack_cooldown <= 0.0
    }

    pub fn attack(&mut self) {
        self.attack_cooldown = 0.5; // Monsters attack every 0.5 seconds
    }

    pub fn calculate_damage(&self, player: &Player) -> i32 {
        calculate_damage(self.monster_type.base_damage(), player.armor.as_ref())
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    pub fn roll_loot(&self) -> Option<Item> {
        if rand::gen_range(0.0, 1.0) < 0.25 {
            // 25% drop rate
            Some(Item::random())
        } else {
            None
        }
    }

    pub fn draw(&self, camera: &GameCamera) {
        let (screen_x, screen_y) = camera.world_to_screen(self.x, self.y);

        let color = self.monster_type.color();
        let size = self.monster_type.size();

        match self.monster_type {
            MonsterType::Goblin | MonsterType::SnowGoblin => {
                self.draw_goblin(screen_x, screen_y, color, size);
            }
            MonsterType::Ogre => {
                self.draw_ogre(screen_x, screen_y, color, size);
            }
            MonsterType::Orc => {
                self.draw_orc(screen_x, screen_y, color, size);
            }
            MonsterType::Wyrm => {
                self.draw_wyrm(screen_x, screen_y, color, size);
            }
            MonsterType::Yeti => {
                self.draw_yeti(screen_x, screen_y, color, size);
            }
        }

        // Health bar above monster
        if self.health < self.max_health {
            let bar_width = size * 2.0;
            let bar_height = 4.0;
            let bar_x = screen_x - bar_width / 2.0;
            let bar_y = screen_y - size - 15.0;

            // Background
            draw_rectangle(bar_x, bar_y, bar_width, bar_height, DARKGRAY);

            // Health
            let health_pct = self.health as f32 / self.max_health as f32;
            draw_rectangle(bar_x, bar_y, bar_width * health_pct, bar_height, RED);
        }
    }

    /// Draw goblin: small humanoid with big sideways-pointing ears
    fn draw_goblin(&self, x: f32, y: f32, color: Color, size: f32) {
        // Body (diamond shape)
        draw_poly(x, y, 4, size, 45.0, color);
        draw_poly_lines(x, y, 4, size, 45.0, 1.0, BLACK);

        // Head
        let head_y = y - size - 5.0;
        let head_radius = size * 0.5;
        draw_circle(x, head_y, head_radius, color);
        draw_circle_lines(x, head_y, head_radius, 1.0, BLACK);

        // Big sideways ears (half head-width on each side)
        let ear_width = head_radius;  // Half head-width
        let ear_height = head_radius * 0.6;
        // Left ear - pointing outward/sideways
        draw_triangle(
            Vec2::new(x - head_radius, head_y),           // Inner point
            Vec2::new(x - head_radius - ear_width, head_y - ear_height * 0.3), // Outer top
            Vec2::new(x - head_radius - ear_width, head_y + ear_height * 0.5), // Outer bottom
            color,
        );
        draw_line(x - head_radius, head_y, x - head_radius - ear_width, head_y - ear_height * 0.3, 1.0, BLACK);
        draw_line(x - head_radius - ear_width, head_y - ear_height * 0.3, x - head_radius - ear_width, head_y + ear_height * 0.5, 1.0, BLACK);
        draw_line(x - head_radius - ear_width, head_y + ear_height * 0.5, x - head_radius, head_y, 1.0, BLACK);

        // Right ear - pointing outward/sideways
        draw_triangle(
            Vec2::new(x + head_radius, head_y),           // Inner point
            Vec2::new(x + head_radius + ear_width, head_y - ear_height * 0.3), // Outer top
            Vec2::new(x + head_radius + ear_width, head_y + ear_height * 0.5), // Outer bottom
            color,
        );
        draw_line(x + head_radius, head_y, x + head_radius + ear_width, head_y - ear_height * 0.3, 1.0, BLACK);
        draw_line(x + head_radius + ear_width, head_y - ear_height * 0.3, x + head_radius + ear_width, head_y + ear_height * 0.5, 1.0, BLACK);
        draw_line(x + head_radius + ear_width, head_y + ear_height * 0.5, x + head_radius, head_y, 1.0, BLACK);

        // Eyes
        let eye_y = head_y - 1.0;
        draw_circle(x - 2.0, eye_y, 1.5, BLACK);
        draw_circle(x + 2.0, eye_y, 1.5, BLACK);
    }

    /// Draw ogre: large humanoid with gray stone club over shoulder
    fn draw_ogre(&self, x: f32, y: f32, color: Color, size: f32) {
        // Body (diamond shape, bulky)
        draw_poly(x, y, 4, size, 45.0, color);
        draw_poly_lines(x, y, 4, size, 45.0, 1.5, BLACK);

        // Head
        let head_y = y - size - 6.0;
        let head_radius = size * 0.45;
        draw_circle(x, head_y, head_radius, color);
        draw_circle_lines(x, head_y, head_radius, 1.5, BLACK);

        // Eyes
        let eye_y = head_y - 1.0;
        draw_circle(x - 3.0, eye_y, 2.0, BLACK);
        draw_circle(x + 3.0, eye_y, 2.0, BLACK);

        // Gray stone club over shoulder (right side)
        let club_color = Color::from_rgba(120, 120, 130, 255); // Gray stone
        let club_x = x + size * 0.6;
        let club_y = head_y - 5.0;
        // Club handle
        draw_line(x + 5.0, y - size * 0.5, club_x, club_y, 3.0, Color::from_rgba(80, 60, 40, 255));
        // Club head (stone)
        draw_poly(club_x, club_y, 6, size * 0.4, 0.0, club_color);
        draw_poly_lines(club_x, club_y, 6, size * 0.4, 0.0, 1.0, BLACK);
    }

    /// Draw orc: medium humanoid with tusks from lower jaw
    fn draw_orc(&self, x: f32, y: f32, color: Color, size: f32) {
        // Body (diamond shape, muscular)
        draw_poly(x, y, 4, size, 45.0, color);
        draw_poly_lines(x, y, 4, size, 45.0, 1.5, BLACK);

        // Head
        let head_y = y - size - 5.0;
        let head_radius = size * 0.5;
        draw_circle(x, head_y, head_radius, color);
        draw_circle_lines(x, head_y, head_radius, 1.5, BLACK);

        // Eyes
        let eye_y = head_y - 2.0;
        draw_circle(x - 3.0, eye_y, 2.0, BLACK);
        draw_circle(x + 3.0, eye_y, 2.0, BLACK);

        // Tusks pointing UP from lower jaw (classic orc style)
        let tusk_color = Color::from_rgba(255, 255, 240, 255); // Ivory
        let jaw_y = head_y + head_radius * 0.5;
        // Left tusk
        draw_triangle(
            Vec2::new(x - 4.0, jaw_y),           // Base left
            Vec2::new(x - 2.0, jaw_y),           // Base right
            Vec2::new(x - 3.0, jaw_y - 8.0),     // Tip pointing up
            tusk_color,
        );
        draw_line(x - 4.0, jaw_y, x - 3.0, jaw_y - 8.0, 1.0, BLACK);
        draw_line(x - 2.0, jaw_y, x - 3.0, jaw_y - 8.0, 1.0, BLACK);
        // Right tusk
        draw_triangle(
            Vec2::new(x + 2.0, jaw_y),           // Base left
            Vec2::new(x + 4.0, jaw_y),           // Base right
            Vec2::new(x + 3.0, jaw_y - 8.0),     // Tip pointing up
            tusk_color,
        );
        draw_line(x + 2.0, jaw_y, x + 3.0, jaw_y - 8.0, 1.0, BLACK);
        draw_line(x + 4.0, jaw_y, x + 3.0, jaw_y - 8.0, 1.0, BLACK);
    }

    /// Draw wyrm: small dragon form with 4 legs, bat-style wings, tail
    fn draw_wyrm(&self, x: f32, y: f32, color: Color, size: f32) {
        // Dragon body (horizontal oval-ish shape)
        draw_poly(x, y, 6, size * 0.8, 0.0, color);
        draw_poly_lines(x, y, 6, size * 0.8, 0.0, 1.5, BLACK);

        // Head (in front, slightly raised)
        let head_x = x + size * 0.7;
        let head_y = y - size * 0.3;
        let head_radius = size * 0.35;
        draw_circle(head_x, head_y, head_radius, color);
        draw_circle_lines(head_x, head_y, head_radius, 1.0, BLACK);

        // Eye
        draw_circle(head_x + 2.0, head_y - 2.0, 2.0, Color::from_rgba(255, 200, 0, 255)); // Yellow dragon eye
        draw_circle(head_x + 2.0, head_y - 2.0, 1.0, BLACK); // Pupil

        // Neck connecting head to body
        draw_line(x + size * 0.3, y - size * 0.2, head_x - head_radius, head_y, 4.0, color);

        // Tail (behind)
        let tail_x = x - size * 0.9;
        let tail_y = y + size * 0.2;
        draw_line(x - size * 0.4, y, tail_x, tail_y, 4.0, color);
        draw_line(x - size * 0.4, y, tail_x, tail_y, 1.0, BLACK);
        // Tail tip
        draw_triangle(
            Vec2::new(tail_x, tail_y - 3.0),
            Vec2::new(tail_x, tail_y + 3.0),
            Vec2::new(tail_x - 8.0, tail_y),
            color,
        );

        // Four legs (small)
        let leg_color = color;
        // Front legs
        draw_line(x + size * 0.2, y + size * 0.3, x + size * 0.3, y + size * 0.7, 2.0, leg_color);
        draw_line(x + size * 0.1, y + size * 0.3, x, y + size * 0.7, 2.0, leg_color);
        // Back legs
        draw_line(x - size * 0.2, y + size * 0.3, x - size * 0.1, y + size * 0.7, 2.0, leg_color);
        draw_line(x - size * 0.3, y + size * 0.3, x - size * 0.4, y + size * 0.7, 2.0, leg_color);

        // Bat-style membrane wings
        let wing_color = Color::from_rgba(180, 80, 40, 200); // Darker membrane
        // Left wing
        draw_triangle(
            Vec2::new(x - size * 0.2, y - size * 0.2),  // Attachment point
            Vec2::new(x - size * 1.0, y - size * 0.8),  // Wing tip top
            Vec2::new(x - size * 0.8, y + size * 0.1),  // Wing tip bottom
            wing_color,
        );
        draw_line(x - size * 0.2, y - size * 0.2, x - size * 1.0, y - size * 0.8, 1.0, BLACK);
        draw_line(x - size * 0.2, y - size * 0.2, x - size * 0.8, y + size * 0.1, 1.0, BLACK);
        // Wing membrane lines
        draw_line(x - size * 0.2, y - size * 0.2, x - size * 0.9, y - size * 0.4, 1.0, Color::from_rgba(100, 50, 30, 150));

        // Right wing
        draw_triangle(
            Vec2::new(x + size * 0.2, y - size * 0.2),  // Attachment point
            Vec2::new(x + size * 1.0, y - size * 0.8),  // Wing tip top
            Vec2::new(x + size * 0.8, y + size * 0.1),  // Wing tip bottom
            wing_color,
        );
        draw_line(x + size * 0.2, y - size * 0.2, x + size * 1.0, y - size * 0.8, 1.0, BLACK);
        draw_line(x + size * 0.2, y - size * 0.2, x + size * 0.8, y + size * 0.1, 1.0, BLACK);
        // Wing membrane lines
        draw_line(x + size * 0.2, y - size * 0.2, x + size * 0.9, y - size * 0.4, 1.0, Color::from_rgba(100, 50, 30, 150));
    }

    /// Draw yeti: large hairy humanoid with visible claws, no weapon
    fn draw_yeti(&self, x: f32, y: f32, color: Color, size: f32) {
        // Hairy body - use jagged polygon to suggest fur
        // Draw multiple overlapping shapes for furry effect
        let fur_dark = Color::from_rgba(220, 220, 230, 255);

        // Main body (diamond with fur tufts)
        draw_poly(x, y, 4, size, 45.0, color);
        // Fur tufts around body
        for i in 0..8 {
            let angle = (i as f32) * std::f32::consts::PI / 4.0;
            let tuft_x = x + angle.cos() * size * 0.9;
            let tuft_y = y + angle.sin() * size * 0.9;
            draw_circle(tuft_x, tuft_y, 4.0, fur_dark);
        }
        draw_poly_lines(x, y, 4, size, 45.0, 1.5, Color::from_rgba(180, 180, 190, 255));

        // Head (with fur)
        let head_y = y - size - 6.0;
        let head_radius = size * 0.5;
        draw_circle(x, head_y, head_radius, color);
        // Fur on head
        for i in 0..6 {
            let angle = (i as f32) * std::f32::consts::PI / 3.0 - std::f32::consts::PI / 2.0;
            let tuft_x = x + angle.cos() * head_radius * 0.8;
            let tuft_y = head_y + angle.sin() * head_radius * 0.8;
            draw_circle(tuft_x, tuft_y, 3.0, fur_dark);
        }
        draw_circle_lines(x, head_y, head_radius, 1.5, Color::from_rgba(180, 180, 190, 255));

        // Eyes (small, menacing)
        let eye_y = head_y - 2.0;
        draw_circle(x - 4.0, eye_y, 2.5, Color::from_rgba(50, 50, 80, 255)); // Dark eyes
        draw_circle(x + 4.0, eye_y, 2.5, Color::from_rgba(50, 50, 80, 255));
        // Eye shine
        draw_circle(x - 3.0, eye_y - 1.0, 1.0, WHITE);
        draw_circle(x + 5.0, eye_y - 1.0, 1.0, WHITE);

        // Visible claws on hands (arms extending from body)
        let claw_color = Color::from_rgba(60, 60, 70, 255); // Dark claws
        // Left arm and claws
        let left_hand_x = x - size * 0.8;
        let left_hand_y = y + size * 0.2;
        draw_line(x - size * 0.4, y, left_hand_x, left_hand_y, 4.0, color);
        // Claws
        draw_line(left_hand_x, left_hand_y, left_hand_x - 6.0, left_hand_y + 4.0, 2.0, claw_color);
        draw_line(left_hand_x, left_hand_y, left_hand_x - 4.0, left_hand_y + 6.0, 2.0, claw_color);
        draw_line(left_hand_x, left_hand_y, left_hand_x - 2.0, left_hand_y + 7.0, 2.0, claw_color);

        // Right arm and claws
        let right_hand_x = x + size * 0.8;
        let right_hand_y = y + size * 0.2;
        draw_line(x + size * 0.4, y, right_hand_x, right_hand_y, 4.0, color);
        // Claws
        draw_line(right_hand_x, right_hand_y, right_hand_x + 6.0, right_hand_y + 4.0, 2.0, claw_color);
        draw_line(right_hand_x, right_hand_y, right_hand_x + 4.0, right_hand_y + 6.0, 2.0, claw_color);
        draw_line(right_hand_x, right_hand_y, right_hand_x + 2.0, right_hand_y + 7.0, 2.0, claw_color);
    }
}
