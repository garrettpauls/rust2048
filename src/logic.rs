use rand::prelude::*;
use state::*;

pub fn new_game() -> GameState {
    let mut state = GameState::new();
    add_tile(&mut state);
    add_tile(&mut state);
    state
}

pub fn add_tile(state: &mut GameState) {
    let mut rng = thread_rng();
    let is_four = rng.gen_range::<u8>(0, 100) < state.four_percentage;
    let cells = state.get_empty_cells();
    let (r, c) = cells[rng.gen_range(0, cells.len())];
    state.set_cell(r, c, Cell::Cell(if is_four { 4 } else { 2 }));
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub fn shift_tiles(mut state: &mut GameState, dir: Direction) -> bool {
    match dir {
        Direction::Left => shift_left(&mut state),
        Direction::Right => shift_right(&mut state),
        Direction::Up => shift_up(&mut state),
        Direction::Down => shift_down(&mut state),
    }
}

#[cfg(test)]
mod test_shift_tiles {
    use logic::*;
    use state::*;

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn empty() {
        let mut state = GameState::from_cells(
            [ 0, 0, 0, 0
            , 0, 0, 0, 0
            , 0, 0, 0, 0
            , 0, 0, 0, 0]);
        let expected = GameState::from_cells(
            [ 0, 0, 0, 0
            , 0, 0, 0, 0
            , 0, 0, 0, 0
            , 0, 0, 0, 0]);
        shift_tiles(&mut state, Direction::Left);
        assert_eq!(state, expected);
        shift_tiles(&mut state, Direction::Right);
        assert_eq!(state, expected);
        shift_tiles(&mut state, Direction::Up);
        assert_eq!(state, expected);
        shift_tiles(&mut state, Direction::Down);
        assert_eq!(state, expected);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn left() {
        let mut state = GameState::from_cells(
            [ 2, 2, 2, 4
            , 4, 4, 4, 4
            , 0, 0, 2, 4
            , 4, 0, 0, 2]);
        let expected = GameState::from_cells(
            [ 4, 2, 4, 0
            , 8, 8, 0, 0
            , 2, 4, 0, 0
            , 4, 2, 0, 0]);
        shift_tiles(&mut state, Direction::Left);
        assert_eq!(state, expected);
    }
}

fn shift_left(mut state: &mut GameState) -> bool {
    let mut changed = false;

    for row in 0..4 {
        for col in 1..4 {
            let prev = state.get_cell(row, col - 1).unwrap();
            let cur = state.get_cell(row, col).unwrap();
            match (prev, cur) {
                (Cell::Empty, Cell::Cell(c)) => {
                    state.set_cell(row, col - 1, Cell::Cell(c));
                    state.set_cell(row, col, Cell::Empty);
                    changed = true;
                }
                (Cell::Cell(p), Cell::Cell(c)) => {
                    if p == c {
                        state.set_cell(row, col - 1, Cell::Cell(p + c));
                        state.set_cell(row, col, Cell::Empty);
                        changed = true;
                    }
                }
                _ => {}
            }
        }
    }

    if changed {
        shift_left(&mut state);
    }

    return changed;
}
fn shift_right(state: &mut GameState) -> bool {
    println!("Shift right");
    return false;
}
fn shift_up(state: &mut GameState) -> bool {
    println!("Shift up");
    return false;
}
fn shift_down(state: &mut GameState) -> bool {
    println!("Shift down");
    return false;
}

pub fn check_state(state: &GameState) -> MoveState {
    const WIN: Cell = Cell::Cell(2048);
    let mut vertical = false;
    let mut horizontal = false;

    for row in 0..4 {
        for col in 0..4 {
            let cell = state.get_cell(row, col).unwrap();
            if cell == WIN {
                return MoveState::Win;
            } else if cell == Cell::Empty {
                horizontal = true;
                vertical = true;
                continue;
            }

            if let Some(h) = state.get_cell(row, col + 1) {
                if h == cell {
                    horizontal = true;
                }
            }

            if let Some(v) = state.get_cell(row + 1, col) {
                if v == cell {
                    vertical = true;
                }
            }
        }
    }

    if vertical || horizontal {
        MoveState::CanMove {
            vertical,
            horizontal,
        }
    } else {
        MoveState::Lose
    }
}

#[cfg(test)]
mod test_state {
    use logic::*;
    use state::GameState;

    #[test]
    fn win() {
        let mut state = GameState::new();
        state.set_cell(3, 3, Cell::Cell(2048));
        assert_eq!(check_state(&state), MoveState::Win);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn lose() {
        let state = GameState::from_cells(
            [ 2, 4, 2, 4
            , 4, 2, 4, 2
            , 2, 4, 2, 4
            , 4, 2, 4, 2]);
        assert_eq!(check_state(&state), MoveState::Lose);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn in_progress_move_vertical() {
        let state = GameState::from_cells(
            [ 2, 4, 2, 4
            , 2, 8, 4, 2
            , 2, 4, 2, 4
            , 4, 2, 4, 2]);
        assert_eq!(
            check_state(&state),
            MoveState::CanMove {
                horizontal: false,
                vertical: true,
            }
        );
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn in_progress_move_horizontal() {
        let state = GameState::from_cells(
            [ 2, 2, 2, 4
            , 4, 8, 4, 2
            , 2, 4, 2, 4
            , 4, 2, 4, 2]);
        assert_eq!(
            check_state(&state),
            MoveState::CanMove {
                horizontal: true,
                vertical: false,
            }
        );
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn in_progress_move_both() {
        let state = GameState::from_cells(
            [ 2, 4, 2, 4
            , 4, 4, 4, 2
            , 2, 4, 2, 4
            , 4, 2, 4, 2]);
        assert_eq!(
            check_state(&state),
            MoveState::CanMove {
                horizontal: true,
                vertical: true,
            }
        );
    }
}
