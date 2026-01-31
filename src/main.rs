use macroquad::prelude::*;
use std::collections::HashSet;

mod camera;
mod combat;
mod inventory;
mod monsters;
mod player;
mod ui;
mod world;

use camera::GameCamera;
use monsters::{Monster, MonsterType};
use player::Player;
use world::World;

pub enum GameState {
    Playing,
    Inventory,
    GameOver,
}

/// Floating text that rises and fades out
pub struct FloatingText {
    text: String,
    world_x: f32,
    world_y: f32,
    offset_y: f32,    // Vertical offset that increases over time
    lifetime: f32,    // Remaining lifetime in seconds
    max_lifetime: f32,
}

impl FloatingText {
    pub fn new(text: String, world_x: f32, world_y: f32) -> Self {
        Self {
            text,
            world_x,
            world_y,
            offset_y: 0.0,
            lifetime: 1.0,
            max_lifetime: 1.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.lifetime -= dt;
        self.offset_y += 30.0 * dt; // Rise upward
    }

    pub fn is_expired(&self) -> bool {
        self.lifetime <= 0.0
    }

    pub fn draw(&self, camera: &GameCamera) {
        let (screen_x, screen_y) = camera.world_to_screen(self.world_x, self.world_y);
        let alpha = (self.lifetime / self.max_lifetime * 255.0) as u8;

        let color = Color::from_rgba(255, 255, 100, alpha); // Yellow text
        let font_size = 18.0;

        let text_dims = measure_text(&self.text, None, font_size as u16, 1.0);
        draw_text(
            &self.text,
            screen_x - text_dims.width / 2.0,
            screen_y - 50.0 - self.offset_y,
            font_size,
            color,
        );
    }
}

pub struct Game {
    state: GameState,
    player: Player,
    world: World,
    camera: GameCamera,
    monsters: Vec<Monster>,
    ground_items: Vec<inventory::GroundItem>,
    spawned_chunks: HashSet<(i32, i32)>,
    floating_texts: Vec<FloatingText>,
}

impl Game {
    const CHUNK_SIZE: i32 = 8;
    const SPAWN_RANGE: i32 = 3; // Spawn in chunks within this range

    pub fn new() -> Self {
        let player = Player::new(0.0, 0.0);
        let world = World::new(12345); // Seed for noise
        let camera = GameCamera::new();

        let mut game = Self {
            state: GameState::Playing,
            player,
            world,
            camera,
            monsters: Vec::new(),
            ground_items: Vec::new(),
            spawned_chunks: HashSet::new(),
            floating_texts: Vec::new(),
        };

        // Initial monster spawn around player
        game.spawn_monsters_around_player();
        game
    }

    fn spawn_monsters_around_player(&mut self) {
        let player_chunk_x = (self.player.x / Self::CHUNK_SIZE as f32).floor() as i32;
        let player_chunk_y = (self.player.y / Self::CHUNK_SIZE as f32).floor() as i32;

        for cy in (player_chunk_y - Self::SPAWN_RANGE)..=(player_chunk_y + Self::SPAWN_RANGE) {
            for cx in (player_chunk_x - Self::SPAWN_RANGE)..=(player_chunk_x + Self::SPAWN_RANGE) {
                self.spawn_chunk(cx, cy);
            }
        }
    }

    fn spawn_chunk(&mut self, chunk_x: i32, chunk_y: i32) {
        if self.spawned_chunks.contains(&(chunk_x, chunk_y)) {
            return;
        }
        self.spawned_chunks.insert((chunk_x, chunk_y));

        // Use deterministic random based on chunk coords
        let hash = ((chunk_x.wrapping_mul(374761393)) ^ (chunk_y.wrapping_mul(668265263))) as u32;

        // ~20% chance to spawn a monster in this chunk
        if hash % 5 != 0 {
            return;
        }

        // Get center of chunk
        let world_x = (chunk_x * Self::CHUNK_SIZE) as f32 + (Self::CHUNK_SIZE as f32 / 2.0);
        let world_y = (chunk_y * Self::CHUNK_SIZE) as f32 + (Self::CHUNK_SIZE as f32 / 2.0);

        // Add some randomness to position within chunk
        let offset_x = ((hash >> 8) % (Self::CHUNK_SIZE as u32)) as f32 - (Self::CHUNK_SIZE as f32 / 2.0);
        let offset_y = ((hash >> 16) % (Self::CHUNK_SIZE as u32)) as f32 - (Self::CHUNK_SIZE as f32 / 2.0);

        let spawn_x = world_x + offset_x;
        let spawn_y = world_y + offset_y;

        // Don't spawn too close to player start
        if spawn_x.abs() < 5.0 && spawn_y.abs() < 5.0 {
            return;
        }

        // Get terrain and spawn appropriate monster
        let terrain = self.world.get_terrain_at(spawn_x, spawn_y);
        let monster_type = MonsterType::random_for_terrain(terrain);

        self.monsters.push(Monster::new(spawn_x, spawn_y, monster_type));
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::Playing => self.update_playing(),
            GameState::Inventory => self.update_inventory(),
            GameState::GameOver => self.update_game_over(),
        }
    }

