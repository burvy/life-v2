use super::window;

/// this function is called to draw graphics in the window
pub fn draw_fn(graphics: &mut window::Graphics) {
    grid_filler(graphics);
    graphics.draw_grid();
}

fn grid_filler(graphics: &mut window::Graphics) {
    let grid = graphics.grid.clone();
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, &pix)| {
            if pix {
                graphics.draw_pixel_on_grid(window::PixelInfo {
                    x: x * graphics.scale,
                    y: y * graphics.scale,
                    color: [255, 255, 255, 255],
                });
            } else {
                graphics.draw_pixel_on_grid(window::PixelInfo {
                    x: x * graphics.scale,
                    y: y * graphics.scale,
                    color: [0, 0, 0, 255],
                });
            }
        })
    });
}
