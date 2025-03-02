use std::{time::Instant, usize};

use minifb::Key;

use crate::{tetromino::{Tetromino, TetrominoGenertor}, GameState, State};

const BOARD_WIDTH: usize = 12;
const BOARD_HEIGHT: usize = 22;

pub struct Board {
    x: u32,
    y: u32,
    // BOARD_WIDTH and BOARD_HEIGHT include the border but we don't need state for those
    state: [[BlockState; BOARD_WIDTH - 2 as usize]; BOARD_HEIGHT - 2 as usize],
    tetromino_generator: TetrominoGenertor,
    current_tetromino: Option<Tetromino>,
    current_tetromino_x: u32,
    current_tetromino_y: u32,
    last_tick: Instant,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct BlockState {
    set: bool,
    color: u32,
}

impl Board {
    pub fn new(x: u32, y: u32) -> Self {
        let tetromino_generator = TetrominoGenertor::new();
        let state = [[BlockState {set: false, color: 0}; BOARD_WIDTH - 2 as usize]; BOARD_HEIGHT - 2 as usize];

        Self {
            x,
            y,
            state,
            tetromino_generator,
            current_tetromino: None,
            current_tetromino_x: 0,
            current_tetromino_y: 0,
            last_tick: Instant::now(),
        }
    }

    pub fn update(&mut self, gs: &mut GameState, down_keys: &Vec<Key>) {
        self.process_input(gs, down_keys);
        self.drop_tetromino(gs);
    }

    pub fn render(&self, gs: &mut GameState) {
        for px in 0..BOARD_WIDTH as u32 {
            gs.display.draw_block(0xFF999999, self.x + px, self.y);
            gs.display.draw_block(0xFF999999, self.x + px, self.y + (BOARD_HEIGHT as u32) - 1);
        }

        for py in 1..BOARD_HEIGHT as u32 {
            gs.display.draw_block(0xFF999999, self.x, self.y + py);
            gs.display.draw_block(0xFF999999, self.x + (BOARD_WIDTH as u32) - 1, self.y + py);
        }

        for h in 0..(BOARD_HEIGHT - 2) as u32 {
            for w in 0..(BOARD_WIDTH - 2) as u32 {
                let state = self.state[h as usize][w as usize];
                let x = w + self.x + 1;
                let y = h + self.y + 1;
                gs.display.draw_block(state.color, x, y);
            }
        }

        if let Some(tetromino) = &self.current_tetromino {
            tetromino.render(self.current_tetromino_x, self.current_tetromino_y, &mut gs.display);
            
            let mut y = self.current_tetromino_y;
            loop { 
                y += 1;
                if self.detect_collision(self.current_tetromino_x, y) {
                    break;
                }
            }

            if self.current_tetromino_y != y - 1 {
                tetromino.render_shadow(self.current_tetromino_x, y - 1, &mut gs.display);
            }
        }
    }

    pub fn place_new_tetromino(&mut self, gs: &mut GameState) {
        let tetromino = &gs.next_tetromino.clone().unwrap_or(self.tetromino_generator.get_random_tetromino());
        if let Some(representation) = tetromino.current_representation() {
            self.current_tetromino_x = self.x + (BOARD_WIDTH as u32 / 2 - ((representation.width as f32/2.0).ceil() as u32));
            self.current_tetromino_y = self.y + 1;
        }

        self.current_tetromino = Some(tetromino.clone());
        gs.next_tetromino = Some(self.tetromino_generator.get_random_tetromino());
    }

    fn process_input(&mut self, gs: &mut GameState, down_keys: &Vec<Key>) {
        down_keys.iter().for_each(|key|
            match key {
                Key::Left => {
                    if self.current_tetromino_x > self.x + 1 {
                        let next_x_position = self.current_tetromino_x - 1;
                        if !self.detect_collision(next_x_position, self.current_tetromino_y) {
                            self.current_tetromino_x -= 1;
                        }
                    }
                }
                Key::Right => {
                    let Some(tetromino) = &self.current_tetromino else { return };
                    let Some(representation) = tetromino.current_representation() else { return };

                    if self.current_tetromino_x + representation.width < self.x + BOARD_WIDTH as u32 - 1 {
                        let next_x_position = self.current_tetromino_x + 1;
                        if !self.detect_collision(next_x_position, self.current_tetromino_y) {
                            self.current_tetromino_x += 1;
                        }
                    }
                }
                Key::Up => {
                    if let Some(tetromino) = &mut self.current_tetromino {
                        tetromino.rotate();
                        if let Some(representation) = tetromino.current_representation() {
                            let width = representation.width;
                            if self.current_tetromino_x + width > self.x + BOARD_WIDTH as u32 - 2 {
                                self.current_tetromino_x = self.x + BOARD_WIDTH as u32 - 2 - width;
                            }
                        }
                    }
                }
                Key::Down => {
                    gs.state = State::DropBlock;
                }
                _ => {}
            });

    }

