use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;
use ratatui::Frame;
use std::rc::Rc;

use crate::gof::Cell;
use crate::tui::Board;
use crate::tui::COLS;
use crate::tui::ROWS;

const ALIVE_BG_COLOR: Color = Color::Yellow;
const DEAD_BG_COLOR: Color = Color::Black;
const ZERO_OR_ONE_COLOR: Color = Color::White;
const TWO_OR_THREE_COLOR: Color = Color::Magenta;
const FOUR_OR_MORE_COLOR: Color = Color::Blue;

struct StateUI {
    state_text_style: Style,
    state_text: String,
    extra_text_style: Style,
    extra_text: Option<String>,
}

fn get_state_ui(cell: &Cell) -> StateUI {
    match cell {
        Cell::ZeroOrOneNeighbors(_) => StateUI {
            state_text: String::from("ZERO OR ONE NEIGHBORS"),
            state_text_style: Style::default().fg(ZERO_OR_ONE_COLOR),
            extra_text: if cell.is_alive() {
                Some(String::from("WILL DIE"))
            } else {
                None
            },
            extra_text_style: Style::default().bg(Color::Red).bold(),
        },
        Cell::TwoOrThreeNeighbors(two_or_three) => StateUI {
            state_text: String::from("TWO OR THREE NEIGHBORS"),
            state_text_style: Style::default().fg(TWO_OR_THREE_COLOR),
            extra_text: if two_or_three.will_spawn() {
                Some(String::from("WILL SPAWN"))
            } else {
                None
            },
            extra_text_style: Style::default().bg(Color::Yellow).bold(),
        },
        Cell::FourOrMoreNeighbors(_) => StateUI {
            state_text: String::from("FOUR OR MORE NEIGHBORS"),
            state_text_style: Style::default().fg(FOUR_OR_MORE_COLOR),
            extra_text: if cell.is_alive() {
                Some(String::from("WILL DIE"))
            } else {
                None
            },
            extra_text_style: Style::default().bg(Color::Red).bold(),
        },
    }
}

pub fn ui(frame: &mut Frame, board: &Board) {
    let layout_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100 / (ROWS as u16)); ROWS])
        .split(frame.size());

    let layout_cells: [Rc<[Rect]>; ROWS] = (0..ROWS)
        .enumerate()
        .map(|(i, _)| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100 / (COLS as u16)); COLS])
                .split(layout_rows[i])
        })
        .collect::<Vec<Rc<[Rect]>>>()
        .try_into()
        .unwrap();

    for (i, row) in board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            let state_ui = get_state_ui(cell);
            let state_line = Line::from(state_ui.state_text).style(state_ui.state_text_style);
            let extra_line = if let Some(extra_text) = state_ui.extra_text {
                Line::from(extra_text).style(state_ui.extra_text_style)
            } else {
                Line::from("")
            };
            let text = Text::from(vec![state_line, extra_line]);
            frame.render_widget(
                Paragraph::new(text)
                    .centered()
                    .wrap(Wrap { trim: true })
                    .block(Block::new().borders(Borders::ALL).bg(if cell.is_alive() {
                        ALIVE_BG_COLOR
                    } else {
                        DEAD_BG_COLOR
                    })),
                layout_cells[i][j],
            );
        }
    }
}
