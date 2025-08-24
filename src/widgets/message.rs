use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{
        Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget,
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

    // pub fn increase_v_scroll_state(&mut self) {
    //     match self.v_scroll_width {
    //         Some(val) => self.v_scroll_state += if self.v_scroll_state >= val { 0 } else { 1 },
    //         None => self.v_scroll_state += 1,
    //     }
    // }
    //
    // pub fn decrease_v_scroll_state(&mut self) {
    //     if self.v_scroll_state == 0 {
    //         return;
    //     }
    //     self.v_scroll_state -= 1
    // }

    // pub fn set_scrollable_width(&mut self, value: usize) {
    //     self.h_scroll_width = Some(value);
    // }
    //
    // pub fn set_scrollable_height(&mut self, value: usize) {
    //     self.v_scroll_width = Some(value);
    // }
}

impl Widget for &mut Message {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.prepare(area);
        let txt = self.text.clone();

        let layout =
            Layout::horizontal([Constraint::Length(1), Constraint::Percentage(80)]).split(area);

        let height = get_height(&txt);
        let layout1 =
            Layout::vertical([Constraint::Length(1), Constraint::Length(height)]).split(layout[0]);
        let layout2 =
            Layout::vertical([Constraint::Length(1), Constraint::Length(height)]).split(layout[1]);

        Block::bordered().title("scr1").render(layout2[0], buf);
        Block::bordered().title("scr2").render(layout1[1], buf);
        Block::bordered().title("msg").render(layout2[1], buf);

        self.render_horizontal_scrollbar(layout2[0], buf);
        // self.render_vertical_scrollbar(layout1[1], buf);

        let block = Block::bordered().title("System");
        Paragraph::new(txt)
            .scroll((0, self.scroll_state as u16))
            .block(block)
            .render(layout2[1], buf);
    }
}

impl Message {
    pub fn render_horizontal_scrollbar(&mut self, area: Rect, buf: &mut Buffer) {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalTop)
            .end_symbol(Some("▶"))
            .thumb_symbol("■")
            .begin_symbol(Some("◀"))
            .track_symbol(Some("─"));

        let mut scrollbar_state = ScrollbarState::new(self.scroll_area).position(self.scroll_state);

        StatefulWidget::render(scrollbar, area, buf, &mut scrollbar_state);
    }

    // pub fn render_vertical_scrollbar(&mut self, area: Rect, buf: &mut Buffer) {
    //     let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
    //         .begin_symbol(Some("▲"))
    //         .end_symbol(Some("▼"))
    //         .track_symbol(Some("│"));
    //
    //     let content_height = get_height(&self.text) as usize;
    //     let viewport_height = area.height as usize;
    //     let scrollable_height = content_height.saturating_sub(viewport_height);
    //     self.set_scrollable_height(scrollable_height);
    //
    //     let mut scrollbar_state =
    //         ScrollbarState::new(scrollable_height).position(self.v_scroll_state);
    //
    //     StatefulWidget::render(scrollbar, area, buf, &mut scrollbar_state);
    // }
}
