use macroquad::prelude::*;

pub fn draw_health_bar(current: i32, max: i32) {
    let bar_x = 20.0;
    let bar_y = 20.0;
    let bar_width = 200.0;
    let bar_height = 25.0;

    // Background
    draw_rectangle(bar_x, bar_y, bar_width, bar_height, DARKGRAY);

    // Health fill
    let health_pct = current as f32 / max as f32;
    let health_color = if health_pct > 0.5 {
        GREEN
    } else if health_pct > 0.25 {
        YELLOW
    } else {
        RED
    };
    draw_rectangle(bar_x, bar_y, bar_width * health_pct, bar_height, health_color);

    // Border
    draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 2.0, WHITE);

    // Text
    let text = format!("{}/{}", current, max);
    let text_dims = measure_text(&text, None, 20, 1.0);
    draw_text(
        &text,
        bar_x + bar_width / 2.0 - text_dims.width / 2.0,
        bar_y + bar_height / 2.0 + text_dims.height / 2.0 - 2.0,
        20.0,
        WHITE,
    );
}
