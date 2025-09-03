use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget},
};

use crate::widgets::{
    message::{Message, OFFSET},
    textarea::TextArea,
};

#[derive(Debug)]
pub struct Chat<'a> {
    pub selected_message_id: usize,
    pub messages: Vec<Message>,
    pub textarea: TextArea<'a>,
    /// the total height of the chat screen including the entire chat history
    pub height: usize,
    /// used to render the scrollbar, represents where
    /// in the scroll_area that the scrollbar is located
    pub scroll_state: usize,
    /// used to render the scrollbar, it represents
    /// how much space the scrollbar will have for scrolling
    pub scroll_area: usize,
    /// if the LLM is generating the response
    pub generating_response: bool
}

pub const MARGIN: i32 = 1;

impl<'a> Chat<'a> {
    pub fn new(input: Vec<String>) -> Self {
        let mut messages: Vec<Message> = input
            .into_iter()
            .enumerate()
            .map(|(i, item)| Message::new(i, item))
            .collect();

        if messages.len() > 0 {
            messages[0].is_selected = true;
        }

        let textarea = TextArea::default();

        Self {
            messages,
            textarea,
            height: 0,
            scroll_area: 0,
            scroll_state: 0,
            selected_message_id: 0,
            generating_response: false,
        }
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_state == 0 {
            return;
        }
        self.scroll_state -= 1;
    }

    pub fn scroll_down(&mut self) {
        if self.scroll_state > self.scroll_area {
            return;
        }
        self.scroll_state += 1;
    }

    pub fn select_next(&mut self) {
        if self.messages.is_empty() {
            return;
        }

        if self.selected_message_id + 1 >= self.messages.len() {
            return;
        }

        self.messages[self.selected_message_id].is_selected = false;
        self.selected_message_id += 1;
        self.messages[self.selected_message_id].is_selected = true;
    }

    pub fn select_prev(&mut self) {
        if self.messages.is_empty() || self.selected_message_id == 0 {
            return;
        }

        self.messages[self.selected_message_id].is_selected = false;
        self.selected_message_id -= 1;
        self.messages[self.selected_message_id].is_selected = true;
    }

    pub fn set_scroll_area(&mut self, scroll_area: usize) -> () {
        self.scroll_area = scroll_area;
    }

    pub fn push_msg(&mut self, value: String) -> usize {
        let idx = self.messages.len();
        let msg = Message::new(idx, value);
        self.messages.push(msg);
        idx
    }

    pub fn alter_msg(&mut self, idx: usize, value: String) -> Result<(), &str> {
        if self.messages.len() <= idx {
            return Err("Failed to alter message");
        }
        self.messages[idx] = Message::new(idx, value);
        Ok(())
    }

    pub fn render_vertical_scrollbar(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        viewport_height: usize,
    ) {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("│"));

        let scrollable_height = self.height.saturating_sub(viewport_height);
        self.set_scroll_area(scrollable_height);

        let mut scrollbar_state =
            ScrollbarState::new(scrollable_height).position(self.scroll_state);

        StatefulWidget::render(scrollbar, area, buf, &mut scrollbar_state);
    }
}

impl<'a> Widget for &mut Chat<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Max(2),
        ])
        .split(area);

        Block::bordered().render(layout[1], buf);
        let chat_inner = Block::bordered().inner(layout[1]);

        let chat_inner_layout =
            Layout::vertical([Constraint::Percentage(100), Constraint::Length(10)])
                .split(chat_inner);

        let chat_inner = chat_inner_layout[0];
        let chat_textarea = chat_inner_layout[1];

        let total_height: u16 = self.messages.iter().map(|m| m.height).sum();
        self.height = total_height as usize + OFFSET;

        let scroll_top = self.scroll_state as i32;
        let visible_height = chat_inner.height as i32;

        let mut y = chat_inner.y as i32;
        let mut new_id = self.selected_message_id;
        let len = self.messages.len();

        self.textarea.render(chat_textarea, buf);

        for item in self.messages.iter_mut() {
            let h = item.height as i32;

            let msg_top = y - chat_inner.y as i32;
            let msg_bottom = msg_top + h;

            // if we could see the entire page, what line is the last one that is visible at the
            // moment.
            // ex:
            // 1
            // 2 | top
            // 3 | bottom
            // 4
            // 5
            // we can say in the example that visible_bottom is 3 because it is the last line
            // visible, and we can say that 2 is the visible_top for the same reason

            let visible_top = scroll_top.max(msg_top);
            let visible_bottom = (scroll_top + visible_height) as i32;

            // this is the height that is still on screen
            let clip_height = (msg_bottom.min(visible_bottom) - visible_top).max(0);
            let clip_start = (visible_top - msg_top).max(0);

            if clip_height > 0 {
                let rect = Rect {
                    x: chat_inner.x,
                    y: (chat_inner.y as i32 + visible_top - scroll_top as i32) as u16,
                    width: chat_inner.width,
                    height: clip_height as u16,
                };

                item.set_skip_lines(clip_start as u16);

                item.render(rect, buf);
            }

            let msg_top_hit_bottom = msg_top + MARGIN == visible_bottom;
            if msg_top_hit_bottom && item.is_selected && item.id > 0 {
                new_id -= 1;
                item.is_selected = false;
            }

            let msg_bottom_hit_top = msg_bottom == visible_top + MARGIN;
            if msg_bottom_hit_top && item.is_selected {
                if item.id + 1 < len {
                    new_id += 1;
                    item.is_selected = false;
                }
            }

            y += h;
        }

        self.selected_message_id = new_id;
        self.messages[new_id].is_selected = true;

        self.render_vertical_scrollbar(layout[2], buf, chat_inner.height as usize);
    }
}
