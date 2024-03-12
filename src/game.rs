use piston_window::{types::Color, Context, G2d};

use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rect};

const FALL_SPEED: f64 = 0.5;
const RESTART_TIME: f64 = 4.0;

#[derive(Clone, Copy, PartialEq)]
pub enum TetrisBlock {
    L,
    T,
    S,
    I,
    O,
    Z,
    J,
    EMPTY,
}
impl TetrisBlock {
    pub fn color(&self) -> Color {
        match *self {
            TetrisBlock::L => [0.2, 1.0, 1.0, 1.0],
            TetrisBlock::T => [0.8, 0.0, 0.5, 1.0],
            TetrisBlock::I => [0.3, 0.3, 0.8, 1.0],
            TetrisBlock::J => [0.0, 0.0, 0.9, 1.0],
            TetrisBlock::O => [0.0, 0.9, 0.9, 1.0],
            TetrisBlock::S => [0.0, 0.8, 0.0, 1.0],
            TetrisBlock::Z => [0.8, 0.0, 0.0, 1.0],
            TetrisBlock::EMPTY => [0.0, 0.0, 0.0, 0.0],
        }
    }
    fn random() -> TetrisBlock {
        let mut rng = thread_rng();
        TetrisBlock::get(rng.gen_range(0..=6))
    }
    fn get(i: usize) -> TetrisBlock {
        match i {
            0 => TetrisBlock::L,
            1 => TetrisBlock::T,
            2 => TetrisBlock::I,
            3 => TetrisBlock::J,
            4 => TetrisBlock::O,
            5 => TetrisBlock::S,
            6 => TetrisBlock::Z,
            _ => TetrisBlock::EMPTY,
        }
    }
}

struct TetrisPiece {
    piece: Vec<Vec<TetrisBlock>>,
    x: usize,
    y: usize,
}
impl TetrisPiece {
    fn new(x: usize, y: usize, block: TetrisBlock) -> TetrisPiece {
        let piece = match block {
            TetrisBlock::I => vec![
                vec![TetrisBlock::I],
                vec![TetrisBlock::I],
                vec![TetrisBlock::I],
                vec![TetrisBlock::I],
            ],
            TetrisBlock::L => vec![
                vec![TetrisBlock::L, TetrisBlock::EMPTY],
                vec![TetrisBlock::L, TetrisBlock::EMPTY],
                vec![TetrisBlock::L, TetrisBlock::L],
            ],
            TetrisBlock::J => vec![
                vec![TetrisBlock::EMPTY, TetrisBlock::J],
                vec![TetrisBlock::EMPTY, TetrisBlock::J],
                vec![TetrisBlock::J, TetrisBlock::J],
            ],
            TetrisBlock::S => vec![
                vec![TetrisBlock::EMPTY, TetrisBlock::S, TetrisBlock::S],
                vec![TetrisBlock::S, TetrisBlock::S, TetrisBlock::EMPTY],
            ],
            TetrisBlock::Z => vec![
                vec![TetrisBlock::Z, TetrisBlock::Z, TetrisBlock::EMPTY],
                vec![TetrisBlock::EMPTY, TetrisBlock::Z, TetrisBlock::Z],
            ],
            TetrisBlock::T => vec![
                vec![TetrisBlock::EMPTY, TetrisBlock::T, TetrisBlock::EMPTY],
                vec![TetrisBlock::T, TetrisBlock::T, TetrisBlock::T],
            ],
            TetrisBlock::O => vec![
                vec![TetrisBlock::O, TetrisBlock::O],
                vec![TetrisBlock::O, TetrisBlock::O],
            ],
            _ => vec![vec![TetrisBlock::EMPTY]],
        };

        TetrisPiece {
            piece,
            x,
            y,
        }
    }

    fn get(&self, x: usize, y: usize) -> &TetrisBlock {
        let outside_board_msg: &str = "Outside board bound";
        self.piece.get(y).expect(outside_board_msg).get(x).expect(outside_board_msg)
    }

    fn size(&self) -> (usize, usize) {
        let first_row: Option<&Vec<TetrisBlock>> = self.piece.get(0);
        match first_row {
            Some(row) => (row.len(), self.piece.len()),
            _ => (0, 0),
        }
    }

    fn burn_into(&self, board: &mut TetrisBoard) {
        let (width, height) = self.size();
        for x in 0..width {
            for y in 0..height {
                let block = self.get(x, y);
                if *block != TetrisBlock::EMPTY {
                    board.set(self.x + x, self.y + y, *block)
                }
            }
        }
    }

    fn rotate(&mut self, clockwise: bool) {
        if clockwise {
            let (width, height) = self.size();
            let mut new_piece: Vec<Vec<TetrisBlock>> = vec![vec![TetrisBlock::EMPTY; height]; width];
            for i in 0..height {
                for j in 0..width {
                    new_piece[j][height - i - 1] = self.piece[i][j];
                }
            }
            self.piece = new_piece;
        } else {
            // Easier than figuring out how to do clockwise rotation
            self.rotate(true);
            self.rotate(true);
            self.rotate(true);
        }
    }
}

struct TetrisBoard {
    board: Vec<Vec<TetrisBlock>>
}
impl TetrisBoard {
    fn new(width: usize, height: usize) -> TetrisBoard {
        let board: Vec<Vec<TetrisBlock>> = vec![vec![TetrisBlock::EMPTY; width]; height];
        TetrisBoard {
            board
        }
    }
    fn get(&self, x: usize, y: usize) -> &TetrisBlock {
        let outside_board_msg: &str = "Outside board bound";
        self.board.get(y).expect(outside_board_msg).get(x).expect(outside_board_msg)
    }
    fn set(&mut self, x: usize, y: usize, block: TetrisBlock) {
        self.board[y][x] = block;
    }

