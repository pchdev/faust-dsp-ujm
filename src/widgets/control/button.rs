
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, 
    layout::{Flex, Rect}, 
    style::{Style, Stylize}, 
    symbols::border, 
    text::Text, 
    widgets::{
        Block, 
        Widget, 
        WidgetRef
    }
};
use ratatui_macros::vertical;

#[derive(Debug, Default)]
pub struct Button {
    state: bool,
    pub label: String
}

impl Button {
    pub fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Enter => {
                self.state = !self.state;
            }
            _ => ()
        }
    }
}

impl WidgetRef for Button {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let style = if self.state {
            Style::default().white().on_black()
        } else {
            Style::default().black().on_white()
        };
        let block = Block::bordered()
            .border_set(border::ROUNDED)
            .style(Style::default().black().on_white())
        ;
        let l = vertical![==40%, ==60%]
            .flex(Flex::Center)
            .split(block.inner(area));
        let label = Text::from(self.label.clone())
            .style(Style::default().bold())
            .centered()
            .render(l[1], buf);
        ;
        block.render(area, buf);
    }
}