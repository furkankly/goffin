use anyhow;
use anyhow::Result;
use crossterm::cursor;
use crossterm::event::Event as CrosstermEvent;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::KeyEventKind;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use futures::FutureExt;
use futures::StreamExt;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::io::Stderr;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

use crate::gof::Cell;
use crate::gof::FourOrMoreNeighbors;
use crate::gof::TwoOrThreeNeighbors;
use crate::gof::ZeroOrOneNeighbors;
use crate::ui::ui;

#[derive(Debug)]
pub enum Event {
    Init,
    Key(KeyEvent),
    Render,
}

pub const ROWS: usize = 6;
pub const COLS: usize = 11;
pub type Board = [[Box<Cell>; COLS]; ROWS];

pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<Stderr>>,
    pub task: JoinHandle<()>,
    pub event_tx: UnboundedSender<Event>,
    pub event_rx: UnboundedReceiver<Event>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        Ok(Self {
            terminal: ratatui::Terminal::new(CrosstermBackend::new(std::io::stderr()))?,
            task: tokio::spawn(async {}),
            event_tx,
            event_rx,
        })
    }
    pub fn enter(&mut self) -> Result<()> {
        enable_raw_mode()?;
        execute!(io::stderr(), EnterAlternateScreen, cursor::Hide)?;
        self.start();
        Ok(())
    }

    pub fn exit(&self) -> Result<()> {
        disable_raw_mode()?;
        execute!(io::stderr(), LeaveAlternateScreen, cursor::Show)?;
        Ok(())
    }

    pub fn start(&mut self) {
        let render_delay = std::time::Duration::from_secs_f64(1.0);
        let event_tx_ = self.event_tx.clone();
        self.task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut render_interval = tokio::time::interval(render_delay);
            event_tx_.send(Event::Init).unwrap();
            loop {
                let render_delay = render_interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    maybe_event = crossterm_event => {
                        if let Some(Ok(CrosstermEvent::Key(key))) = maybe_event {
                            if key.kind == KeyEventKind::Press {
                                event_tx_.send(Event::Key(key)).unwrap();
                                }
                            }
                        }
                    _ = render_delay => {
                        event_tx_.send(Event::Render).unwrap();
                    }
                }
            }
        })
    }
}

// A valid initial state is how you play this zero-player game
fn init_board() -> Board {
    std::array::from_fn::<_, ROWS, _>(|i| {
        (0..COLS)
            .map(|j| {
                if (i == 0 && j == 5) || (i == 2 && j == 4) {
                    Box::new(Cell::ZeroOrOneNeighbors(ZeroOrOneNeighbors::new(true)))
                } else if (i == 1 && j == 6) || (i == 2 && j == 5) || (i == 2 && j == 6) {
                    Box::new(Cell::TwoOrThreeNeighbors(TwoOrThreeNeighbors::new(
                        true, false,
                    )))
                } else if (i == 1 && j == 4) || (i == 3 && j == 5) {
                    Box::new(Cell::TwoOrThreeNeighbors(TwoOrThreeNeighbors::new(
                        false, true,
                    )))
                } else if (i == 0 && j == 6)
                    || (i == 1 && j == 7)
                    || (i == 2 && j == 7)
                    || (i == 3 && j == 4)
                    || (i == 3 && j == 6)
                {
                    Box::new(Cell::TwoOrThreeNeighbors(TwoOrThreeNeighbors::new(
                        false, false,
                    )))
                } else if i == 1 && j == 5 {
                    Box::new(Cell::FourOrMoreNeighbors(FourOrMoreNeighbors::new(false)))
                } else {
                    Box::new(Cell::ZeroOrOneNeighbors(ZeroOrOneNeighbors::new(false)))
                }
            })
            .collect::<Vec<Box<Cell>>>()
            .try_into()
            .unwrap()
    })
}

pub async fn run() -> Result<()> {
    let mut tui = Tui::new()?;
    tui.enter()?;

    let mut board = init_board();
    while let Some(event) = tui.event_rx.recv().await {
        match event {
            Event::Key(key) => {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
            Event::Render => {
                tui.terminal.draw(|frame| {
                    ui(frame, &board);
                    Cell::complete_lifecycle(&mut board);
                })?;
            }
            _ => {}
        }
    }

    tui.exit()?;
    Ok(())
}
