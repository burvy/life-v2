use super::window;

pub fn draw_fn(graphics: &mut window::Graphics) {
    (10..30).for_each(|x| {
        (10..30).for_each(|y| {
            graphics.draw_pixel(x, y, [255, 0, 100, 255]);
        })
    });
    graphics.draw_pixel_on_grid(window::PixelInfo {
        x: 1,
        y: 1,
        scale: 10,
        color: [255, 255, 255, 255],
    })
}
