extern crate getch;
extern crate rand;

mod logic;
mod state;
use getch::Getch;
use logic::{check_state, new_game, shift_tiles, Direction};
use state::*;

fn main() {
    let g = Getch::new();
    let mut state = new_game();

    println!("Press left/right/up/down arrow keys to play, or Q to quit.");

    let mut h = true;
    let mut v = true;

    loop {
        println!("{}", state);
        if let Some(dir) = get_input(&g) {
            if !h && (dir == Direction::Left || dir == Direction::Right) {
                continue;
            }
            if !v && (dir == Direction::Up || dir == Direction::Down) {
                continue;
            }

            clear_console();
            shift_tiles(&mut state, dir);
            match check_state(&state) {
                MoveState::Win => {
                    println!("You win!");
                    return;
                }
                MoveState::Lose => {
                    println!("You lose!");
                    return;
                }
                MoveState::CanMove {
                    vertical,
                    horizontal,
                } => {
                    h = horizontal;
                    v = vertical;
                }
            }
        } else {
            // Quit
            return;
        }
    }
}

fn get_input(g: &Getch) -> Option<Direction> {
    const UPPER_Q: u8 = 81;
    const LOWER_Q: u8 = 113;
    const ARROW_PREFIX: u8 = 224;
    const ARROW_LEFT: u8 = 75;
    const ARROW_UP: u8 = 72;
    const ARROW_RIGHT: u8 = 77;
    const ARROW_DOWN: u8 = 80;

    loop {
        match g.getch() {
            Ok(UPPER_Q) => return None,
            Ok(LOWER_Q) => return None,
            Ok(ARROW_PREFIX) => match g.getch() {
                Ok(ARROW_LEFT) => return Some(Direction::Left),
                Ok(ARROW_UP) => return Some(Direction::Up),
                Ok(ARROW_RIGHT) => return Some(Direction::Right),
                Ok(ARROW_DOWN) => return Some(Direction::Down),
                Err(e) => eprintln!("Failed to read key input: {}", e),
                _ => continue,
            },
            Err(e) => eprintln!("Failed to read key input: {}", e),
            _ => continue,
        }
    }
}

fn clear_console() {
    use std::io::*;
    stdout().write_all("\x1b[2J\x1b[1;1H".as_bytes()).unwrap();
}
