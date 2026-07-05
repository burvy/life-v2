use super::window;

struct NeighborBehavior([Behavior; 9]);

#[derive(Copy, Clone)]
enum Behavior {
    Birth,
    Earth,
    Death,
}

use Behavior::{Birth, Death, Earth};

const CONFIG: NeighborBehavior = NeighborBehavior([
    Death, Death, Earth, Birth, Death, Death, Death, Death, Death,
]);

/// this function is called to draw graphics in the window
pub fn draw_fn(graphics: &mut window::Graphics) {
    grid_looper(graphics);
    graphics.draw_grid();
}

fn grid_looper(graphics: &mut window::Graphics) {
    let prev = graphics.grid.clone(); // we change graphics through this be careful
    prev.iter().enumerate().for_each(|(y, row)| {
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
                match neighbor_condition(neighbor_count(&mut graphics.grid, x, y), &CONFIG)
                    .unwrap_or(Death)
                {
                    Birth => true,
                    Earth => {
                        if *graphics
                            .grid
                            .get(y)
                            .and_then(|row| row.get(x))
                            .expect("grid couldnt be read here")
                        {
                            true
                        } else {
                            false
                        }
                    }
                    Death => false,
                }
        })
    });
}

/// checks input neighbor count and tells you if this cell should be alive
fn neighbor_condition(count: usize, config: &NeighborBehavior) -> Option<Behavior> {
    config.0.get(count).copied()
}

/// checks nearby neighbors and returns neighbor count
fn neighbor_count(grid: &[Vec<bool>], x: usize, y: usize) -> usize {
    (0..3)
        .flat_map(|i| (0..3).map(move |j| (i, j)))
        .filter(|&(i, j)| (i, j) != (1, 1)) // skip middle
        .map(|(i, j)| {
            if (y + j)
                .checked_sub(1) // y - 1 + j
                .zip((x + i).checked_sub(1)) // x - 1 + i
                .and_then(|(cy, cx)| grid.get(cy).and_then(|row| row.get(cx))) // check if that is true
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
