use ratatui::{layout::{Constraint, Layout}, widgets::{Block, Paragraph, Widget}};

#[derive(Debug)]
pub struct Message {
    pub text: String,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::from("Hello world"),
        }
    }
}

impl Widget for &Message {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let width = Layout::horizontal([Constraint::Length(100)]).split(area)[0];

        let layout = Layout::vertical([Constraint::Length(10)]).split(width)[0];

        let block = Block::bordered().title("System");

        Paragraph::new(self.text.clone())
            .block(block)
            .render(layout, buf);
    }
}
