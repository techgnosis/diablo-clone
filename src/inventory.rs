use macroquad::prelude::*;

use crate::combat::Item;
use crate::player::Player;

pub const INVENTORY_SIZE: usize = 8;

#[derive(Clone)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: Item) -> bool {
        if self.items.len() < INVENTORY_SIZE {
            self.items.push(item);
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn is_full(&self) -> bool {
        self.items.len() >= INVENTORY_SIZE
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}

pub struct GroundItem {
    pub x: f32,
    pub y: f32,
    pub item: Item,
}

pub fn draw_ground_item(ground_item: &GroundItem, screen_x: f32, screen_y: f32) {
    let color = match &ground_item.item {
        Item::Weapon(_) => ORANGE,
        Item::Armor(_) => SKYBLUE,
    };

    // Draw as a small diamond
    let size = 8.0;
    draw_poly(screen_x, screen_y, 4, size, 45.0, color);
    draw_poly_lines(screen_x, screen_y, 4, size, 45.0, 1.5, WHITE);
}

// Returns the index of clicked inventory slot, if any
pub fn get_clicked_slot() -> Option<usize> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    let (mouse_x, mouse_y) = mouse_position();
    let screen_w = screen_width();
    let screen_h = screen_height();

    let panel_w = 400.0;
    let panel_h = 500.0;
    let panel_x = screen_w / 2.0 - panel_w / 2.0;
    let panel_y = screen_h / 2.0 - panel_h / 2.0;

    let slot_size = 50.0;
    let slot_padding = 10.0;
    let slots_per_row = 4;
    let start_x = panel_x + 20.0;
    let start_y = panel_y + 200.0;

    for i in 0..INVENTORY_SIZE {
        let row = i / slots_per_row;
        let col = i % slots_per_row;
        let slot_x = start_x + col as f32 * (slot_size + slot_padding);
        let slot_y = start_y + row as f32 * (slot_size + slot_padding);

        if mouse_x >= slot_x
            && mouse_x <= slot_x + slot_size
            && mouse_y >= slot_y
            && mouse_y <= slot_y + slot_size
        {
            return Some(i);
        }
    }

    None
}

// Returns the index of hovered inventory slot, if any
fn get_hovered_slot() -> Option<usize> {
    let (mouse_x, mouse_y) = mouse_position();
    let screen_w = screen_width();
    let screen_h = screen_height();

    let panel_w = 400.0;
    let panel_h = 500.0;
    let panel_x = screen_w / 2.0 - panel_w / 2.0;
    let panel_y = screen_h / 2.0 - panel_h / 2.0;

    let slot_size = 50.0;
    let slot_padding = 10.0;
    let slots_per_row = 4;
    let start_x = panel_x + 20.0;
    let start_y = panel_y + 200.0;

    for i in 0..INVENTORY_SIZE {
        let row = i / slots_per_row;
        let col = i % slots_per_row;
        let slot_x = start_x + col as f32 * (slot_size + slot_padding);
        let slot_y = start_y + row as f32 * (slot_size + slot_padding);

        if mouse_x >= slot_x
            && mouse_x <= slot_x + slot_size
            && mouse_y >= slot_y
            && mouse_y <= slot_y + slot_size
        {
            return Some(i);
        }
    }

    None
}

