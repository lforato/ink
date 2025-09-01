use log::info;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};
use tui_textarea::TextArea as TxtArea;

#[derive(Debug)]
pub struct TextArea<'a> {
    pub is_selected: bool,
    pub area: TxtArea<'a>,
    pub text: String,
}

impl<'a> Default for TextArea<'a> {
    fn default() -> Self {
        Self {
            is_selected: true,
            area: TxtArea::default(),
            text: String::from(""),
        }
    }
}

impl<'a> Widget for &mut TextArea<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.text = self.area.lines().join("\n");

        let selected_style = if self.is_selected {
            Style::default().fg(Color::LightGreen)
        } else {
            Style::default().fg(Color::White)
        };

        let inner = Block::bordered().inner(area);
        Block::bordered().style(selected_style).render(area, buf);
        self.area.render(inner, buf);
    }
}
