use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer, layout::{
        Constraint, Flex, Layout, Rect
    }, style::{palette::tailwind::SLATE, Style, Stylize}, symbols, text::{
        Line, Text
    }, widgets::{Block, Borders, HighlightSpacing, List, ListItem, Paragraph, Widget, WidgetRef}
};
use ratatui_macros::{horizontal, vertical};

use crate::screens::Screen;


const TOC: &'static str = r"


╺┳╸┏━┓┏┓ ╻  ┏━╸   ┏━┓┏━╸   ┏━╸┏━┓┏┓╻╺┳╸┏━╸┏┓╻╺┳╸┏━┓
 ┃ ┣━┫┣┻┓┃  ┣╸    ┃ ┃┣╸    ┃  ┃ ┃┃┗┫ ┃ ┣╸ ┃┗┫ ┃ ┗━┓
 ╹ ╹ ╹┗━┛┗━╸┗━╸   ┗━┛╹     ┗━╸┗━┛╹ ╹ ╹ ┗━╸╹ ╹ ╹ ┗━┛
";

const TOC_PH: &'static str = r"

1. AGENDA
2. SOUND
3. SIGNAL
4. etc. 
";

#[derive(Debug, Default)]
pub struct TableOfContents;

impl WidgetRef for TableOfContents {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let lv = vertical![==25%, ==75%]
            .flex(Flex::Center)
            .split(area)
        ;
        let lh = horizontal![==25%, ==75%]
            .flex(Flex::Center)
            .split(lv[1])
        ;
        let mut subtitle = Paragraph::new(TOC)
            .centered()
        ;
        let mut items = vec![
            ListItem::new("Agenda"),
            ListItem::new("Sound"),
            ListItem::new("Signal"),
        ];
        let mut list = List::new(items)
            .block(
                Block::new()
                    .title(Line::raw("Test").centered())
                    .borders(Borders::TOP)
                    .border_set(symbols::border::EMPTY)
                    .border_style(Style::new())
                    // .bg(SLATE.c950)
            )
            // .highlight_style()
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always)
        ;
        let mut txt = Paragraph::new(TOC_PH)
            // .centered()
            .bold()
        ;
        subtitle.render(lv[0], buf);
        list.render(lh[1], buf);
    }
}

impl Screen for TableOfContents {
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            _ => ()
        }       
    }
}