    fn drop_tetromino(&mut self, gs: &mut GameState) {
        if gs.state != State::DropBlock && self.last_tick.elapsed().as_millis() < 300 {
            return;
        }

        if self.current_tetromino.is_none() {
            return;
        }

        loop {
            if !self.detect_collision(self.current_tetromino_x, self.current_tetromino_y + 1) {
                self.current_tetromino_y += 1;
                self.last_tick = Instant::now();
                if gs.state != State::DropBlock {
                    return;
                }
            } else {
                break;
            }
        }

        if gs.state == State::DropBlock {
            gs.state = State::Playing;
            return;
        }

        self.settle_tetromino();
        let line_erased = self.remove_full_line();

        if line_erased > 0 {
            gs.state = State::UpdateScore(line_erased);
        }
        else {
            gs.state = State::NewTetrominoNeeded;
        }
    }

    fn detect_collision(&self, x: u32, y: u32) -> bool {
        let Some(tetromino) = &self.current_tetromino else { return false };
        let Some(representation) = tetromino.current_representation() else { return false };

        let vertices = &representation.vertices;

        if y + representation.height >= self.y + BOARD_HEIGHT as u32 {
            return true
        }

        for &(vertex_x, vertex_y) in vertices {
            let row = (y + vertex_y) - self.y - 1;
            let col = (x + vertex_x) - self.x - 1;
            if self.state[row as usize][col as usize].set {
                return true
            }
        }

        false
    }

    fn remove_full_line(&mut self) -> i32 { 
        let mut line_reset_count = 0;
        for row in (0..BOARD_HEIGHT-2).rev() {
            if !self.is_line_full(row) {
                continue;
            }

            self.reset_line(row);
            line_reset_count += 1;
        }

        if line_reset_count > 0 {
            self.drop_lines();
        }

        line_reset_count
    }

    fn settle_tetromino(&mut self) {
        let Some(tetromino) = &self.current_tetromino else { return };
        let Some(representation) = tetromino.current_representation() else { return };

        let vertices = &representation.vertices;
        for &(vertex_x, vertex_y) in vertices {
            let row = (self.current_tetromino_y + vertex_y) - self.y - 1;
            let col = (self.current_tetromino_x + vertex_x) - self.x - 1;
            self.state[row as usize][col as usize].set = true;
            self.state[row as usize][col as usize].color = representation.color;
        }
    }

    fn is_line_empty(&self, row: usize) -> bool {
        for col in 0..BOARD_WIDTH - 2 {
            if self.state[row][col].set {
                return false;
            }
        }

        true
    }

    fn is_line_full(&self, row: usize) -> bool {
        for col in 0..BOARD_WIDTH - 2 {
            if !self.state[row as usize][col as usize].set {
                return false;
            }
        }

        true
    }

    fn reset_line(&mut self, row: usize) {
        for col in 0..BOARD_WIDTH - 2 {
            self.state[row][col].set = false;
            self.state[row][col].color = 0;
        }
    }

    fn drop_lines(&mut self) {
        for row in (0..BOARD_HEIGHT - 3).rev() {
            let mut dst_row = row;
            let mut next_row = row + 1;

            while next_row != BOARD_HEIGHT - 2 && self.is_line_empty(next_row) {
                dst_row = next_row;
                next_row += 1;
            }

            if row != dst_row {
                self.copy_line(row, dst_row);
                self.reset_line(row);
            }
        }
    }

    fn copy_line(&mut self, src: usize, dst: usize) {
        for col in 0..BOARD_WIDTH - 2 {
            self.state[dst][col] = self.state[src][col];
        }
    }
}
