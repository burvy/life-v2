use std::ops::Deref;

use super::window;

struct NeighborBehavior {
    one: Behavior,
    two: Behavior,
    three: Behavior,
    four: Behavior,
    five: Behavior,
    six: Behavior,
    seven: Behavior,
    eight: Behavior,
}
enum Behavior {
    Birth,
    Survive,
    Die,
}

use Behavior::{Birth, Die, Survive};

const CONFIG: NeighborBehavior = NeighborBehavior {
    one: Die,
    two: Survive,
    three: Birth,
    four: Die,
    five: Die,
    six: Die,
    seven: Die,
    eight: Die,
};

/// this function is called to draw graphics in the window
pub fn draw_fn(graphics: &mut window::Graphics) {
    grid_looper(graphics);
    graphics.draw_grid();
}

fn grid_looper(graphics: &mut window::Graphics) {
    graphics
        .grid
        .clone()
        .iter()
        .enumerate()
        .for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, &pix)| {
                if pix {
                    graphics.draw_pixel_on_grid(window::PixelInfo {
                        x: x * graphics.scale,
                        y: y * graphics.scale,
                        color: [255, 255, 255, 255],
                    });
                    // test mutation
                    // mutate like this
                    *graphics
                        .grid
                        .get_mut(y)
                        .and_then(|row| row.get_mut(x))
                        .expect("grid cant be mutated") =
                        neighbor_condition(neighbor_count(graphics, x, y), CONFIG);
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

/// checks input neighbor count and tells you if this cell should be alive
fn neighbor_condition(count: u8, config: NeighborBehavior) -> bool {
    todo!()
}

/// checks nearby neighbors and returns neighbor count
fn neighbor_count(graphics: &window::Graphics, x: usize, y: usize) -> u8 {
    (0..3)
        .flat_map(|i| (0..3).map(move |j| (i, j))) // allows ownership of both i and j now
        .map(|(i, j)| {
            if graphics
                .grid
                .get(y - 1 + j)
                .and_then(|row| row.get(x - 1 + i))
                .copied()
                .unwrap_or(false)
            {
                1_u8
            } else {
                0_u8
            }
        })
        .sum()
}
