use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use ratatui::prelude::CrosstermBackend;
use ratatui::style::Color;
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::canvas::Line;
use ratatui::widgets::canvas::Map;
use ratatui::widgets::canvas::MapResolution;
use ratatui::widgets::canvas::Rectangle;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::Terminal;
use std::error::Error;
use std::io;
use std::io::Stdout;
use std::time::Duration;

use crate::gof::Cell;
use crate::gof::CellState;
use crate::gof::ZeroOrOneNeighbors;
use crate::gof::COLS;
use crate::gof::ROWS;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let canvas = Canvas::default()
        .block(Block::default().title("Canvas").borders(Borders::ALL))
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&Map {
                resolution: MapResolution::High,
                color: Color::White,
            });
            ctx.layer();
            ctx.draw(&Line {
                x1: 0.0,
                y1: 10.0,
                x2: 10.0,
                y2: 10.0,
                color: Color::White,
            });
            ctx.draw(&Rectangle {
                x: 10.0,
                y: 20.0,
                width: 10.0,
                height: 10.0,
                color: Color::Red,
            });
        });

    let mut cells = std::array::from_fn::<_, ROWS, _>(|_| {
        [(); COLS].map(|_| (ZeroOrOneNeighbors { alive: false }))
    });
    let mut board = cells
        .each_mut()
        .map(|row| row.each_mut().map(|cell| cell as &mut dyn CellState));
    let board = &mut board;
    let cells = cells.map(|row| {
        row.map(|_cell| Cell {
            board,
            marker: std::marker::PhantomData::<ZeroOrOneNeighbors>,
        })
    });

    loop {
        terminal.draw(|frame| {
            frame.render_widget(&canvas, frame.size());
        })?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }
            }
        }
    }
    Ok(())
}

// const ROWS: usize = 10;
// const COLS: usize = 10;
// #[derive(Copy, Clone)]
// struct MyStruct {
//     x: i32,
//     y: i32,
// }
// trait MyTrait<'a> {}
// impl MyTrait<'_> for MyStruct {}
//
// fn main() {
//     let mut arr =
//         std::array::from_fn::<_, ROWS, _>(|_| [(); COLS].map(|_| MyStruct { x: 0, y: 0 }));
//     let mut refs = arr.each_mut().map(|row| row.each_mut());
//
//     pub type Cluster<'a> = [[&'a mut dyn MyTrait<'a>; COLS]; ROWS];
//     struct AnonStruct<'a> {
//         cluster: &'a mut Cluster<'a>,
//     }
//     let mut cluster = refs.map(|row| row.map(|i| i as &mut dyn MyTrait));
//     let anon = AnonStruct {
//         cluster: &mut cluster,
//     };
// }
