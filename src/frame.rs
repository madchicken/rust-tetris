use crate::{SCREEN_NUM_COLS, SCREEN_NUM_ROWS};

pub const X: char = '⎕';
pub const O: char = ' ';
pub type Frame = [[char; SCREEN_NUM_ROWS]; SCREEN_NUM_COLS];

pub fn new_frame() -> Frame {
    [[O; SCREEN_NUM_ROWS]; SCREEN_NUM_COLS]
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

pub fn draw_sprite(frame: &mut Frame, sprite: &Vec<Vec<char>>, x: usize, y: usize) {
    for (sy, row) in sprite.iter().enumerate() {
        for (sx, c) in row.iter().enumerate() {
            if frame[sx + x][sy + y].ne(&X) {
                frame[sx + x][sy + y] = *c;
            }
        }
    }
}

pub fn draw_text(frame: &mut Frame, text: &str, x: usize, y: usize) {
    for (i, c) in text.chars().enumerate() {
        frame[x + i][y] = c;
    }
}
