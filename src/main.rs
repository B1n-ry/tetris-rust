extern crate piston_window;

use piston_window::{grid::Grid, *};

const BLOCK_SIZE: f64 = 16.0;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      context.transform,
                      graphics);
        });
    }
}

#[derive(Clone)]
enum TetrisBlock {
    L,
    T,
    S,
    Z,
    LINE,
    O,
    EMPTY,
}

struct Level {
    grid: Grid,
}

impl Level {
    pub fn new(width: u32, height: u32) -> Self {
        Level {
            grid: Grid {
                rows: height,
                cols: width,
                units: BLOCK_SIZE,
            },
        }
    }
}
