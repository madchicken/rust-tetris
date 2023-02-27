use crate::frame::{draw_text, Drawable, Frame};

pub struct ScoreBoard {
    top_x: usize,
    top_y: usize,
    score: u64,
    level: u32,
    total_lines: u32,
}

impl ScoreBoard {
    pub fn new(top_x: usize, top_y: usize) -> Self {
        Self {
            score: 0,
            level: 1,
            top_x,
            top_y,
            total_lines: 0,
        }
    }

    pub fn update(&mut self, lines: usize) -> bool {
        self.total_lines += lines as u32;
        self.score += (lines * 100) as u64; // 100 points for each line
        if lines > 1 {
            // add bonus!
            match lines {
                2 => self.score += 50,
                3 => self.score += 150,
                4 => self.score += 300,
                _ => self.score += 100,
            }
        }
        if lines > 0 {
            if self.total_lines % 10 == 0 {
                self.inc_level();
                return true;
            }
        }
        false
    }

    fn inc_level(&mut self) {
        self.level += 1;
    }
}

impl Drawable for ScoreBoard {
    fn draw(&self, frame: &mut Frame) {
        draw_text(
            frame,
            format!("Score: {}", self.score).as_str(),
            self.top_x,
            self.top_y,
        );
        draw_text(
            frame,
            format!("Level: {}", self.level).as_str(),
            self.top_x,
            self.top_y + 2,
        );
    }
}
