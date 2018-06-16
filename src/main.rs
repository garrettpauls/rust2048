mod state;
use state::GameState;

fn main() {
    let state = GameState::new();
    println!("{:?}", state);
}
