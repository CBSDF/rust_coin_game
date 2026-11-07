use macroquad::{color, prelude::*};

pub fn button(text: &str, x: f32, y: f32, width: f32, height: f32) -> bool {
    let (mx, my) = mouse_position();
    let hovered = mx > x && mx < x + width && my > y && my < y + height;
    
    let color = if hovered { color::LIGHTGRAY } else { color::GRAY };
    draw_rectangle(x, y, width, height, color);
    draw_text(text, x + 20.0, y + height / 2.0 + 8.0, 30.0, WHITE);

    hovered && is_mouse_button_pressed(MouseButton::Left)
}