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
    SnowOgre,
}

impl MonsterType {
    pub fn max_health(&self) -> i32 {
        match self {
            MonsterType::Goblin => 10,
            MonsterType::Ogre => 30,
            MonsterType::Orc => 20,
            MonsterType::Wyrm => 50,
            MonsterType::SnowGoblin => 10,
            MonsterType::SnowOgre => 30,
        }
    }

    pub fn base_damage(&self) -> i32 {
        match self {
            MonsterType::Goblin => 5,
            MonsterType::Ogre => 8,
            MonsterType::Orc => 6,
            MonsterType::Wyrm => 10,
            MonsterType::SnowGoblin => 5,
            MonsterType::SnowOgre => 8,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            MonsterType::Goblin => Color::from_rgba(80, 180, 80, 255),      // Green
            MonsterType::Ogre => Color::from_rgba(120, 80, 60, 255),        // Brown
            MonsterType::Orc => Color::from_rgba(100, 140, 80, 255),        // Olive
            MonsterType::Wyrm => Color::from_rgba(180, 140, 60, 255),       // Gold/tan
            MonsterType::SnowGoblin => Color::from_rgba(150, 200, 220, 255), // Ice blue
            MonsterType::SnowOgre => Color::from_rgba(200, 200, 220, 255),   // Pale
        }
    }

    pub fn size(&self) -> f32 {
        match self {
            MonsterType::Goblin | MonsterType::SnowGoblin => 12.0,
            MonsterType::Orc => 16.0,
            MonsterType::Ogre | MonsterType::SnowOgre => 22.0,
            MonsterType::Wyrm => 25.0,
        }
    }

    pub fn for_terrain(terrain: Terrain) -> Vec<MonsterType> {
        match terrain {
            Terrain::Grass => vec![MonsterType::Goblin, MonsterType::Ogre],
            Terrain::Desert => vec![MonsterType::Orc, MonsterType::Wyrm],
            Terrain::Snow => vec![MonsterType::SnowGoblin, MonsterType::SnowOgre],
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

        // Draw monster body (hexagon for variety)
        draw_poly(screen_x, screen_y, 6, size, 0.0, color);

        // Outline
        draw_poly_lines(screen_x, screen_y, 6, size, 0.0, 1.5, BLACK);

        // Health bar above monster
        if self.health < self.max_health {
            let bar_width = size * 2.0;
            let bar_height = 4.0;
            let bar_x = screen_x - bar_width / 2.0;
            let bar_y = screen_y - size - 10.0;

            // Background
            draw_rectangle(bar_x, bar_y, bar_width, bar_height, DARKGRAY);

            // Health
            let health_pct = self.health as f32 / self.max_health as f32;
            draw_rectangle(bar_x, bar_y, bar_width * health_pct, bar_height, RED);
        }
    }
}
