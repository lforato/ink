use std::{io::Result, time::Duration};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
};

use crate::utils::get_height;

#[derive(Debug)]
pub struct TextArea {
    pub is_selected: bool,
    pub text: String,
    pub height: u16,
}

impl TextArea {
    pub fn new() -> Self {
        return Self {
            text: String::from(""),
            is_selected: true,
            height: 10,
        };
    }

    pub fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char(v) => self.text.push(v),
                    KeyCode::Backspace => {
                        self.text.pop();
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

impl Widget for &mut TextArea {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let height = get_height(&self.text);
        self.height = height + 6;
        let block = Block::bordered();
        let text = self.text.clone();
        Paragraph::new(text).block(block).render(area, buf);
    }
}
