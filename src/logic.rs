use super::window;

/// this function is called to draw graphics in the window
pub fn draw_fn(graphics: &mut window::Graphics) {
    let (size_x, size_y) = (
        graphics.pixels.texture().size().width as usize,
        graphics.pixels.texture().size().height as usize,
    );
    let scale = graphics.scale;
    let grid = &mut graphics.grid;
    graphics.draw_pixel_on_grid(window::PixelInfo {
        x: 1,
        y: 1,
        color: [255, 255, 255, 255],
    });
    graphics.draw_grid();
}
