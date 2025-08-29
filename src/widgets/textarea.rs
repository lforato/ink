use tui_textarea::TextArea as TxtArea;

#[derive(Debug)]
pub struct TextArea<'a> {
    pub is_selected: bool,
    pub textarea: TxtArea<'a>,
}

impl<'a> Default for TextArea<'a> {
    fn default() -> Self {
        Self {
            is_selected: true,
            textarea: TxtArea::default(),
        }
    }
}
