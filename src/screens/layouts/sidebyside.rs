use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer, 
    layout::{Flex, Rect}, 
    style::{Style, Stylize}, 
    widgets::{
        Block, 
        BorderType, Borders, 
        ListState, 
        Paragraph, 
        Widget, WidgetRef, 
        Wrap
    }
};
use ratatui_macros::horizontal;

use crate::{
    screens::{
        layouts::{
            content::{
                Content, 
                ContentArea
            }, 
            Layout
        },
    }, 
    widgets::InteractiveWidget
};

#[derive(Default, PartialEq)]
pub enum Focus { #[default] Lhs, Rhs }

#[derive(Default)]
pub struct SideBySide<'a> {
    lhs: ContentArea<'a>,
    rhs: HashMap<usize, Box<dyn InteractiveWidget>>,
    sel: Option<usize>,
    focus: Focus,
    fullscreen: bool,
}

impl<'a> Layout for SideBySide<'a> {
    fn add_title(mut self, title: &'static str) -> Self {
        self.lhs.title = Some(String::from(title));
        return self;
    }
    fn add_paragraph(mut self, txt: &'static str) -> Self {
        println!("Adding paragraph: {txt}");
        self.lhs.contents.push(
            Content::Paragraph(
                Paragraph::new(
                    tui_markdown::from_str(txt)
                ).wrap(Wrap {trim: true})
            )
        );
        return self;
    }
    fn add_list(mut self, list: Vec<&'static str>) -> Self {
        println!("Adding list: {:?}", list);
        let mut items = vec![];
        for i in list {
            items.push(String::from(i));
        }
        self.lhs.contents.push(
            Content::List(items, ListState::default())
        );
        return self;
    }
    fn add_widget(mut self, w: Box<dyn InteractiveWidget>) -> Self {
        self.rhs.insert(self.lhs.contents.len()-1, w);
        return self;
    }
}

impl<'a> InteractiveWidget for SideBySide<'a> {
    fn on_key_event(&mut self, k: KeyEvent) {
        match self.focus {
            Focus::Lhs => {
                match k.code {
                    KeyCode::Right => {
                        if k.modifiers.contains(KeyModifiers::CONTROL)
                        && k.modifiers.contains(KeyModifiers::SHIFT) {
                            self.focus = Focus::Rhs;
                        }
                    }
                    KeyCode::Enter => {
                        self.focus = Focus::Rhs;
                        if self.rhs.contains_key(&self.lhs.select) {
                            self.sel = Some(self.lhs.select);
                        }   
                    }
                    _ => {
                        self.lhs.on_key_event(k);
                    }
                }
            }
            Focus::Rhs => {
                if k.modifiers.contains(KeyModifiers::CONTROL)
                && k.modifiers.contains(KeyModifiers::SHIFT) {
                    match k.code {
                        KeyCode::Left => {
                            self.focus = Focus::Lhs;
                            self.fullscreen = false;
                        }
                        KeyCode::Up => {
                            self.fullscreen = true;
                        }
                        KeyCode::Down => {
                            self.fullscreen = false;
                        }
                        _ => ()
                    }
                } else {
                    match &self.sel {
                        Some(x) => {
                            match self.rhs.get_mut(x) {
                                Some(w) => {
                                    w.on_key_event(k);
                                }
                                None => ()
                            }
                        }
                        _ => ()
                    }                    
                }
            }
        }
    }
    fn on_tick(&mut self, t: usize) {
        match &self.sel {
            Some(x) => {
                match self.rhs.get_mut(x) {
                    Some(w) => {
                        w.on_tick(t);
                    }
                    _ => ()
                }
            }
            _ => ()
        }
    }
}

impl<'a> WidgetRef for SideBySide<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        // Divide screen 50/50 horizontally: 
        if !self.fullscreen {
            let [lhl, lhr] = horizontal![==50%, ==50%]
                .flex(Flex::Center)
                .areas(area)
            ;
            // Add vertical separator:
            Block::bordered()
                .borders(Borders::LEFT)
                .border_type(BorderType::Plain)
                .render(lhr, buf)
            ;
            match self.focus {
                Focus::Rhs => {
                    Block::bordered()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double)
                        .style(Style::default().dark_gray())
                        .render(lhr, buf);
                }
                _ => ()
            }
            self.lhs.render_ref(lhl, buf);
            match &self.sel {
                Some(x) => {
                    match self.rhs.get(x) {
                        Some(w) => {
                            w.render_ref(lhr, buf);
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        } else {
            match self.focus {
                Focus::Rhs => {
                    Block::bordered()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double)
                        .style(Style::default().dark_gray())
                        .render(area, buf);
                }
                _ => ()
            }
            match &self.sel {
                Some(x) => {
                    match self.rhs.get(x) {
                        Some(w) => {
                            w.render_ref(area, buf);
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        }
    }
}