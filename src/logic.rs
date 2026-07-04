use super::window;

/// this function is called to draw graphics in the window
pub fn draw_fn(graphics: &mut window::Graphics) {
    graphics.draw_pixel_on_grid(window::PixelInfo {
        x: 1,
        y: 1,
        color: [255, 255, 255, 255],
    });
    graphics.draw_grid();
}

fn grid_filler(grid: Vec<bool>, graphics: &mut window::Graphics) {
    grid.iter()
        .enumerate() // our structure is similar to the pixel buffer and is usable
        .filter_map(|(i, &c)| if c { Some(i) } else { None })
        .for_each(|i| )
    todo!()
}
