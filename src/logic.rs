use super::window;

pub fn draw_fn(graphics: &mut window::Graphics) {
    graphics.draw_pixel_on_grid(window::PixelInfo {
        x: 1,
        y: 1,
        color: [255, 255, 255, 255],
    });
    graphics.draw_grid();
}
