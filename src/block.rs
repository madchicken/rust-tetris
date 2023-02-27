use rand::rngs::OsRng;
use rand::RngCore;

use crate::frame::{O, PIXEL_SIZE, X, Y};

#[derive(Clone)]
pub struct Block {
    frames: Vec<Vec<Vec<char>>>, // 4 frames, one for each rotation angle
    x: usize, // x position of the block on the screen (referred to upper left corner)
    y: usize, // y position of the block on the screen (referred to upper left corner)
    current_frame: i32,
}

impl Block {
    pub fn new(frames: Vec<Vec<Vec<char>>>) -> Self {
        Self {
            frames,
            x: 0,
            y: 0,
            current_frame: 0,
        }
    }

    pub fn rotate(&mut self) {
        self.current_frame += 1;
        if self.current_frame > 3 {
            self.current_frame = 0;
        }
    }

    pub fn lower_y(&mut self) -> usize {
        self.y + self.get_height()
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }

    pub fn move_right(&mut self) {
        self.x += PIXEL_SIZE;
    }

    pub fn move_left(&mut self) {
        self.x -= PIXEL_SIZE;
    }

    #[allow(dead_code)]
    pub fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn current_x(&self) -> usize {
        self.x
    }

    pub fn current_y(&self) -> usize {
        self.y
    }

    pub fn get_height(&mut self) -> usize {
        self.frames[self.current_frame as usize].len()
    }

    pub fn get_width(&self) -> usize {
        self.frames[self.current_frame as usize]
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or(0)
    }

    #[allow(dead_code)]
    pub fn get_all_frames(&self) -> &Vec<Vec<Vec<char>>> {
        &self.frames
    }

    pub fn get_current_frame(&self) -> &Vec<Vec<char>> {
        &self.frames[self.current_frame as usize]
    }
}

pub enum BlockType {
    Line,
    Square,
    LeftS,
    RightS,
    T,
    RightL,
    LeftL,
}

pub fn build_block(block_type: BlockType) -> Block {
    match block_type {
        BlockType::Line => Block::new(vec![
            vec![vec![X, Y, X, Y, X, Y, X, Y]],
            vec![vec![X, Y], vec![X, Y], vec![X, Y], vec![X, Y]],
            vec![vec![X, Y, X, Y, X, Y, X, Y]],
            vec![vec![X, Y], vec![X, Y], vec![X, Y], vec![X, Y]],
        ]),
        BlockType::Square => Block::new(vec![
            vec![vec![X, Y, X, Y], vec![X, Y, X, Y]],
            vec![vec![X, Y, X, Y], vec![X, Y, X, Y]],
            vec![vec![X, Y, X, Y], vec![X, Y, X, Y]],
            vec![vec![X, Y, X, Y], vec![X, Y, X, Y]],
        ]),
        BlockType::LeftS => Block::new(vec![
            vec![vec![X, Y, X, Y, O, O], vec![O, O, X, Y, X, Y]],
            vec![vec![O, O, X, Y], vec![X, Y, X, Y], vec![X, Y, O, O]],
            vec![vec![X, Y, X, Y, O, O], vec![O, O, X, Y, X, Y]],
            vec![vec![O, O, X, Y], vec![X, Y, X, Y], vec![X, Y, O, O]],
        ]),
        BlockType::RightS => Block::new(vec![
            vec![vec![O, O, X, Y, X, Y], vec![X, Y, X, Y, O, O]],
            vec![vec![X, Y, O, O], vec![X, Y, X, Y], vec![O, O, X, Y]],
            vec![vec![O, O, X, Y, X, Y], vec![X, Y, X, Y, O, O]],
            vec![vec![X, Y, O, O], vec![X, Y, X, Y], vec![O, O, X, Y]],
        ]),
        BlockType::LeftL => Block::new(vec![
            vec![vec![X, Y, O, O, O, O], vec![X, Y, X, Y, X, Y]],
            vec![vec![X, Y, X, Y], vec![X, Y, O, O], vec![X, Y, O, O]],
            vec![vec![X, Y, X, Y, X, Y], vec![O, O, O, O, X, Y]],
            vec![vec![O, O, X, Y], vec![O, O, X, Y], vec![X, Y, X, Y]],
        ]),
        BlockType::RightL => Block::new(vec![
            vec![vec![O, O, O, O, X, Y], vec![X, Y, X, Y, X, Y]],
            vec![vec![X, Y, O, O], vec![X, Y, O, O], vec![X, Y, X, Y]],
            vec![vec![X, Y, X, Y, X, Y], vec![X, Y, O, O, O, O]],
            vec![vec![X, Y, X, Y], vec![O, O, X, Y], vec![O, O, X, Y]],
        ]),
        BlockType::T => Block::new(vec![
            vec![vec![O, O, X, Y, O, O], vec![X, Y, X, Y, X, Y]],
            vec![vec![X, Y, O, O], vec![X, Y, X, Y], vec![X, Y, O, O]],
            vec![vec![X, Y, X, Y, X, Y], vec![O, O, X, Y, O, O]],
            vec![vec![O, O, X, Y], vec![X, Y, X, Y], vec![O, O, X, Y]],
        ]),
    }
}

pub fn randomize_block() -> Block {
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    let random = OsRng.next_u32() % 7; // the size of BlockType enum

    match random {
        0 => build_block(BlockType::LeftS),
        1 => build_block(BlockType::RightS),
        2 => build_block(BlockType::Square),
        3 => build_block(BlockType::LeftL),
        4 => build_block(BlockType::RightL),
        5 => build_block(BlockType::Line),
        6 => build_block(BlockType::T),
        _ => unreachable!("Trying to generate an unknown block"),
    }
}
