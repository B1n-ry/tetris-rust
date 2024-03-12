use draw::to_coord;
use game::TetrisGame;
use piston_window::{clear, types::Color, Button, Key, PistonWindow, PressEvent, UpdateEvent, WindowSettings};

extern crate rand;
extern crate piston_window;

mod game;
mod draw;

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
    let (width, height) = (10, 20);

    let mut window: PistonWindow = WindowSettings::new("Tetris", [to_coord(width) as u32, to_coord(height) as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = TetrisGame::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Left => game.move_falling_horizontal(-1),
                Key::Right => game.move_falling_horizontal(1),
                Key::Down => game.move_falling_down(),
                Key::Q => game.rotate_falling(false),
                Key::E => game.rotate_falling(true),
                Key::Space => game.smash_down_falling(),
                _ => (),
            };
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.render(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
