use ratatui::{
    prelude::{Buffer, Rect, Widget},
    text::Text,
    widgets::Block,
};

use crate::view::{
    style,
    top_mid_bottom,
};

pub struct ScrollContent<'a> {
    pub text: Text<'a>,
    pub more_above: bool,
    pub more_below: bool,
}

pub struct ScrollArea<'a, F>
where
    F: FnOnce(usize) -> ScrollContent<'a>
{
    pub build: F
}

fn scroll_hint(more: bool) -> &'static str{
    if more {" ..."} else { "" }
}

impl<'a, F> Widget for ScrollArea<'a, F>
where
    F: FnOnce(usize) -> ScrollContent<'a>
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top_line, mid_area, bottom_line] = top_mid_bottom(area);
        let ScrollContent {
            text,
            more_above,
            more_below
        } = (self.build)(mid_area.height as usize);
        Block::new()
            .style(style::BG_DEFAULT)
            .render(mid_area, buf);
        text.render(mid_area, buf);
        Text::from(scroll_hint(more_above))
            .style(style::DEFAULT)
            .render(top_line, buf);
        Text::from(scroll_hint(more_below))
            .style(style::DEFAULT)
            .render(bottom_line, buf);
    }
}
