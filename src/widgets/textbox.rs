use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Default, Debug)]
pub struct TextBox {
    pub text: String,
}

impl Widget for &TextBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.text.clone())
            .block(Block::bordered().title("Paragraph"))
            .style(Style::new().white().on_black())
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}
