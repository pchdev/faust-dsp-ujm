use color_eyre::owo_colors::{OwoColorize, Rgb};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, 
    layout::{
        self, Constraint, Flex, Rect
    }, 
    style::{
        Color, Style, Stylize
    }, 
    widgets::{
        HighlightSpacing, 
        List, ListItem, ListState, 
        Paragraph, 
        StatefulWidget, 
        Widget, WidgetRef, 
        Wrap
    }
};

use ratatui_macros::vertical;

pub mod myself;
pub mod agenda;
pub mod signal;
pub mod sound;
pub mod splash;

pub(crate) enum Content<'a> {
    Paragraph(Paragraph<'a>),
    List(Vec<String>, ListState),
    Widget(Box<dyn WidgetRef>)
}

#[derive(Default)]
pub(crate) struct ContentArea<'a> {
      select: usize,
       title: Option<String>,
    contents: Vec<Content<'a>>
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
            ==20%, // title 
            ==75%  // contents
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
        for content in self.contents.iter() {
            match content {
                Content::Paragraph(ph) => {
                    let lc = ph.line_count(lv[2].width);
                    constraints.push(
                        Constraint::Length(lc.try_into().unwrap())
                    );
                }
                Content::List(list, ..) => {
                    constraints.push(
                        Constraint::Length(list.len() as u16)
                    )
                }
                Content::Widget(w) => {
                }
            }
        }
        // Build the layout:
        let lp = layout::Layout::vertical(constraints)
            .spacing(1)
            .split(lv[2])
        ;
        // Render everything:
        let mut i = 0;
        for (n, content) in self.contents.iter().enumerate() {
            match content {
                Content::Paragraph(ph) => {
                    if self.select == i {
                        // If paragraph is selected:
                        let p = ph.clone().black().on_gray();
                        p.render(lp[n], buf);
                    } else if self.select > i {
                        ph.render(lp[n], buf);
                    } else {
                        let p = ph.clone().gray().on_white();
                        p.render(lp[n], buf);
                    }               
                    i += 1;     
                }
                Content::List(svec, state) => {
                    let mut ivec = vec![];
                    let mut s = state.clone();
                    let select = self.select as isize - n as isize;
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
                        .highlight_symbol(">> ")
                        .highlight_style(Style::new().black().on_gray())
                        .highlight_spacing(HighlightSpacing::Always)
                    ;
                    StatefulWidget::render(l, lp[n], buf, &mut s);
                    i += svec.len();
                }
                Content::Widget(w) => {

                }
            }
        }
    }
}

pub(crate) trait Screen : WidgetRef {
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