#[derive(Debug, PartialEq)]
pub enum MoveState {
    Win,
    Lose,
    CanMove { vertical: bool, horizontal: bool },
}

#[derive(Debug)]
pub struct GameState {
    cells: [u16; 16],
}
impl GameState {
    pub fn new() -> GameState {
        GameState { cells: [0; 16] }
    }
    pub fn check_state(&self) -> MoveState {
        MoveState::CanMove {
            vertical: false,
            horizontal: false,
        }
    }
}

#[test]
fn check_state_win() {
    let mut state = GameState::new();
    state.cells[0] = 2048;
    assert_eq!(state.check_state(), MoveState::Win);
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn check_state_lose() {
    let mut state = GameState::new();
    state.cells =
        [ 2, 4, 2, 4
        , 4, 2, 4, 2
        , 2, 4, 2, 4
        , 4, 2, 4, 2];
    assert_eq!(state.check_state(), MoveState::Lose);
}

#[test]
#[cfg_attr(rustfmt, rustfmt_skip)]
fn check_state_in_progress() {
    let mut state = GameState::new();

    state.cells =
        [ 2, 4, 2, 4
        , 2, 8, 4, 2
        , 2, 4, 2, 4
        , 4, 2, 4, 2];
    assert_eq!(
        state.check_state(),
        MoveState::CanMove {
            horizontal: false,
            vertical: true,
        }
    );
    
    state.cells =
        [ 2, 2, 2, 4
        , 4, 8, 4, 2
        , 2, 4, 2, 4
        , 4, 2, 4, 2];
    assert_eq!(
        state.check_state(),
        MoveState::CanMove {
            horizontal: true,
            vertical: false,
        }
    );
    
    state.cells =
        [ 2, 4, 2, 4
        , 4, 4, 4, 2
        , 2, 4, 2, 4
        , 4, 2, 4, 2];
    assert_eq!(
        state.check_state(),
        MoveState::CanMove {
            horizontal: true,
            vertical: true,
        }
    );
}
