use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Widget, WidgetRef},
};

use crate::utils;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Default,
    Debug,
}

impl Mode {
    pub fn next(self) -> Self {
        match self {
            Mode::Default => Mode::Debug,
            Mode::Debug => Mode::Default,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Mode::Default => Mode::Debug,
            Mode::Debug => Mode::Default,
        }
    }

    pub fn to_string(self) -> String {
        match self {
            Mode::Default => String::from("Default"),
            Mode::Debug => String::from("Debug"),
        }
    }

    pub fn len(self) -> u16 {
        2
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Default
    }
}

#[derive(Debug)]
pub struct ChatMode {
    pub mode: Mode,
}

impl Widget for ChatMode {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let centered_box = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Length(5 * self.mode.len()),
                Constraint::Percentage(50),
            ])
            .split(area)[1];

        let centered_box = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Length(100),
                Constraint::Percentage(50),
            ])
            .split(centered_box)[1];

        let option_boxes = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3); self.mode.len().into()])
            .split(centered_box);

        let mut mode = Mode::default();

        option_boxes.into_iter().for_each(|option_box| {
            let style = if self.mode == mode {
                Style::on_light_green(Style::default().black())
            } else {
                Style::on_black(Style::default().light_green())
            };

            let block = Block::default().style(style);

            block.render_ref(*option_box, buf);

            let inner_area = block.inner(*option_box);

            let centered_box = utils::center(
                inner_area,
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            );

            Paragraph::new(mode.to_string())
                .style(style)
                .centered()
                .render(centered_box, buf);

            mode = mode.next()
        });

        Block::default().render(centered_box, buf);
    }
}

impl ChatMode {
    pub fn new() -> Self {
        Self {
            mode: Mode::Default,
        }
    }

    pub fn set_mode(&mut self, mode: &Mode) {
        self.mode = mode.clone()
    }
}
