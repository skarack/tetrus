use std::collections::VecDeque;

use rand::{thread_rng, Rng};

use crate::display::Display;

#[derive(Debug, Clone)]
pub struct Representation {
    pub vertices: Vec<(u32, u32)>,
    pub width: u32,
    pub height: u32,
    pub color: u32
}

#[derive(Clone, Debug)]
pub struct Tetromino {
    representations: VecDeque<Representation>,
}

impl Tetromino {
    pub fn new(reprensentations: Vec<Representation>) -> Self {
        Self { representations: VecDeque::from(reprensentations)}
    }

    pub fn current_representation(&self) -> Option<&Representation> {
        self.representations.front()
    }

    pub fn render(&self, x: u32, y: u32, display: &mut Display) {
        assert!(self.representations.len() > 0, "Tetromino should have at least one representation to render");

        if let Some(current_representation) = self.representations.front() {
            for &(vertex_x, vertex_y) in &current_representation.vertices {
                display.draw_block(current_representation.color, x + vertex_x, y + vertex_y);
            }
        }
    }

    pub fn render_shadow(&self, x: u32, y: u32, display: &mut Display) {
        assert!(self.representations.len() > 0, "Tetromino should have at least one representation to render");

        if let Some(current_representation) = self.representations.front() {
            for &(vertex_x, vertex_y) in &current_representation.vertices {
                display.draw_block(0xFF161616, x + vertex_x, y + vertex_y);
            }
        }
    }

    pub fn rotate(&mut self) {
        assert!(self.representations.len() > 0, "Block should have at least one representation to rotate");

        if let Some(rep) = self.representations.pop_front() {
            self.representations.push_back(rep);
        }
    }
}

pub struct TetrominoGenertor {
    cache: Vec<Tetromino>,
}

impl TetrominoGenertor {
    pub fn new() -> Self {
        let mut tetrominos = Vec::with_capacity(7);

        // I Block
        let representations = vec![
            Representation { vertices: vec![(0, 0), (0, 1), (0, 2), (0, 3)], width: 1, height: 4, color: 0xFF00F0F0 },
            Representation { vertices: vec![(0, 0), (1, 0), (2, 0), (3, 0)], width: 4, height: 1, color: 0xFF00F0F0 },
        ];
        tetrominos.push(Tetromino::new(representations));
        // O Tetromino
        let representations = vec![
            Representation { vertices: vec![(0, 0), (1, 0), (0, 1), (1, 1)], width: 2, height: 2, color: 0xF0F000 },
        ];
        tetrominos.push(Tetromino::new(representations));
        // J Tetromino
        let representations = vec![
            Representation { vertices: vec![(1, 0), (1, 1), (0, 2), (1, 2)], width: 2, height: 3, color: 0xFF0000F0 },
            Representation { vertices: vec![(0, 0), (0, 1), (1, 1), (2, 1)], width: 3, height: 2, color: 0xFF0000F0 },
            Representation { vertices: vec![(0, 0), (1, 0), (0, 1), (0, 2)], width: 2, height: 3, color: 0xFF0000F0 },
            Representation { vertices: vec![(0, 0), (1, 0), (2, 0), (2, 1)], width: 3, height: 2, color: 0xFF0000F0 },
        ];
        tetrominos.push(Tetromino::new(representations));
        // L Tetromino
        let representations = vec![
            Representation { vertices: vec![(0, 0), (0, 1), (0, 2), (1, 2)], width: 2, height: 3, color: 0xFFF0A000 },
            Representation { vertices: vec![(0, 0), (1, 0), (2, 0), (0, 1)], width: 3, height: 2, color: 0xFFF0A000 },
            Representation { vertices: vec![(0, 0), (1, 0), (1, 1), (1, 2)], width: 2, height: 3, color: 0xFFF0A000 },
            Representation { vertices: vec![(0, 1), (1, 1), (2, 1), (2, 0)], width: 3, height: 2, color: 0xFFF0A000 },
        ];
        tetrominos.push(Tetromino::new(representations));
        // S Tetromino
        let representations = vec![
            Representation { vertices: vec![(1, 0), (2, 0), (0, 1), (1, 1)], width: 3, height: 2, color: 0xFF00F000 },
            Representation { vertices: vec![(0, 0), (0, 1), (1, 1), (1, 2)], width: 2, height: 3, color: 0xFF00F000 },
        ];
        tetrominos.push(Tetromino::new(representations));
        // Z Tetromino
        let representations = vec![
            Representation { vertices: vec![(0, 0), (1, 0), (1, 1), (2, 1)], width: 3, height: 2, color: 0xFFF00000 },
            Representation { vertices: vec![(1, 0), (1, 1), (0, 1), (0, 2)], width: 2, height: 3, color: 0xFFF00000 },
        ];
        tetrominos.push(Tetromino::new(representations));
        // T Tetromino
        let representations = vec![
            Representation { vertices: vec![(0, 0), (1, 0), (2, 0), (1, 1)], width: 3, height: 2, color: 0xFFA000F0 },
            Representation { vertices: vec![(1, 0), (1, 1), (1, 2), (0, 1)], width: 2, height: 3, color: 0xFFA000F0 },
            Representation { vertices: vec![(0, 1), (1, 1), (2, 1), (1, 0)], width: 3, height: 2, color: 0xFFA000F0 },
            Representation { vertices: vec![(0, 0), (0, 1), (0, 2), (1, 1)], width: 2, height: 3, color: 0xFFA000F0 },
        ];
        tetrominos.push(Tetromino::new(representations));

        Self {
            cache: tetrominos,
        }
    }

    pub fn get_random_tetromino(&self) -> Tetromino {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..self.cache.len());
        self.cache[index].clone()
    }
}
