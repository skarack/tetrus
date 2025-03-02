mod bitmap;
mod display;
mod board;
mod tetromino;

use std::usize;

use board::Board;
use display::Display;
use minifb::{Key, Window, WindowOptions};
use tetromino::Tetromino;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;
const PIXEL_SIZE: usize = 16;

struct GameState {
    display: Display,
    state: State,
    next_tetromino: Option<Tetromino>,
}

#[derive(PartialEq, Debug)]
enum State {
    NewGame,
    DropBlock,
    Playing,
    NewTetrominoNeeded,
    UpdateScore(i32),
}

fn main() {
    let mut board = Board::new(1, 5);
    let mut gs = GameState {
        display: Display::new(WIDTH, HEIGHT, PIXEL_SIZE, vec![0; WIDTH * HEIGHT]),
        state: State::NewGame,
        next_tetromino: None,
    };

    let mut window = Window::new(
        "Tetrust+",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);
    let mut score = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        gs.display.clear_buffer();
        gs.display.draw_text(28, 1, "tetris!".to_string());

        match gs.state {
            State::NewGame => gs.state = State::NewTetrominoNeeded,
            State::NewTetrominoNeeded => {
                board.place_new_tetromino(&mut gs);
                gs.state = State::Playing;
            }
            State::UpdateScore(line_erased) => {
                gs.state = State::NewTetrominoNeeded;
                score += match line_erased
                {
                    1 => 40,
                    2 => 100,
                    3 => 300,
                    _ => 1200
                };
            }
            State::Playing => {
                board.update(&mut gs, &window.get_keys_pressed(minifb::KeyRepeat::Yes));
            }
            _ => {},
        }

        board.render(&mut gs);
        draw_next_tetromino(&mut gs);
        draw_score(score, &mut gs);

        window.update_with_buffer(&gs.display.buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn draw_next_tetromino(gs: &mut GameState) {
    gs.display.draw_text(30, 8, "NEXT".to_string());
    if let Some(tetromino) = &gs.next_tetromino {
        tetromino.render(30, 9, &mut gs.display);
    }
}

fn draw_score(score: i32, gs: &mut GameState) {
    gs.display.draw_text(30, 14, "SCORE".to_string());
    gs.display.draw_text(30, 15, score.to_string());
}
