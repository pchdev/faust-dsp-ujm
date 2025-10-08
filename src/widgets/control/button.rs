
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, 
    layout::{Flex, Rect}, 
    style::{Style, Stylize}, 
    symbols::border, 
    text::Text, 
    widgets::{
        Block, Clear, Widget, WidgetRef
    }
};
use ratatui_macros::vertical;

use crate::widgets::InteractiveWidget;

#[derive(Debug, Default)]
pub struct Button {
    state: bool,
    pub label: String
}

impl InteractiveWidget for Button {
    fn on_key_event(&mut self, k: KeyEvent) {
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
            .style(style)
        ;
        Clear::default().render(area, buf);
        let l = vertical![==40%, ==60%]
            .flex(Flex::Center)
            .split(block.inner(area))
        ;
        Text::from(self.label.clone())
            .style(Style::default().bold())
            .centered()
            .render(l[1], buf)
        ;
        block.render(area, buf);
    }
}