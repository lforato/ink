use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};
use uuid::Uuid;

#[derive(Debug)]
pub struct TextArea<'a> {
    pub id: Uuid,
    pub is_selected: bool,
    pub area: tui_textarea::TextArea<'a>,
    pub x: u16,
    pub y: u16,
    pub height: u16,
    pub width: u16,
}

impl<'a> Default for TextArea<'a> {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            is_selected: true,
            area: tui_textarea::TextArea::default(),
            x: 0,
            y: 0,
            height: 0,
            width: 0,
        }
    }
}

impl<'a> TextArea<'a> {
    pub fn clear(&mut self) -> String {
        let txt = self.area.lines().join("\n").trim().to_string();
        self.area = tui_textarea::TextArea::default();
        self.area.move_cursor(tui_textarea::CursorMove::Top);
        txt
    }

    pub fn is_within(&self, x: u16, y: u16) -> bool {
        let within_x = x >= self.x && x < self.x + self.width;
        let within_y = y >= self.y && y < self.y + self.height;
        within_x && within_y
    }
}

impl<'a> Widget for &mut TextArea<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let selected_style = if self.is_selected {
            Style::default().fg(Color::LightGreen)
        } else {
            Style::default().fg(Color::White)
        };

        self.x = area.x;
        self.y = area.y;
        self.height = area.height;
        self.width = area.width;

        self.area.set_block(Block::bordered().style(selected_style));
        self.area.render(area, buf);
    }
}
