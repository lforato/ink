use log::info;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget
    },
};

use crate::utils::{get_height, get_longest_string};

pub const OFFSET: usize = 2;

#[derive(Debug)]
pub struct Message {
    pub text: String,
    pub height: u16,
    /// horizontal scroll position
    pub scroll_state: usize,
    /// horizontal scroll area
    pub scroll_area: usize,
}

impl Message {
    pub fn new(text: String) -> Self {
        let height = get_height(&text) as usize;
        Message {
            text,
            height: height as u16,
            scroll_state: 0,
            scroll_area: 0,
        }
    }

    pub fn prepare(&mut self, area: Rect) {
        let width = get_longest_string(&self.text);
        let viewport_width = area.width as usize;
        let scroll_area = width.saturating_sub(viewport_width);

        self.scroll_area = scroll_area;
    }

    pub fn scroll_right(&mut self) -> () {
        if self.scroll_area > self.scroll_state {
            return;
        }
        self.scroll_state += 1;
    }

    pub fn scroll_left(&mut self) {
        if self.scroll_state == 0 {
            return;
        }
        self.scroll_state -= 1
    }

    pub fn render_horizontal_scrollbar(&mut self, area: Rect, buf: &mut Buffer) {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalTop)
            .end_symbol(Some("▶"))
            .thumb_symbol("■")
            .begin_symbol(Some("◀"))
            .track_symbol(Some("─"));

        let mut scrollbar_state = ScrollbarState::new(self.scroll_area).position(self.scroll_state);

        StatefulWidget::render(scrollbar, area, buf, &mut scrollbar_state);
    }

    pub fn render_partial(&mut self, area: Rect, buf: &mut Buffer, skip_lines: u16) {
        self.prepare(area);
        let txt = self.text.clone();

        let scroll_or_zero = self.scroll_area.min(1) as u16;

        let layout = Layout::vertical([
            Constraint::Length(scroll_or_zero),
            Constraint::Length(self.height - skip_lines),
        ])
        .split(area);

        self.render_horizontal_scrollbar(layout[0], buf);

        let block = Block::default().borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM);

        Paragraph::new(txt)
            .scroll((skip_lines, self.scroll_state as u16))
            .block(block)
            .render(layout[1], buf);
    }
}

impl Widget for &mut Message {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.prepare(area);
        let txt = self.text.clone();

        let scroll_or_zero = self.scroll_area.min(1) as u16;

        let layout = Layout::vertical([
            Constraint::Length(scroll_or_zero),
            Constraint::Length(self.height),
        ])
        .split(area);

        self.render_horizontal_scrollbar(layout[0], buf);

        let block = Block::bordered().title("System");
        Paragraph::new(txt)
            .scroll((0, self.scroll_state as u16))
            .block(block)
            .render(layout[1], buf);
    }
}