pub fn draw_inventory_screen(player: &Player) {
    let screen_w = screen_width();
    let screen_h = screen_height();

    // Darken background
    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::from_rgba(0, 0, 0, 180));

    // Inventory panel
    let panel_w = 400.0;
    let panel_h = 500.0;
    let panel_x = screen_w / 2.0 - panel_w / 2.0;
    let panel_y = screen_h / 2.0 - panel_h / 2.0;

    draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::from_rgba(40, 40, 50, 255));
    draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, 2.0, WHITE);

    // Title
    draw_text("INVENTORY", panel_x + 20.0, panel_y + 35.0, 32.0, WHITE);

    // Equipment section
    draw_text("Equipped:", panel_x + 20.0, panel_y + 80.0, 20.0, GRAY);

    // Weapon slot
    let weapon_name = player.weapon.name();
    draw_text(
        &format!("Weapon: {}", weapon_name),
        panel_x + 30.0,
        panel_y + 110.0,
        18.0,
        ORANGE,
    );

    // Armor slot
    let armor_name = player.armor.as_ref().map(|a| a.name()).unwrap_or("None");
    draw_text(
        &format!("Armor: {}", armor_name),
        panel_x + 30.0,
        panel_y + 135.0,
        18.0,
        SKYBLUE,
    );

    // Inventory grid
    draw_text("Backpack (click to equip):", panel_x + 20.0, panel_y + 180.0, 20.0, GRAY);

    let slot_size = 50.0;
    let slot_padding = 10.0;
    let slots_per_row = 4;
    let start_x = panel_x + 20.0;
    let start_y = panel_y + 200.0;

    let hovered_slot = get_hovered_slot();

    for i in 0..INVENTORY_SIZE {
        let row = i / slots_per_row;
        let col = i % slots_per_row;
        let slot_x = start_x + col as f32 * (slot_size + slot_padding);
        let slot_y = start_y + row as f32 * (slot_size + slot_padding);

        // Draw slot background (highlight if hovered)
        let bg_color = if hovered_slot == Some(i) && player.inventory.items.get(i).is_some() {
            Color::from_rgba(80, 80, 100, 255)
        } else {
            Color::from_rgba(60, 60, 70, 255)
        };
        draw_rectangle(slot_x, slot_y, slot_size, slot_size, bg_color);
        draw_rectangle_lines(slot_x, slot_y, slot_size, slot_size, 1.0, GRAY);

        // Draw item if present
        if let Some(item) = player.inventory.items.get(i) {
            let color = match item {
                Item::Weapon(_) => ORANGE,
                Item::Armor(_) => SKYBLUE,
            };
            draw_poly(
                slot_x + slot_size / 2.0,
                slot_y + slot_size / 2.0,
                4,
                15.0,
                45.0,
                color,
            );
        }
    }

    // Draw tooltip for hovered item
    if let Some(slot_idx) = hovered_slot {
        if let Some(item) = player.inventory.items.get(slot_idx) {
            let (mouse_x, mouse_y) = mouse_position();
            draw_tooltip(mouse_x + 15.0, mouse_y + 15.0, item);
        }
    }

    // Item count
    draw_text(
        &format!("{}/{} slots used", player.inventory.count(), INVENTORY_SIZE),
        panel_x + 20.0,
        panel_y + panel_h - 40.0,
        16.0,
        GRAY,
    );

    // Instructions
    draw_text(
        "Click item to equip | Press I or ESC to close",
        panel_x + 20.0,
        panel_y + panel_h - 20.0,
        14.0,
        GRAY,
    );
}

fn draw_tooltip(x: f32, y: f32, item: &Item) {
    let name = item.name();
    let desc = item.description();

    let padding = 8.0;
    let name_size = 18.0;
    let desc_size = 14.0;

    let name_dims = measure_text(name, None, name_size as u16, 1.0);
    let desc_dims = measure_text(&desc, None, desc_size as u16, 1.0);

    let tooltip_w = name_dims.width.max(desc_dims.width) + padding * 2.0;
    let tooltip_h = name_size + desc_size + padding * 2.0;

    // Ensure tooltip stays on screen
    let screen_w = screen_width();
    let actual_x = if x + tooltip_w > screen_w {
        x - tooltip_w - 15.0
    } else {
        x
    };

    // Background
    draw_rectangle(
        actual_x,
        y,
        tooltip_w,
        tooltip_h,
        Color::from_rgba(20, 20, 30, 240),
    );
    draw_rectangle_lines(actual_x, y, tooltip_w, tooltip_h, 1.0, WHITE);

    // Name
    let name_color = match item {
        Item::Weapon(_) => ORANGE,
        Item::Armor(_) => SKYBLUE,
    };
    draw_text(name, actual_x + padding, y + padding + name_size - 4.0, name_size, name_color);

    // Description
    draw_text(
        &desc,
        actual_x + padding,
        y + padding + name_size + desc_size,
        desc_size,
        LIGHTGRAY,
    );
}
