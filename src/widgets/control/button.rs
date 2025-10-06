use color_eyre::owo_colors::OwoColorize;

use ratatui::{
    buffer::Buffer, 
    layout::Rect, 
    symbols::border, 
    text::Text, 
    widgets::{
        Block, 
        Widget, 
        WidgetRef
    }
};

struct Button {
    state: bool,
    label: String
}

impl WidgetRef for Button {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .border_set(border::ROUNDED)
        ;
        let label = Text::from(self.label.clone())
            .centered()
            .render(block.inner(area), buf);
        ;
        block.render(area, buf);
    }
}