    fn size(&self) -> (usize, usize) {
        let first_row: Option<&Vec<TetrisBlock>> = self.board.get(0); 
        match first_row {
            Some(row) => (row.len(), self.board.len()),
            _ => (0, 0),
        }
    }
}

pub struct TetrisGame {
    board: TetrisBoard,
    falling_piece: TetrisPiece,

    waiting_time: f64,
    game_over: bool,
    score: f64,
}
impl TetrisGame {
    pub fn new(width: usize, height: usize) -> TetrisGame {
        TetrisGame {
            board: TetrisBoard::new(width, height),
            falling_piece: TetrisPiece::new(width / 2 - 1, 0, TetrisBlock::random()),
            waiting_time: 0.0,
            game_over: false,
            score: 0.0,
        }
    }

    pub fn render(&self, con: &Context, g: &mut G2d) {
        let (board_width, board_height): (usize, usize) = self.board.size();
        let (falling_width, falling_height) = self.falling_piece.size();
        for x in 0..board_width {
            for y in 0..board_height {
                if (self.falling_piece.x..(self.falling_piece.x + falling_width)).contains(&x)
                && (self.falling_piece.y..(self.falling_piece.y + falling_height)).contains(&y)
                && *self.falling_piece.get(x - self.falling_piece.x, y - self.falling_piece.y) != TetrisBlock::EMPTY {
                    draw_block(self.falling_piece.get(x - self.falling_piece.x, y - self.falling_piece.y), x, y, con, g);
                } else {
                    draw_block(self.board.get(x, y), x, y, con, g);
                }
            }
        }

        if self.game_over {
            draw_rect([0.8, 0.0, 0.0, 0.5], 0, 0, board_width, board_height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if self.waiting_time > FALL_SPEED {
            self.move_falling_down();
        }
    }

    pub fn move_falling_down(&mut self) {
        if self.game_over {
            return;
        }

        self.falling_piece.y += 1;
        self.waiting_time = 0.0;
        if !self.falling_valid() {
            self.falling_piece.y -= 1;
            self.falling_piece.burn_into(&mut self.board);

            self.gather_score();

            self.new_falling_piece();
            if !self.falling_valid() {
                self.game_over = true;
                println!("Your score: {}", self.score);
            }
        }
    }
    pub fn move_falling_horizontal(&mut self, distance: isize) {
        if self.game_over {
            return;
        }

        let (board_width, _): (usize, usize) = self.board.size();
        let (falling_width, _): (usize, usize) = self.falling_piece.size();
        let old_x = self.falling_piece.x;
        let moved_x = self.falling_piece.x.wrapping_add_signed(distance);
        if (0..=(board_width - falling_width)).contains(&moved_x) {
            self.falling_piece.x = moved_x;

            if !self.falling_valid() {
                self.falling_piece.x = old_x;
            }
        }
    }
    fn falling_valid(&self) -> bool {
        let (board_width, board_height) = self.board.size();
        let (falling_width, falling_height) = self.falling_piece.size();

        for x in 0..falling_width {
            for y in 0..falling_height {
                if *self.falling_piece.get(x, y) != TetrisBlock::EMPTY {
                    if self.falling_piece.x + x >= board_width || self.falling_piece.y + y >= board_height {
                        return false;
                    } else if *self.board.get(self.falling_piece.x + x, self.falling_piece.y + y) != TetrisBlock::EMPTY {
                        return false;
                    }
                }
            }
        }
        return true;
    }
    fn new_falling_piece(&mut self) {
        let (width, _) = self.board.size();
        self.falling_piece = TetrisPiece::new(width / 2 - 1, 0, TetrisBlock::random())
    }
    pub fn rotate_falling(&mut self, clockwise: bool) {
        self.falling_piece.rotate(clockwise);

        if !self.falling_valid() {
            self.falling_piece.rotate(!clockwise);
        }
    }
    fn gather_score(&mut self) {
        let (board_width, board_height) = self.board.size();
        let mut full_rows: usize = 0;
        for row in (0..board_height).rev() {
            for col in 0..board_width {
                self.board.board[row + full_rows][col] = self.board.board[row][col];
            }
            if !self.board.board[row].contains(&TetrisBlock::EMPTY) {
                full_rows += 1;
            }
        }

        // Makes sure we clear all
        for row in 0..full_rows {
            for col in 0..board_width {
                self.board.board[row][col] = TetrisBlock::EMPTY;
            }
        }
        if full_rows > 0 {
            self.score += full_rows as f64 * (full_rows as f64 / 2.0) * 100.0;
            println!("Score: {}", self.score);
        }
    }
    pub fn smash_down_falling(&mut self) {
        if self.game_over {
            return;
        }
        
        self.falling_piece.y += 1;
        while self.falling_valid() {
            self.falling_piece.y -= 1;
            self.move_falling_down();
            self.falling_piece.y += 1;
        }
        self.falling_piece.y -= 1;
        self.waiting_time = 0.0;
    }
    fn restart(&mut self) {
        let (width, height) = self.board.size();
        self.game_over = false;
        self.board = TetrisBoard::new(width, height);
        self.falling_piece = TetrisPiece::new(width / 2 - 1, 0, TetrisBlock::random());
        if !self.falling_valid() {
            self.falling_piece.x = 0;
        }
        self.waiting_time = 0.0;
    }
}
