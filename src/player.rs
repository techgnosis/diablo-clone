use macroquad::prelude::*;

use crate::camera::GameCamera;
use crate::combat::{ArmorType, Item, WeaponType};
use crate::inventory::Inventory;
use crate::world::World;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub health: i32,
    pub max_health: i32,
    pub weapon: WeaponType,
    pub armor: Option<ArmorType>,
    pub inventory: Inventory,
    pub attack_cooldown: f32,
    pub regen_timer: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            health: 50,
            max_health: 50,
            weapon: WeaponType::Sword,
            armor: None,
            inventory: Inventory::new(),
            attack_cooldown: 0.0,
            regen_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, _world: &World) {
        // Movement (5 tiles per second)
        let speed: f32 = 5.0;
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;

        // WASD movement - adjusted for isometric feel
        // W/Up = move up-left in world space
        // S/Down = move down-right in world space
        // A/Left = move down-left in world space
        // D/Right = move up-right in world space
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            dx -= 1.0;
            dy -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            dx += 1.0;
            dy += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dx -= 1.0;
            dy += 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dx += 1.0;
            dy -= 1.0;
        }

        // Normalize diagonal movement
        let len = (dx * dx + dy * dy).sqrt();
        if len > 0.0 {
            dx /= len;
            dy /= len;
        }

        self.x += dx * speed * dt;
        self.y += dy * speed * dt;

        // Attack cooldown
        if self.attack_cooldown > 0.0 {
            self.attack_cooldown -= dt;
        }

        // Health regeneration (1 HP per second)
        if self.health < self.max_health {
            self.regen_timer += dt;
            if self.regen_timer >= 1.0 {
                self.regen_timer -= 1.0;
                self.health = (self.health + 1).min(self.max_health);
            }
        }
    }

    pub fn can_attack(&self) -> bool {
        self.attack_cooldown <= 0.0
    }

    pub fn attack(&mut self) {
        self.attack_cooldown = 0.3; // 0.3 second cooldown
    }

    pub fn calculate_damage(&self) -> i32 {
        self.weapon.roll_damage()
    }

    pub fn take_damage(&mut self, raw_damage: i32) {
        let reduction = self.armor.as_ref().map(|a| a.damage_reduction()).unwrap_or(0);
        let damage = (raw_damage - reduction).max(0);
        self.health = (self.health - damage).max(0);
    }

    pub fn equip_item(&mut self, item: Item) -> Option<Item> {
        match item {
            Item::Weapon(w) => {
                let old = Item::Weapon(self.weapon.clone());
                self.weapon = w;
                Some(old)
            }
            Item::Armor(a) => {
                let old = self.armor.take().map(Item::Armor);
                self.armor = Some(a);
                old
            }
        }
    }

    pub fn draw(&self, camera: &GameCamera) {
        let (screen_x, screen_y) = camera.world_to_screen(self.x, self.y);

        // Draw player as a simple shape
        // Base body color depends on armor
        let body_color = match &self.armor {
            None => Color::from_rgba(200, 150, 100, 255), // Skin tone - no armor
            Some(ArmorType::Leather) => Color::from_rgba(139, 90, 43, 255), // Brown
            Some(ArmorType::Chainmail) => Color::from_rgba(150, 150, 160, 255), // Silver
            Some(ArmorType::Platemail) => Color::from_rgba(100, 100, 120, 255), // Dark steel
        };

        // Body (diamond shape for isometric)
        draw_poly(screen_x, screen_y - 10.0, 4, 20.0, 45.0, body_color);

        // Head (circle)
        draw_circle(screen_x, screen_y - 35.0, 10.0, Color::from_rgba(220, 180, 140, 255));

        // Weapon indicator (line extending from body)
        let weapon_color = match &self.weapon {
            WeaponType::Sword => LIGHTGRAY,
            WeaponType::Axe => Color::from_rgba(100, 80, 60, 255),
            WeaponType::Mace => DARKGRAY,
        };
        draw_line(
            screen_x + 15.0,
            screen_y - 10.0,
            screen_x + 30.0,
            screen_y - 25.0,
            3.0,
            weapon_color,
        );

        // Attack animation (flash when attacking)
        if self.attack_cooldown > 0.2 {
            draw_circle(screen_x + 25.0, screen_y - 15.0, 8.0, Color::from_rgba(255, 255, 200, 150));
        }
    }
}
