#[derive(Debug, PartialEq)]
pub enum MoveState {
    Win,
    Lose,
    CanMove { vertical: bool, horizontal: bool },
}

#[derive(Debug, PartialEq)]
pub enum Cell {
    Empty,
    Cell(u16),
}

const WIN: u16 = 2048;

#[derive(Debug, PartialEq)]
pub struct GameState {
    cells: [u16; 16],
}
impl GameState {
    pub fn new() -> GameState {
        GameState { cells: [0; 16] }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Cell {
        let v = self.cells[row * 4 + col];
        match v {
            0 => Cell::Empty,
            _ => Cell::Cell(v),
        }
    }

    pub fn check_state(&self) -> MoveState {
        let mut vertical = false;
        let mut horizontal = false;

        for cell in self.cells.iter() {
            match *cell {
                WIN => return MoveState::Win,
                0 => {
                    vertical = true;
                    horizontal = true;
                }
                _ => {}
            }
        }

        if !vertical {
            vertical = self.can_merge_vertically();
        }

        if !horizontal {
            horizontal = self.can_merge_horizontally();
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

    fn can_merge_vertically(&self) -> bool {
        for col in 0..4 {
            for row in 0..3 {
                let cur = self.get_cell(row, col);
                let next = self.get_cell(row + 1, col);
                if cur != Cell::Empty && cur == next {
                    return true;
                }
            }
        }

        return false;
    }

    fn can_merge_horizontally(&self) -> bool {
        for row in 0..4 {
            for col in 0..3 {
                let cur = self.get_cell(row, col);
                let next = self.get_cell(row, col + 1);
                if cur != Cell::Empty && cur == next {
                    return true;
                }
            }
        }

        return false;
    }
}

#[cfg(test)]
mod test_check_state {
    use state::*;

    #[test]
    fn win() {
        let mut state = GameState::new();
        state.cells[15] = 2048;
        assert_eq!(state.check_state(), MoveState::Win);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn lose() {
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
    fn in_progress_move_vertical() {
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
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn in_progress_move_horizontal() {
        let mut state = GameState::new();
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
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn in_progress_move_both() {
        let mut state = GameState::new();
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
}
