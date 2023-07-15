use ::rand::Rng;
use macroquad::prelude::XY;
use std::fmt::Debug;

pub struct Minefield {
    /// The cell matrix, stored in rows.
    pub cells: Vec<Vec<Cell>>,
    /// Amount of mines.
    pub mine_amt: u16,
    /// Amount of mines that are not yet open.
    pub closed_cells: u16,
}

impl Minefield {
    pub fn new(dims: XY<u16>) -> Self {
        let cells: Vec<Vec<Cell>> = (0..dims.y)
            .map(|_| (0..dims.x).map(|_| Cell::new()).collect())
            .collect();

        let cells_amt = dims.x * dims.y;

        Self {
            cells,
            mine_amt: 0,
            closed_cells: cells_amt,
        }
    }

    pub fn populate(&mut self, rng: &mut impl Rng, mines: u16, safe_pos: XY<usize>) {
        let dims = self.dimensions();
        let cells_amt = dims.x * dims.y;

        let mut safe_zone = self.neighbour_positions(safe_pos);
        safe_zone.push(safe_pos);

        let mut mines_to_insert = mines;
        'mine_insertion: loop {
            for (_, cell) in self
                .cells
                .iter_mut()
                .enumerate()
                .map(|(y, row)| row.iter_mut().enumerate().map(move |(x, c)| ((y, x), c)))
                .flatten()
                .filter(|(pos, cell)| {
                    !cell.is_mine && !safe_zone.contains(&XY { x: pos.1, y: pos.0 })
                })
            {
                if mines_to_insert == 0 {
                    break 'mine_insertion;
                }
                if rng.gen_range(0..cells_amt) == 0 {
                    cell.is_mine = true;
                    mines_to_insert -= 1;
                }
            }
        }

        self.mine_amt = mines;
    }

    #[must_use = "Must check if a mine was opened!"]
    pub fn open(&mut self, pos: XY<usize>) -> bool {
        let cell = &self.cells[pos.y][pos.x];
        if cell.state == CellState::Closed {
            let neighbour_positions = self.neighbour_positions(pos);
            let mine_neighbours = neighbour_positions
                .iter()
                .filter(|&&XY { x, y }| self.cells[y][x].is_mine)
                .count();

            // We re-index to get a mutable reference only after we have went over the neighbours.
            let cell = &mut self.cells[pos.y][pos.x];
            cell.state = CellState::Open(mine_neighbours as u8);

            if cell.is_mine {
                return true;
            }

            // Automatically open surrounding cells if there are no neighbouring mines.
            if mine_neighbours == 0 {
                for neighbour_pos in neighbour_positions {
                    let _ = self.open(neighbour_pos);
                }
            }

            self.closed_cells -= 1;
        }

        false
    }

    pub fn toggle_flag(&mut self, pos: XY<usize>) {
        let cell = &mut self.cells[pos.y][pos.x];
        match cell.state {
            CellState::Closed => cell.state = CellState::Flagged,
            CellState::Flagged => cell.state = CellState::Closed,
            _ => (),
        }
    }

    fn neighbour_positions(&self, pos: XY<usize>) -> Vec<XY<usize>> {
        let mut neighbours = vec![];

        let left = pos.x > 0;
        let right = pos.x < self.cells[0].len() - 1;
        let up = pos.y > 0;
        let down = pos.y < self.cells.len() - 1;

        if left {
            neighbours.push(XY {
                x: pos.x - 1,
                ..pos
            });
        }
        if right {
            neighbours.push(XY {
                x: pos.x + 1,
                ..pos
            })
        }
        if up {
            neighbours.push(XY {
                y: pos.y - 1,
                ..pos
            });
        }
        if down {
            neighbours.push(XY {
                y: pos.y + 1,
                ..pos
            })
        }
        if left && up {
            neighbours.push(XY {
                x: pos.x - 1,
                y: pos.y - 1,
            })
        }
        if right && up {
            neighbours.push(XY {
                x: pos.x + 1,
                y: pos.y - 1,
            })
        }
        if left && down {
            neighbours.push(XY {
                x: pos.x - 1,
                y: pos.y + 1,
            })
        }
        if right && down {
            neighbours.push(XY {
                x: pos.x + 1,
                y: pos.y + 1,
            });
        }

        neighbours
    }

    pub fn dimensions(&self) -> XY<usize> {
        XY {
            x: self.cells[0].len(),
            y: self.cells.len(),
        }
    }

    /// Whether all non-mine cells are open.
    pub fn is_clear(&self) -> bool {
        self.closed_cells == self.mine_amt
    }
}

impl Debug for Minefield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{cell:?} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// A single square - or cell - on the game field.
pub struct Cell {
    pub state: CellState,
    pub is_mine: bool,
}

impl Cell {
    fn new() -> Self {
        Self {
            is_mine: false,
            state: CellState::Closed,
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.is_mine { "!" } else { "#" })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CellState {
    /// A closed cell.
    Closed,
    /// An unopened cell which has been flagged.
    Flagged,
    /// An exposed cell, containing the number of neighbouring mines.
    Open(u8),
}
