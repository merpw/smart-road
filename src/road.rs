use macroquad::prelude::*;
pub fn render_roads() {
    // Vertical lines
    draw_line(450.0, 0.0, 450.0, 1000.0, 1.0, WHITE);
    draw_line(500.0, 0.0, 500.0, 1000.0, 1.0, WHITE);
    draw_line(550.0, 0.0, 550.0, 1000.0, 1.0, WHITE);

    // Horizontal lines
    draw_line(0.0, 450.0, 1000.0, 450.0, 1.0, WHITE);
    draw_line(0.0, 500.0, 1000.0, 500.0, 1.0, WHITE);
    draw_line(0.0, 550.0, 1000.0, 550.0, 1.0, WHITE);
}
