use std::time::Duration;

use rusty_time::Timer;

use crate::block::{randomize_block, Block};
use crate::frame::{draw_sprite, draw_text, Drawable, Frame, O, PIXEL_SIZE, X, Y};

pub const BOARD_NUM_ROWS: usize = 20;
pub const BOARD_NUM_COLS: usize = 20;
const BORDER_SIZE: usize = 1;
const START_SPEED: u64 = 500;
const SPEED_DELTA: u64 = 50;

pub enum Direction {
    Down,
    Left,
    Right,
}

pub struct Board {
    grid: Vec<Vec<bool>>,
    current_block: Block,
    next_block: Block,
    move_timer: Timer,
    top_x: usize,
    top_y: usize,
    current_speed: u64,
    debug_mode: bool,
}

impl Board {
    pub fn new(debug_mode: bool) -> Self {
        Self {
            grid: vec![vec![false; BOARD_NUM_COLS]; BOARD_NUM_ROWS],
            current_block: randomize_block(),
            next_block: randomize_block(),
            move_timer: Timer::from_millis(START_SPEED),
            top_x: 0,
            top_y: 0,
            current_speed: START_SPEED,
            debug_mode,
        }
    }

    pub fn speed_up(&mut self) {
        self.move_timer = Timer::from_millis(0);
    }

    pub fn increase_speed(&mut self) {
        if self.current_speed > 200 {
            self.current_speed -= SPEED_DELTA;
            self.move_timer = Timer::from_millis(self.current_speed);
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer = Timer::from_millis(self.current_speed);
            if !self.move_block_down() {
                self.freeze_block();
                self.current_block = self.next_block.clone();
                self.next_block = randomize_block();
                return true;
            }
        }
        false
    }

    // This function moves the pixels of the falling block into the board structure, so that we can detect
    // future collisions and full horizontal lines
    pub fn freeze_block(&mut self) {
        let sprite = self.current_block.get_current_frame();
        let x = self.current_block.current_x();
        let y = self.current_block.current_y();
        for (sy, row) in sprite.iter().enumerate() {
            for (sx, pixel) in row.iter().enumerate() {
                if pixel.eq(&X) {
                    for i in 0..PIXEL_SIZE {
                        self.grid[y + sy][x + sx + i] = true;
                    }
                }
            }
        }
    }

    pub fn detect_collision(&mut self, direction: Direction) -> bool {
        let mut block = self.current_block.clone();
        let row_index = block.lower_y() + self.get_upper_offset();
        let x = block.current_x();
        let y = block.current_y();

        // println!("x:{} y:{} index:{}", x, y, row_index);
        match direction {
            Direction::Down => {
                if row_index == self.get_bottom_offset() {
                    // reached the bottom
                    return true;
                }

                for (sy, row) in block.get_current_frame().iter().enumerate() {
                    for (sx, pixel) in row.iter().enumerate() {
                        // if the row has a pixel and the grid is already taken we get a collision
                        if pixel.eq(&X) && self.grid[sy + y + 1][sx + x] == true {
                            return true;
                        }
                    }
                }
            }
            Direction::Left => {
                if x == self.get_left_offset() {
                    return true;
                }
                for (sy, row) in block.get_current_frame().iter().enumerate() {
                    for (sx, pixel) in row.iter().enumerate() {
                        // if the row has a pixel and the grid is already taken we get a collision
                        if pixel.eq(&X) && self.grid[sy + y][sx + x - 1] == true {
                            return true;
                        }
                    }
                }
            }
            Direction::Right => {
                if (x + block.get_width()) == self.get_right_offset() {
                    return true;
                }
                for (sy, row) in block.get_current_frame().iter().enumerate() {
                    for (sx, pixel) in row.iter().enumerate() {
                        // if the row has a pixel and the grid is already taken we get a collision
                        if pixel.eq(&X) && self.grid[sy + y][sx + x + 1] == true {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn board_is_full(self: &mut Board) -> bool {
        self.current_block.current_y() - self.top_y == 0 && self.detect_collision(Direction::Down)
    }

    pub fn get_left_offset(&self) -> usize {
        self.top_x
    }

    pub fn get_right_offset(&self) -> usize {
        self.get_left_offset() + BOARD_NUM_COLS
    }

    pub fn get_bottom_offset(&self) -> usize {
        self.get_upper_offset() + BOARD_NUM_ROWS
    }

    pub fn get_upper_offset(&self) -> usize {
        self.top_y
    }

    pub fn move_block_down(&mut self) -> bool {
        if !self.detect_collision(Direction::Down) {
            self.current_block.move_down();
            return true;
        }
        false
    }

    pub fn move_block_right(&mut self) -> bool {
        if !self.detect_collision(Direction::Right) {
            self.current_block.move_right();
            return true;
        }
        false
    }

    pub fn move_block_left(&mut self) -> bool {
        if !self.detect_collision(Direction::Left) {
            self.current_block.move_left();
            return true;
        }
        false
    }

    pub fn rotate(&mut self) -> bool {
        self.current_block.rotate();
        while self.current_block.current_x() + self.current_block.get_width()
            > self.top_x + BOARD_NUM_COLS
        {
            self.current_block.move_left();
        }
        true
    }

    pub fn check_completed_lines(&mut self) -> usize {
        let mut count: usize = 0;
        let grid = self.grid.clone();
        for (index, row) in grid.iter().enumerate() {
            if row.iter().filter(|v| **v == true).count() == row.len() {
                self.grid.remove(index);
                self.grid.insert(0, vec![false; BOARD_NUM_COLS]);
                count += 1;
            }
        }
        count
    }
}

impl Drawable for Board {
    fn draw(&self, frame: &mut Frame) {
        // Draw the borders
        for i in 0..BOARD_NUM_ROWS {
            frame[self.top_x][i + self.top_y] = '🀫';
            frame[self.top_x + BOARD_NUM_COLS + 1][i + self.top_y] = '🀫';
        }

        for i in 0..BOARD_NUM_COLS + 2 {
            frame[i + self.top_x][self.top_y + BOARD_NUM_ROWS] = '🀫';
        }

        if self.debug_mode {
            // draw memory map
            for (y, row) in self.grid.iter().enumerate() {
                for (x, v) in row.iter().enumerate() {
                    if *v == true {
                        frame[x + self.top_x + BOARD_NUM_ROWS + 10][y + self.top_y] = X;
                    } else {
                        frame[x + self.top_x + BOARD_NUM_ROWS + 10][y + self.top_y] = O;
                    }
                }
            }
        }

        // draw all blocks
        for (y, row) in self.grid.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                if *v == true {
                    frame[x + self.top_x + BORDER_SIZE][y + self.top_y] =
                        if x % 2 == 0 { X } else { Y };
                } else {
                    frame[x + self.top_x + BORDER_SIZE][y + self.top_y] = O;
                }
            }
        }

        // draw the current falling block
        draw_sprite(
            frame,
            self.current_block.get_current_frame(),
            self.current_block.current_x() + self.top_x + BORDER_SIZE,
            self.current_block.current_y() + self.top_y,
        );
        // draw the next falling block
        draw_text(
            frame,
            "Next block:",
            self.top_x + BORDER_SIZE + BOARD_NUM_COLS + 5,
            self.top_y,
        );
        draw_sprite(
            frame,
            self.next_block.get_current_frame(),
            self.top_x + BORDER_SIZE + BOARD_NUM_COLS + 5,
            self.top_y + 2,
        );
    }
}
