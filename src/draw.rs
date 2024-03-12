use piston_window::{rectangle, types::Color, Context, G2d};

use crate::game::TetrisBlock;


const BLOCK_SIZE: usize = 32;

pub fn draw_block(block: &TetrisBlock, x: usize, y: usize, con: &Context, g: &mut G2d) {
    let gui_x = x * BLOCK_SIZE;
    let gui_y = y * BLOCK_SIZE;
    rectangle(
        block.color(),
        [
            gui_x as f64, gui_y as f64,
            BLOCK_SIZE as f64, BLOCK_SIZE as f64,
        ],
        con.transform,
        g,
    )
}

pub fn draw_rect(color: Color, x: usize, y: usize, width: usize, height: usize, con: &Context, g: &mut G2d) {
    let gui_x = x * BLOCK_SIZE;
    let gui_y = y * BLOCK_SIZE;
    rectangle(
        color,
        [
            gui_x as f64, gui_y as f64,
            (BLOCK_SIZE * width) as f64, (BLOCK_SIZE * height) as f64,
        ],
        con.transform,
        g,
    )
}

pub fn to_coord(val: usize) -> usize {
    val * BLOCK_SIZE
}