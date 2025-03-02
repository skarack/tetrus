use std::collections::HashMap;

use crate::bitmap::load_bitmap;

const CLEAR_COLOR: u32 = 0x1D1D1D;
const BLOCK_SIZE: usize = 16;

pub struct Display {
    pub width: usize,
    pub height: usize,
    pub virtual_width: usize,
    pub virtual_height: usize,
    pub pixel_size: usize,
    pub buffer: Vec<u32>,
    block_factory: BlockFactory,
    glyph_cache: GlyphCache,
}

impl Display {
    pub fn new(width: usize, height: usize, pixel_size: usize, buffer: Vec<u32>) -> Self {
        Self {
            width,
            height,
            virtual_width: width / pixel_size,
            virtual_height: height / pixel_size,
            pixel_size,
            buffer,
            block_factory: BlockFactory::new(),
            glyph_cache: GlyphCache::new(),
        }
    }

    pub fn clear_buffer(&mut self) {
        for pixel in &mut self.buffer {
            *pixel = CLEAR_COLOR;
        }
    }

    pub fn draw_block(&mut self, color: u32, x: u32, y: u32) {
        assert!(x + (BLOCK_SIZE as u32 / self.pixel_size as u32) < self.virtual_width as u32 && y + (BLOCK_SIZE as u32 / self.pixel_size as u32) < self.virtual_height as u32, "Drawing block outside buffer boundaries");

        if let Some(block) = self.block_factory.get_block(color) {
            for h in 0..BLOCK_SIZE {
                for w in 0..BLOCK_SIZE {
                    let src_index = (BLOCK_SIZE * h) + w;
                    let dst_index = (self.width as u32 * (h as u32 + y * self.pixel_size as u32)) + (w as u32 + x * self.pixel_size as u32);

                    self.buffer[dst_index as usize] = block[src_index as usize];
                }
            }
        } else { 
            panic!("Couldn't create block") 
        }
    }

    pub fn draw_text(&mut self, x: u32, y: u32, text: String) {
        let mut char_x = x;
        for c in text.to_uppercase().chars() {
            if let Some(glyph) = self.glyph_cache.get_glyph(&c) {
                for h in 0..BLOCK_SIZE {
                    for w in 0..BLOCK_SIZE {
                        let src_index = (BLOCK_SIZE * h) + w;
                        let dst_index = (self.width as u32 * (h as u32 + y * self.pixel_size as u32)) + (w as u32 + char_x * self.pixel_size as u32);

                        self.buffer[dst_index as usize] = glyph[src_index as usize];
                    }
                }
            } else { 
                panic!("Writing text with unknown glyph") 
            }

            char_x += 1;
        }
    }
}

struct BlockFactory {
    cache: HashMap<u32, Vec<u32>>,
    block_template: Vec<u32>
}

impl BlockFactory {
    fn new() -> Self {
        let template = match load_bitmap("./block.bmp") {
            Ok(bitmap) => bitmap,
            Err(error) => panic!("{}", error)
        };
        assert!(template.len() == BLOCK_SIZE * BLOCK_SIZE, "Block bitmap should be 16x16");

        Self {
            cache: HashMap::new(),
            block_template: template
        }
    }

    fn get_block(&mut self, color: u32) -> Option<&Vec<u32>> {
        if !self.cache.contains_key(&color) {
            let new_block = self.create_colored_block(color);
            self.cache.insert(color, new_block);
        }

        self.cache.get(&color)
    }

    fn create_colored_block(&self, color: u32) -> Vec<u32> {
        let mut new_block = self.block_template.clone();
        for pixel in &mut new_block {
            let lum_r = ((*pixel >> 16) & 0xFF) as f32 / 255.0;
            let lum_g = ((*pixel >> 8) & 0xFF) as f32 / 255.0;
            let lum_b = (*pixel & 0xFF) as f32 / 255.0;

            let new_r = (color >> 16) & 0xFF;
            let new_g = (color >> 8) & 0xFF;
            let new_b = color & 0xFF;

            *pixel = ((new_r as f32 * lum_r) as u32) << 16 | ((new_g as f32 * lum_g) as u32) << 8 | (new_b as f32 * lum_b) as u32;
        }

        new_block
    }
}

struct GlyphCache {
    cache: HashMap<char, Vec<u32>>,
}

impl GlyphCache {
    fn new() -> Self {
        const GLYPHS_WIDTH: u32 = 64;
        const GLYPHS_HEIGHT: u32 = 80;
        let keys: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'T', 'I', 'S', 'C', 'O', 'R', 'E', 'N', 'X', '!'];

        let glyphs = match load_bitmap("./glyphs.bmp") {
            Ok(bitmap) => bitmap,
            Err(error) => panic!("{}", error)
        };
        assert!(glyphs.len() == (GLYPHS_WIDTH * GLYPHS_HEIGHT) as usize, "Glyphs bitmap should be 64x80");

        let mut cache: HashMap<char, Vec<u32>> = HashMap::with_capacity(keys.len());

        for (i, k) in keys.iter().enumerate() {
            let y = i as u32 / (GLYPHS_WIDTH / BLOCK_SIZE as u32);
            let x = i as u32 % (GLYPHS_WIDTH / BLOCK_SIZE as u32);

            let mut glyph: Vec<u32> = vec![0; BLOCK_SIZE * BLOCK_SIZE];

            for h in 0..BLOCK_SIZE {
                for w in 0..BLOCK_SIZE {
                    let dst_index = (BLOCK_SIZE * h) + w;
                    let src_index = (GLYPHS_WIDTH * (h as u32 + (y * BLOCK_SIZE as u32))) + (w as u32 + (x * BLOCK_SIZE as u32));

                    glyph[dst_index as usize] = glyphs[src_index as usize];
                }
            }

            cache.insert(*k, glyph);
        }

        Self {
            cache,
        }
    }

    fn get_glyph(&self, key: &char) -> Option<&Vec<u32>> {
        self.cache.get(key)
    }
}
