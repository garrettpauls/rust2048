use std::fmt;

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

    pub fn get_empty_cells(&self) -> Vec<(usize, usize)> {
        (0..16)
            .filter(|i| self.cells[*i] == 0_u16)
            .map(|i| {
                let r = i / 4;
                let c = i % 4;
                (r, c)
            })
            .collect()
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
mod test_state {
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

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn get_empty_cells() {
        let mut state = GameState::new();
        state.cells =
            [ 0, 4, 0, 4
            , 4, 0, 4, 0
            , 0, 4, 0, 4
            , 4, 0, 4, 0];
        assert_eq!(state.get_empty_cells(), vec![
            (0, 0), (0, 2),
            (1, 1), (1, 3),
            (2, 0), (2, 2),
            (3, 1), (3, 3)
        ]);
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "    "),
            Cell::Cell(n) => write!(f, "{: >4}", n),
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\
┏━━━━┯━━━━┯━━━━┯━━━━┓
┃{00}│{01}│{02}│{03}┃
┠────┼────┼────┼────┨
┃{04}│{05}│{06}│{07}┃
┠────┼────┼────┼────┨
┃{08}│{09}│{10}│{11}┃
┠────┼────┼────┼────┨
┃{12}│{13}│{14}│{15}┃
┗━━━━┷━━━━┷━━━━┷━━━━┛",
            self.get_cell(0, 0), self.get_cell(0, 1), self.get_cell(0, 2), self.get_cell(0, 3),
            self.get_cell(1, 0), self.get_cell(1, 1), self.get_cell(1, 2), self.get_cell(1, 3),
            self.get_cell(2, 0), self.get_cell(2, 1), self.get_cell(2, 2), self.get_cell(2, 3),
            self.get_cell(3, 0), self.get_cell(3, 1), self.get_cell(3, 2), self.get_cell(3, 3)
        )
    }
}

#[cfg(test)]
mod test_display {
    use state::*;

    #[test]
    fn cell_empty() {
        let cell = Cell::Empty;
        assert_eq!(format!("{}", cell), "    ".to_owned());
    }

    #[test]
    fn cell_digit_one() {
        let cell = Cell::Cell(2);
        assert_eq!(format!("{}", cell), "   2".to_owned());
    }

    #[test]
    fn cell_digit_two() {
        let cell = Cell::Cell(64);
        assert_eq!(format!("{}", cell), "  64".to_owned());
    }

    #[test]
    fn cell_digit_three() {
        let cell = Cell::Cell(512);
        assert_eq!(format!("{}", cell), " 512".to_owned());
    }

    #[test]
    fn cell_digit_four() {
        let cell = Cell::Cell(2048);
        assert_eq!(format!("{}", cell), "2048".to_owned());
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn state() {
        let mut state = GameState::new();
        state.cells =
            [ 2, 512, 0, 256
            , 0, 4, 128, 0
            , 0, 64, 8, 2048
            , 32, 0, 1024, 16];
        assert_eq!(format!("{}", state), "\
┏━━━━┯━━━━┯━━━━┯━━━━┓
┃   2│ 512│    │ 256┃
┠────┼────┼────┼────┨
┃    │   4│ 128│    ┃
┠────┼────┼────┼────┨
┃    │  64│   8│2048┃
┠────┼────┼────┼────┨
┃  32│    │1024│  16┃
┗━━━━┷━━━━┷━━━━┷━━━━┛".to_owned());
    }
}
