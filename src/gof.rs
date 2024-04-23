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
pub type Board<'a> = [[&'a mut dyn CellState<'a>; COLS]; ROWS];

pub trait CellState<'a> {
    fn is_alive(&self) -> bool;
    fn set_alive(&mut self, alive: bool);
    fn count_neighbors(&self, slots: &Board<'a>, pos: (usize, usize)) -> i32 {
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
                    && slots[i as usize][j as usize].is_alive()
                {
                    count += 1;
                }
            }
        }
        count
    }
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
    pub board: &'a mut Board<'a>,
    pub marker: std::marker::PhantomData<S>,
}

impl Cell<'_, TwoOrThreeNeighbors> {
    fn spawn(&mut self, pos: (usize, usize)) {
        let (i, j) = pos;
        self.board[i][j].set_alive(true);
    }
    pub fn check(&mut self, pos: (usize, usize)) {
        let (i, j) = pos;
        let neighbors = self.board[i][j].count_neighbors(self.board, pos);
        if neighbors == 3 {
            self.spawn(pos);
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
    fn die(&mut self, pos: (usize, usize)) {
        let (i, j) = pos;
        self.board[i][j].set_alive(false);
    }
    pub fn check(&mut self, pos: (usize, usize)) {
        let (i, j) = pos;
        let neighbors = self.board[i][j].count_neighbors(self.board, pos);
        if !(neighbors == 2 || neighbors == 3) {
            self.die(pos);
        }
    }
}
