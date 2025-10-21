use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer, 
    layout::{Flex, Rect}, 
    widgets::{ListState, Paragraph, WidgetRef, Wrap}
};
use ratatui_macros::horizontal;

use crate::screens::{layouts::content::{Content, ContentArea}, Screen};

#[derive(Default)]
pub struct PlainFull<'a> {
    contents: ContentArea<'a>,
}

impl<'a> PlainFull<'a> {
    pub fn add_title(mut self, title: &'static str) -> Self {
        self.contents.title = Some(String::from(title));
        return self;
    }
    pub fn add_paragraph(mut self, txt: &'static str) -> Self {
        self.contents.contents.push(
            Content::Paragraph(
                Paragraph::new(
                    tui_markdown::from_str(txt)
                ).wrap(Wrap {trim: true})
            )
        );
        return self;
    }
    pub fn add_list(mut self, list: Vec<&'static str>) -> Self {
        let mut items = vec![];
        for i in list {
            items.push(String::from(i));
        }
        self.contents.contents.push(
            Content::List(items, ListState::default())
        );
        return self;
    }    
}

impl<'a> Screen for PlainFull<'a> {
    fn title(&self) -> &'static str {
        "plain-full template"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        self.contents.on_key_event(k);   
    }
}

impl<'a> WidgetRef for PlainFull<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let lh = horizontal![==20%, ==60%, ==20%]
            .flex(Flex::Center)
            .split(area)
        ;
        self.contents.render_ref(lh[1], buf);
    }
}