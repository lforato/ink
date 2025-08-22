use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{
        Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget,
    },
};

use crate::utils::{get_height, get_longest_string};

#[derive(Debug)]
pub struct Message {
    pub text: String,
    pub scrollbar_state: usize,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::from("
0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz

0-abcdefghijklmnopqrstuvwxyz 1-abcdefghijklmnopqrstuvwxyz 2-abcdefghijklmnopqrstuvwxyz 3-abcdefghijklmnopqrstuvwxyz 4-abcdefghijklmnopqrstuvwxyz 5-abcdefghijklmnopqrstuvwxyz 6-abcdefghijklmnopqrstuvwxyz 7-abcdefghijklmnopqrstuvwxyz 8-abcdefghijklmnopqrstuvwxyz
"),
            scrollbar_state: 0,
        }
    }
}

impl Message {
    pub fn increase_scrollbar_state(&mut self) {
        self.scrollbar_state += 1;
    }

    pub fn decrease_scrollbar_state(&mut self) {
        self.scrollbar_state -= 1;
    }
}

impl Widget for &Message {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let txt = self.text.clone();

        let width = get_longest_string(&txt);
        let layout = Layout::horizontal([Constraint::Percentage(80)]).split(area)[0];

        let scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalTop)
            .end_symbol(Some("]"))
            .thumb_symbol("■")
            .begin_symbol(Some("["))
            .track_symbol(Some("─"));
        let mut scrollbar_state = ScrollbarState::new(width).position(self.scrollbar_state);

        let height = get_height(&txt);
        let layout =
            Layout::vertical([Constraint::Length(2), Constraint::Length(height)]).split(layout);

        StatefulWidget::render(scrollbar, layout[0], buf, &mut scrollbar_state);

        let block = Block::bordered().title("System");

        Paragraph::new(txt)
            .scroll((0, self.scrollbar_state as u16))
            .block(block)
            .render(layout[1], buf);
    }
}
