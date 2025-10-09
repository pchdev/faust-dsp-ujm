use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer, 
    layout::{
        self, Constraint, Flex, Rect, Size
    }, 
    style::{
        Style, Stylize
    }, 
    widgets::{
        Block, 
        BorderType, Borders, 
        HighlightSpacing, 
        List, ListItem, ListState, 
        Paragraph, 
        StatefulWidget, Widget, WidgetRef,
        Wrap
    }
};

use ratatui_macros::{horizontal, vertical};

pub mod myself;
pub mod agenda;
pub mod signal;
pub mod sound;
pub mod splash;
pub mod digital;
pub mod faust;


macro_rules! leafy {
    ($str:expr) => {
        concat!("**[↲]**  ", indoc!($str))
    };
}

pub(crate) use leafy;
use tui_widgets::scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

use crate::widgets::InteractiveWidget;

pub(crate) enum Content<'a> {
    Paragraph(Paragraph<'a>),
    List(Vec<String>, ListState),
}

#[derive(Default)]
pub(crate) struct ContentArea<'a> {
    select: usize,
    pub title: Option<String>,
    pub contents: Vec<Content<'a>>
}

impl<'a> ContentArea<'a> {
    pub(crate) fn add_title(mut self, title: &'static str) -> Self {
        self.title = Some(String::from(title));
        return self;
    }
    pub(crate) fn add_paragraph(mut self, txt: &'static str) -> Self {
        self.contents.push(
            Content::Paragraph(
                Paragraph::new(
                    tui_markdown::from_str(txt)
                ).wrap(Wrap {trim: true})
            )
        );
        return self;
    }
    pub(crate) fn add_list(mut self, list: Vec<&'static str>) -> Self {
        let mut items = vec![];
        for i in list {
            items.push(String::from(i));
        }
        self.contents.push(
            Content::List(items, ListState::default())
        );
        return self;
    }
    pub(crate) fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Down => {
                self.select += 1;
            }
            KeyCode::Up => {
                if self.select > 0 {
                   self.select -= 1;
                }
            }
            _ => ()
        }
    }
}

impl<'a> WidgetRef for ContentArea<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let lv = vertical![
             ==5%, // some spacing before title
            ==15%, // title 
            ==80%  // contents
        ]
            .flex(Flex::Center)
            .horizontal_margin(5)
            .split(area)
        ;
        // Render title:
        match &self.title {
            Some(t) => {
                Paragraph::new(t.as_str())
                    .centered()
                    .render(lv[1], buf)
                ;
            }
            None => ()
        }
        // Compute layout constraints:
        let mut constraints = vec![];
        let (mut w, mut h) = (lv[2].width, 0);
        for content in self.contents.iter() {
            match content {
                Content::Paragraph(ph) => {
                    let lc = ph.line_count(lv[2].width);
                    h += lc + 1;
                    constraints.push(
                        Constraint::Length(lc.try_into().unwrap())
                    );
                }
                Content::List(list, ..) => {
                    constraints.push(
                        Constraint::Length(list.len() as u16)
                    );
                    h += list.len() + 1;
                }
            }
        }
        // Build the layout:
        let lp = layout::Layout::vertical(constraints)
            .spacing(1)
            .split(lv[2])
        ;
        let mut scroll = ScrollView::new(Size::new(w, h as u16))
            .vertical_scrollbar_visibility(ScrollbarVisibility::Always)
        ;
        // Render everything:
        let mut i = 0;
        for (n, content) in self.contents.iter().enumerate() {
            match content {
                Content::Paragraph(ph) => {
                    if self.select == i {
                        // If paragraph is selected:
                        let p = ph.clone().black().on_gray();
                        scroll.render_widget(p, lp[n]);
                        // p.render(lp[n], buf);
                    } else if self.select > i {
                        scroll.render_widget(ph, lp[n]);
                        // ph.render(lp[n], buf);
                    } else {
                        let p = ph.clone().gray().on_white();
                        // p.render(lp[n], buf);
                        scroll.render_widget(p, lp[n]);
                    }               
                    i += 1;     
                }
                Content::List(svec, state) => {
                    let mut ivec = vec![];
                    let mut s = state.clone();
                    let select = self.select as isize - i as isize;
                    if select < 0 || select >= svec.len() as isize {
                        s.select(None);
                    } else {
                        s.select(Some(select as usize));
                    }
                    for str in svec {
                        ivec.push(ListItem::new(
                            tui_markdown::from_str(
                                str.as_str()
                            )
                        ));
                    }   
                    let style = if select < 0 {
                        Style::new().gray().on_white()
                    } else {
                        Style::new()
                    };
                    let l = List::new(ivec)
                        .style(style)
                        .highlight_symbol("⤷ ")
                        .highlight_style(Style::new().black().on_gray())
                        .highlight_spacing(HighlightSpacing::Always)
                    ;
                    // scroll.render_widget(l, lp[n]);
                    // StatefulWidget::render(l, lp[n], buf, &mut s);
                    i += svec.len();
                }
            }
        }
        let mut state = ScrollViewState::new();
        scroll.render(lv[2], buf, &mut state);
    }
}

#[derive(Default)]
enum Focus { #[default] Lhs, Rhs }

#[derive(Default)]
pub struct SideBySide<'a> {
    lhs: ContentArea<'a>,
    rhs: HashMap<usize, Box<dyn InteractiveWidget>>,
    sel: Option<usize>,
    focus: Focus,
}

impl<'a> SideBySide<'a> {
    pub fn add_title(mut self, title: &'static str) -> Self {
        self.lhs.title = Some(String::from(title));
        return self;
    }
    pub fn add_paragraph(mut self, txt: &'static str) -> Self {
        self.lhs.contents.push(
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
        self.lhs.contents.push(
            Content::List(items, ListState::default())
        );
        return self;
    }
    pub fn add_widget(mut self, index: usize, w: Box<dyn InteractiveWidget>) -> Self {
        self.rhs.insert(index, w);
        return self;
    }
}

impl<'a> Screen for SideBySide<'a> {
    fn title(&self) -> &'static str {
        "side-by-side template"
    }
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
                && k.modifiers.contains(KeyModifiers::SHIFT)
                && k.code == KeyCode::Left {
                    self.focus = Focus::Lhs;
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
    }
}

pub(crate) trait Screen : WidgetRef {
    fn title(&self) -> &'static str;
    fn on_key_event(&mut self, k: KeyEvent) {}
    fn on_tick(&mut self, t: usize) {}
}

// The ideal would be:
// 

// #[derive(Screen)]
// #[screen(layout = Layout::SideBySide)]

// struct MyScreen {
//     // ---------------------------
//     /// Get **markdown comments**
//     ph1: Paragraph<'_>,
//     // ---------------------------
//     /// - First item
//     /// - Second item
//     /// - etc.
//     ph2: List<'_>
//     // ---------------------------
// }