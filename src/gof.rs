use std::borrow::Borrow;

use crate::tui::Board;
use crate::tui::COLS;
use crate::tui::ROWS;

#[derive(Debug)]
pub struct ZeroOrOneNeighbors {
    alive: bool,
}
impl ZeroOrOneNeighbors {
    pub fn new(alive: bool) -> ZeroOrOneNeighbors {
        ZeroOrOneNeighbors { alive }
    }
}
#[derive(Debug)]
pub struct TwoOrThreeNeighbors {
    alive: bool,
    will_spawn: bool,
}
impl TwoOrThreeNeighbors {
    pub fn new(alive: bool, will_spawn: bool) -> TwoOrThreeNeighbors {
        TwoOrThreeNeighbors { alive, will_spawn }
    }
    pub fn will_spawn(&self) -> bool {
        self.will_spawn
    }
}
#[derive(Debug)]
pub struct FourOrMoreNeighbors {
    alive: bool,
}
impl FourOrMoreNeighbors {
    pub fn new(alive: bool) -> FourOrMoreNeighbors {
        FourOrMoreNeighbors { alive }
    }
}

#[derive(Debug)]
pub enum Cell {
    ZeroOrOneNeighbors(ZeroOrOneNeighbors),
    TwoOrThreeNeighbors(TwoOrThreeNeighbors),
    FourOrMoreNeighbors(FourOrMoreNeighbors),
}

impl Cell {
    fn set_alive(&mut self, alive: bool) {
        match self {
            Cell::ZeroOrOneNeighbors(zero_or_one) => zero_or_one.alive = alive,
            Cell::TwoOrThreeNeighbors(two_or_three) => two_or_three.alive = alive,
            Cell::FourOrMoreNeighbors(four_or_more) => four_or_more.alive = alive,
        }
    }
    pub fn is_alive(&self) -> bool {
        match self {
            Cell::ZeroOrOneNeighbors(zero_or_one) => zero_or_one.alive,
            Cell::TwoOrThreeNeighbors(two_or_three) => two_or_three.alive,
            Cell::FourOrMoreNeighbors(four_or_more) => four_or_more.alive,
        }
    }
    fn count_neighbors(pos: (usize, usize), board: &Board) -> i32 {
        let mut count = 0;
        let (i, j) = pos;
        for di in -1..=1 {
            for dj in -1..=1 {
                if !(di == 0 && dj == 0) {
                    let i = i as isize + di;
                    let j = j as isize + dj;
                    if i >= 0 && i < (ROWS as isize) && j >= 0 && j < (COLS as isize) {
                        let cell: &Cell = board[i as usize][j as usize].borrow();
                        if cell.is_alive() {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
    fn live_through(cell: &mut Cell) {
        match cell {
            // die
            Cell::ZeroOrOneNeighbors(_) => cell.set_alive(false),
            // spawn
            Cell::TwoOrThreeNeighbors(two_or_three_neighbors) => {
                if two_or_three_neighbors.will_spawn {
                    cell.set_alive(true)
                }
            }
            // die
            Cell::FourOrMoreNeighbors(_) => cell.set_alive(false),
        }
    }
    fn reincarnate(pos: (usize, usize), board: &mut Board) {
        let (i, j) = pos;
        let cell: &Cell = board[i][j].borrow();
        let neighbors_count = Cell::count_neighbors(pos, board);
        if neighbors_count == 0 || neighbors_count == 1 {
            let new_cell = Cell::ZeroOrOneNeighbors(ZeroOrOneNeighbors {
                alive: cell.is_alive(),
            });
            board[i][j] = Box::new(new_cell);
        } else if neighbors_count == 2 || neighbors_count == 3 {
            let new_cell = Cell::TwoOrThreeNeighbors(TwoOrThreeNeighbors {
                alive: board[i][j].is_alive(),
                will_spawn: !cell.is_alive() && neighbors_count == 3,
            });
            board[i][j] = Box::new(new_cell);
        } else {
            let new_cell = Cell::FourOrMoreNeighbors(FourOrMoreNeighbors {
                alive: cell.is_alive(),
            });
            board[i][j] = Box::new(new_cell);
        }
    }
    pub fn complete_lifecycle(board: &mut Board) {
        for row in board.iter_mut() {
            for cell in row {
                Cell::live_through(cell);
            }
        }
        for i in 0..ROWS {
            for j in 0..COLS {
                Cell::reincarnate((i, j), board);
            }
        }
    }
}