    fn update_playing(&mut self) {
        // Toggle inventory
        if is_key_pressed(KeyCode::I) {
            self.state = GameState::Inventory;
            return;
        }

        let dt = get_frame_time();

        // Update player
        self.player.update(dt, &self.world);

        // Update camera to follow player
        self.camera.follow(self.player.x, self.player.y, dt);

        // Spawn monsters as player explores
        self.spawn_monsters_around_player();

        // Update monsters
        for monster in &mut self.monsters {
            monster.update(dt, self.player.x, self.player.y);
        }

        // Handle combat
        self.handle_combat();

        // Check for item pickup
        self.check_item_pickup();

        // Update floating texts
        for text in &mut self.floating_texts {
            text.update(dt);
        }
        self.floating_texts.retain(|t| !t.is_expired());

        // Check player death
        if self.player.health <= 0 {
            self.state = GameState::GameOver;
        }
    }

    fn update_inventory(&mut self) {
        if is_key_pressed(KeyCode::I) || is_key_pressed(KeyCode::Escape) {
            self.state = GameState::Playing;
            return;
        }

        // Handle inventory slot clicks for equipping
        if let Some(slot_idx) = inventory::get_clicked_slot() {
            if let Some(item) = self.player.inventory.remove_item(slot_idx) {
                // Equip the item and get back the old equipped item
                if let Some(old_item) = self.player.equip_item(item) {
                    // Put old item back in inventory
                    self.player.inventory.add_item(old_item);
                }
            }
        }
    }

    fn update_game_over(&mut self) {
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
            // Restart game
            *self = Game::new();
        }
    }

    fn handle_combat(&mut self) {
        // Player attacking monsters
        if is_mouse_button_pressed(MouseButton::Left) && self.player.can_attack() {
            self.player.attack();

            let attack_range = 1.0; // 1 tile
            let mut dead_indices = Vec::new();

            for (i, monster) in self.monsters.iter_mut().enumerate() {
                let dx = monster.x - self.player.x;
                let dy = monster.y - self.player.y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist <= attack_range {
                    let damage = self.player.calculate_damage();
                    monster.take_damage(damage);

                    if monster.health <= 0 {
                        dead_indices.push(i);
                    }
                }
            }

            // Remove dead monsters and spawn loot
            for i in dead_indices.into_iter().rev() {
                let monster = self.monsters.remove(i);
                if let Some(item) = monster.roll_loot() {
                    self.ground_items.push(inventory::GroundItem {
                        x: monster.x,
                        y: monster.y,
                        item,
                    });
                }
            }
        }

        // Monsters attacking player
        for monster in &mut self.monsters {
            if monster.can_attack() {
                let dx = self.player.x - monster.x;
                let dy = self.player.y - monster.y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist <= 1.0 {
                    monster.attack();
                    let damage = monster.calculate_damage(&self.player);
                    self.player.take_damage(damage);
                }
            }
        }
    }

    fn check_item_pickup(&mut self) {
        let pickup_range = 0.5;
        let mut picked_items: Vec<(usize, String)> = Vec::new();

        for (i, ground_item) in self.ground_items.iter().enumerate() {
            let dx = ground_item.x - self.player.x;
            let dy = ground_item.y - self.player.y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist <= pickup_range {
                if self.player.inventory.add_item(ground_item.item.clone()) {
                    let item_name = ground_item.item.name().to_string();
                    picked_items.push((i, item_name));
                }
            }
        }

        for (i, item_name) in picked_items.into_iter().rev() {
            self.ground_items.remove(i);
            // Spawn floating text
            let text = format!("Picked up {}!", item_name);
            self.floating_texts.push(FloatingText::new(
                text,
                self.player.x,
                self.player.y,
            ));
        }
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(30, 30, 40, 255));

        match self.state {
            GameState::Playing => self.draw_playing(),
            GameState::Inventory => {
                self.draw_playing(); // Draw game behind
                self.draw_inventory();
            }
            GameState::GameOver => self.draw_game_over(),
        }

        // Always draw UI
        ui::draw_health_bar(self.player.health, self.player.max_health);
    }

    fn draw_playing(&self) {
        // Draw world
        self.world.draw(&self.camera);

        // Draw ground items
        for item in &self.ground_items {
            let (screen_x, screen_y) = self.camera.world_to_screen(item.x, item.y);
            inventory::draw_ground_item(item, screen_x, screen_y);
        }

        // Draw monsters
        for monster in &self.monsters {
            monster.draw(&self.camera);
        }

        // Draw player
        self.player.draw(&self.camera);

        // Draw floating texts
        for text in &self.floating_texts {
            text.draw(&self.camera);
        }
    }

    fn draw_inventory(&self) {
        inventory::draw_inventory_screen(&self.player);
    }

    fn draw_game_over(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::from_rgba(0, 0, 0, 200));

        let text = "GAME OVER";
        let font_size = 64.0;
        let text_dims = measure_text(text, None, font_size as u16, 1.0);
        draw_text(
            text,
            screen_w / 2.0 - text_dims.width / 2.0,
            screen_h / 2.0,
            font_size,
            RED,
        );

        let restart_text = "Press SPACE or ENTER to restart";
        let restart_dims = measure_text(restart_text, None, 24, 1.0);
        draw_text(
            restart_text,
            screen_w / 2.0 - restart_dims.width / 2.0,
            screen_h / 2.0 + 50.0,
            24.0,
            WHITE,
        );
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Diablo Clone".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();

        next_frame().await
    }
}
