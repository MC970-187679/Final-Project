use crate::cell::{Cell, Grid};

use super::Engine;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SerialEngine;

impl SerialEngine {
    #[must_use]
    fn next_cell_at(grid: &Grid, row: usize, col: usize) -> Cell {
        let start_row = row.saturating_sub(1);
        let start_col = col.saturating_sub(1);

        let mut live_cells = 0;

        for i in start_row..start_row+3 {
            for j in start_col..start_col+3 {
                if (i, j) != (row, col) && grid.get_cell(i, j) == Some(&Cell::Live) {
                    live_cells += 1
                }
            }
        }

        if live_cells == 3 || (live_cells == 2 && grid[row][col].is_live()) {
            Cell::Live
        } else {
            Cell::Dead
        }
    }

    #[must_use]
    fn prepare_next_grid(grid: &Grid) -> Grid {
        let mut next = Grid::new_with(grid.rows(), grid.columns(), Cell::Dead);

        for (row, cells) in next.iter_mut().enumerate() {
            for (col, cell) in cells.iter_mut().enumerate() {
                if Self::next_cell_at(grid, row, col).is_live() {
                    *cell = Cell::Live
                }
            }
        }

        next
    }
}

impl Engine for SerialEngine {
    #[inline]
    #[must_use]
    fn update(&self, grid: &Grid) -> Grid {
        Self::prepare_next_grid(grid)
    }
}
