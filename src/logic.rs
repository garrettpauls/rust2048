use state::*;

const WIN: Cell = Cell::Cell(2048);

pub fn check_state(state: &GameState) -> MoveState {
    let mut vertical = false;
    let mut horizontal = false;

    for row in 0..4 {
        for col in 0..4 {
            let cell = state.get_cell(row, col);
            if cell == WIN {
                return MoveState::Win;
            } else if cell == Cell::Cell(0) {
                vertical = true;
                horizontal = true;
            }
        }
    }

    if !vertical {
        vertical = can_merge_vertically(state);
    }

    if !horizontal {
        horizontal = can_merge_horizontally(state);
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

fn can_merge_vertically(state: &GameState) -> bool {
    for col in 0..4 {
        for row in 0..3 {
            let cur = state.get_cell(row, col);
            let next = state.get_cell(row + 1, col);
            if cur != Cell::Empty && cur == next {
                return true;
            }
        }
    }

    return false;
}

fn can_merge_horizontally(state: &GameState) -> bool {
    for row in 0..4 {
        for col in 0..3 {
            let cur = state.get_cell(row, col);
            let next = state.get_cell(row, col + 1);
            if cur != Cell::Empty && cur == next {
                return true;
            }
        }
    }

    return false;
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
