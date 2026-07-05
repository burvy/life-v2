use super::window;

struct NeighborBehavior([bool; 9]);

const CONFIG: NeighborBehavior =
    NeighborBehavior([false, false, true, true, false, false, false, false, false]);

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
                let color = if pix {
                    [255, 255, 255, 255]
                } else {
                    [0, 0, 0, 255]
                };
                graphics.draw_pixel_on_grid(window::PixelInfo {
                    x: x * graphics.scale,
                    y: y * graphics.scale,
                    color,
                });
                // mutate like this
                *graphics
                    .grid
                    .get_mut(y)
                    .and_then(|row| row.get_mut(x))
                    .expect("grid cant be mutated") =
                    neighbor_condition(neighbor_count(graphics, x, y), &CONFIG).unwrap_or(false);
            })
        });
}

/// checks input neighbor count and tells you if this cell should be alive
fn neighbor_condition(count: usize, config: &NeighborBehavior) -> Option<bool> {
    config.0.get(count).copied()
}

/// checks nearby neighbors and returns neighbor count
fn neighbor_count(graphics: &window::Graphics, x: usize, y: usize) -> usize {
    (0..3)
        .flat_map(|i| (0..3).map(move |j| (i, j)))
        .filter(|&(i, j)| (i, j) != (1, 1)) // skip middle
        .map(|(i, j)| {
            if (y + j)
                .checked_sub(1) // y - 1 + j
                .zip((x + i).checked_sub(1)) // x - 1 + i
                .and_then(|(cx, cy)| graphics.grid.get(cx).and_then(|row| row.get(cy))) // check if that is true
                .copied() // no borrow checker issues
                .unwrap_or(false)
            {
                1
            } else {
                0
            }
        })
        .sum() // how many neighbors are alive!
}
