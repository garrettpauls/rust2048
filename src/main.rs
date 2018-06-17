extern crate rand;

mod logic;
mod state;
use logic::new_game;

fn main() {
    let state = new_game();
    println!("{}", state);
}
