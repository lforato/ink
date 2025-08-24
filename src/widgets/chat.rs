use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, List, ListDirection, ListItem, Widget},
};

use crate::widgets::message::Message;

#[derive(Debug)]
pub struct Chat {
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
        if self.scroll_area > self.scroll_state {
            return;
        }
        self.scroll_state += 1;
   }
}

impl Widget for &mut Chat {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([Constraint::Percentage(5), Constraint::Percentage(90)])
            .horizontal_margin(10)
            .split(area)[1];

        let items: Vec<ListItem> = self.messages.iter().map(|m| m.as_list_item()).collect();

        let list = List::new(self.messages)
            .block(Block::bordered().title("List"))
            .repeat_highlight_symbol(true)
            .direction(ListDirection::BottomToTop);

        Block::bordered().render(layout, buf);
    }
}
