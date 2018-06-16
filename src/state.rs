#[derive(Debug)]
pub struct GameState {
    cells: [u16; 16],
}
impl GameState {
    pub fn new() -> GameState {
        GameState { cells: [0; 16] }
    }
}
