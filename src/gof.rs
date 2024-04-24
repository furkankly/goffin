use std::borrow::BorrowMut;

#[derive(Debug, Clone, Copy)]
pub struct ZeroOrOneNeighbors {
    pub alive: bool,
}
#[derive(Debug, Clone, Copy)]
pub struct TwoOrThreeNeighbors {
    pub alive: bool,
}
#[derive(Debug, Clone, Copy)]
pub struct FourOrMoreNeighbors {
    pub alive: bool,
}

pub const ROWS: usize = 11;
pub const COLS: usize = 11;
pub type Board<'a, S> = [[&'a mut Cell<'a, S>; COLS]; ROWS];

pub trait CellState<'a> {
    fn is_alive(&self) -> bool;
    fn set_alive(&mut self, alive: bool);
}
impl<'a> CellState<'a> for ZeroOrOneNeighbors {
    fn set_alive(&mut self, alive: bool) {
        self.alive = alive
    }
    fn is_alive(&self) -> bool {
        self.alive
    }
}
impl<'a> CellState<'a> for TwoOrThreeNeighbors {
    fn set_alive(&mut self, alive: bool) {
        self.alive = alive
    }
    fn is_alive(&self) -> bool {
        self.alive
    }
}
impl<'a> CellState<'a> for FourOrMoreNeighbors {
    fn set_alive(&mut self, alive: bool) {
        self.alive = alive
    }
    fn is_alive(&self) -> bool {
        self.alive
    }
}

pub struct Cell<'a, S: CellState<'a>> {
    pub state: &'a mut S,
}

impl<'a, S> Cell<'a, S>
where
    S: CellState<'a>,
{
    fn count_neighbors(pos: (usize, usize), board: &mut Board<'a, S>) -> i32 {
        let cell = board[pos.0][pos.1].borrow_mut();
        let mut count = 0;
        let (i, j) = pos;
        for di in -1..=1 {
            for dj in -1..=1 {
                let i = i as isize + di;
                let j = j as isize + dj;

                if i >= 0
                    && i < ROWS as isize
                    && j >= 0
                    && j < COLS as isize
                    && cell.state.is_alive()
                {
                    count += 1;
                }
            }
        }
        count
    }
}

impl<'a> Cell<'a, TwoOrThreeNeighbors> {
    fn spawn(cell: &mut Cell<'a, TwoOrThreeNeighbors>) {
        cell.state.set_alive(true);
    }
    pub fn check(pos: (usize, usize), board: &mut Board<'a, TwoOrThreeNeighbors>) {
        let neighbors = Cell::count_neighbors(pos, board);
        if neighbors == 3 {
            let cell = board[pos.0][pos.1].borrow_mut();
            Cell::spawn(cell);
        }
    }
}

pub trait DyingState {}
impl DyingState for ZeroOrOneNeighbors {}
impl DyingState for FourOrMoreNeighbors {}
impl<'a, S> Cell<'a, S>
where
    S: DyingState + CellState<'a>,
{
    fn die(cell: &mut Cell<'a, S>) {
        cell.state.set_alive(false);
    }
    pub fn check(pos: (usize, usize), board: &mut Board<'a, S>) {
        let neighbors = Cell::count_neighbors(pos, board);
        if !(neighbors == 2 || neighbors == 3) {
            let cell = board[pos.0][pos.1].borrow_mut();
            Cell::die(cell);
        }
    }
}
