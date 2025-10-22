use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{buffer::Buffer, layout::{self, Constraint, Flex, Rect}, style::{Style, Stylize}, widgets::{HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget, Widget, WidgetRef, Wrap}};
use ratatui_macros::vertical;

pub enum Content<'a> {
    Paragraph(Paragraph<'a>),
    List(Vec<String>, ListState),
}

#[derive(Default)]
pub struct ContentArea<'a> {
    pub select: usize,
    pub title: Option<String>,
    pub contents: Vec<Content<'a>>
}

impl<'a> ContentArea<'a> {
    pub fn on_key_event(&mut self, k: KeyEvent) {
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
            ==17%, // title 
            ==78%  // contents
        ]
            .flex(Flex::Center)
            .horizontal_margin(2)
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
                    StatefulWidget::render(l, lp[n], buf, &mut s);
                    i += svec.len();
                }
            }
        }
    }
}