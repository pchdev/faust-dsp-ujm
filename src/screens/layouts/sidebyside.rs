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

// #[derive(Screen)]
// #[screen(layout = Layout::SideBySide)]
// #[screen(title = TITLE)]
// #[screen(description = "Sound (1/2)")]
// struct Soundtest {
//     // ----------------------------------------------------------------------------
//     /// Sound is a ***pressure wave*** that propagates
//     /// through a **medium** (*gas*, *liquid* or *solid*).
//     p0: (Paragraph, Ripple),
//     // ----------------------------------------------------------------------------
//     /// Propagation is caused by the **oscillation** (*vibration*) of
//     /// the medium's *particles*, around their ***equilibrium*** positions.
//     p1: (Paragraph, Particles),
//     // ----------------------------------------------------------------------------
//     /// Sound has the following properties:
//     p2: Paragraph,
//     // ----------------------------------------------------------------------------
//     /// • **Speed**: ~343 m/s in **air**
//     /// • **Amplitude**: in *Pascals* (***Pa***) or *Decibels* (***dB***)
//     /// • **Period**: time between two oscillations
//     /// • **Wavelength**: distance between two oscillations
//     /// • **Frequency**: cycles/sec., in *Hertz* (***Hz***, ***kHz***, ***MHz***)
//     /// • **Spectrum**, or *Timbre*
//     l0: List,
// }

macro_rules! side_by_side {
    // ------------------------------------------------------------------
    (
        name: $name:ident, 
        title: $title:expr, 
        description: $descr:literal
    // ------------------------------------------------------------------
    ) => {

        pub struct $name<'a> {
            layout: SideBySide<'a>
        }
        impl<'a> ratatui::widgets::WidgetRef for $name<'a> {
            fn render_ref(&self, 
                area: ratatui::prelude::Rect, 
                 buf: &mut ratatui::prelude::Buffer
            ) {
                self.layout.render_ref(area, buf);   
            }            
        }
        impl<'a> Screen for $name<'a> {
            fn title(&self) -> &'static str {
                $title
            }
            fn description(&self) -> &'static str {
                $descr
            }
            fn layout(&self) -> Option<&dyn Layout> {
                Some(&self.layout)
            }
            fn layout_mut(&mut self) -> Option<&mut dyn Layout> {
                Some(&mut self.layout)
            }
        }
    };
    // ------------------------------------------------------------------
    (
        name: $name:ident,
        title: $title:expr, 
        description: $descr:literal, 
        contents: [ 
            $(wparagraph: {$wph:expr, $w:expr}),*
        ]
    // ------------------------------------------------------------------
    ) => {        
        side_by_side!(name: $name, title: $title, description: $descr);

        impl<'a> Default for $name<'a> {
            fn default() -> Self {
                let mut layout = SideBySide::default().add_title($title);
                $(
                    layout = layout.add_paragraph(crate::leafy!{$wph});
                    layout = layout.add_widget($w);
                )*
                Self { layout }
            }
        }
    }
}

pub(crate) use side_by_side;

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