use log::info;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget},
};

use crate::widgets::message::{Message, OFFSET};

#[derive(Debug)]
pub struct Chat {
    pub height: usize,
    pub messages: Vec<Message>,
    /// used to render the scrollbar, represents where
    /// in the scroll_area that the scrollbar is located
    pub scroll_state: usize,
    /// used to render the scrollbar, it represents
    /// how much space the scrollbar will have for scrolling.
    pub scroll_area: usize,
}

impl Chat {
    pub fn new(messages: Vec<String>) -> Self {
        let messages = messages
            .into_iter()
            .map(|item| Message::new(item))
            .collect();

        Self {
            messages,
            height: 0,
            scroll_area: 0,
            scroll_state: 0,
        }
    }

    pub fn scroll_up(&mut self) -> () {
        if self.scroll_state == 0 {
            return;
        }
        self.scroll_state -= 1
    }

    pub fn scroll_down(&mut self) -> () {
        if self.scroll_state > self.scroll_area {
            return;
        }
        self.scroll_state += 1;
    }

    pub fn set_scroll_area(&mut self, scroll_area: usize) -> () {
        self.scroll_area = scroll_area;
    }

    pub fn render_vertical_scrollbar(&mut self, area: Rect, buf: &mut Buffer) {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("│"));

        let viewport_height = area.height as usize;
        let scrollable_height = self.height.saturating_sub(viewport_height);
        self.set_scroll_area(scrollable_height);

        let mut scrollbar_state =
            ScrollbarState::new(scrollable_height).position(self.scroll_state);

        StatefulWidget::render(scrollbar, area, buf, &mut scrollbar_state);
    }
}

impl Widget for &mut Chat {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Max(2),
        ])
        .split(area);

        Block::bordered().render(layout[1], buf);
        let chat_inner = Block::bordered().inner(layout[1]);

        let total_height: u16 = self.messages.iter().map(|m| m.height).sum();
        self.height = total_height as usize + OFFSET;

        let scroll_top = self.scroll_state as i32;
        let visible_height = chat_inner.height as i32;

        let mut y = chat_inner.y as i32;

        for item in self.messages.iter_mut() {
            let h = item.height as i32;

            let msg_top = y - chat_inner.y as i32;
            let msg_bottom = msg_top + h;

            let visible_top = scroll_top.max(msg_top);
            let visible_bottom = (scroll_top + visible_height) as i32;
            let clip_start = (visible_top - msg_top).max(0);
            let clip_height = (msg_bottom.min(visible_bottom) - visible_top).max(0);

            if clip_height > 0 {
                let rect = Rect {
                    x: chat_inner.x,
                    y: (chat_inner.y as i32 + visible_top - scroll_top as i32) as u16,
                    width: chat_inner.width,
                    height: clip_height as u16,
                };

                if clip_start > 0 {
                    item.render_partial(rect, buf, (clip_start-1) as u16);
                } else {
                    item.render(rect, buf);
                }
            }

            y += h;
        }

        self.render_vertical_scrollbar(layout[2], buf);
    }
}
