use macroquad::{color, prelude::*};

fn draw_text_in_rect(text: &str, x: f32, y: f32, w: f32, h: f32, mut size: f32, color: Color) {
    let padding = 10.0;

    // уменьшаем шрифт, пока текст не влезет в кнопку
    while size > 10.0 && measure_text(text, None, size as u16, 1.0).width > w - padding * 2.0 {
        size -= 1.0;
    }

    let dims = measure_text(text, None, size as u16, 1.0);
    let tx = x + (w - dims.width) / 2.0;
    let ty = y + (h - dims.height) / 2.0 + dims.offset_y;
    draw_text(text, tx, ty, size, color);
}

pub fn button(text: &str, x: f32, y: f32, width: f32, height: f32) -> bool {
    let (mx, my) = mouse_position();
    let hovered = mx > x && mx < x + width && my > y && my < y + height;
    
    let color = if hovered { color::LIGHTGRAY } else { color::GRAY };
    draw_rectangle(x, y, width, height, color);
     draw_text_in_rect(text, x, y, width, height, 30.0, WHITE);

    hovered && is_mouse_button_pressed(MouseButton::Left)
}

pub fn tab_button(text: &str, x: f32, y: f32, w: f32, h: f32, active: bool) -> bool {
    let (mx, my) = mouse_position();
    let hovered = mx > x && mx < x + w && my > y && my < y + h;

    let color = if active { DARKBLUE } else if hovered { LIGHTGRAY } else { GRAY };
    draw_rectangle(x, y, w, h, color);
    draw_text_in_rect(text, x, y, w, h, 30.0, WHITE);

    hovered && is_mouse_button_pressed(MouseButton::Left)
}