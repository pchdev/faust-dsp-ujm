// use crossterm::event::{KeyCode, KeyEvent};
// use ratatui::{
//     buffer::Buffer, layout::{
//         Constraint, Flex, Layout, Rect
//     }, style::{palette::tailwind::SLATE, Style, Stylize}, symbols, text::{
//         Line, Text
//     }, widgets::{
//         Block, Borders, 
//         HighlightSpacing, 
//         List, ListItem, ListState, 
//         Paragraph, 
//         StatefulWidget, StatefulWidgetRef, 
//         Table, Widget, WidgetRef
//     }
// };
// use ratatui_macros::{horizontal, vertical};

// use crate::screens::Screen;

// const TOC: &'static str = r"


// ╺┳╸┏━┓┏┓ ╻  ┏━╸   ┏━┓┏━╸   ┏━╸┏━┓┏┓╻╺┳╸┏━╸┏┓╻╺┳╸┏━┓
//  ┃ ┣━┫┣┻┓┃  ┣╸    ┃ ┃┣╸    ┃  ┃ ┃┃┗┫ ┃ ┣╸ ┃┗┫ ┃ ┗━┓
//  ╹ ╹ ╹┗━┛┗━╸┗━╸   ┗━┛╹     ┗━╸┗━┛╹ ╹ ╹ ┗━╸╹ ╹ ╹ ┗━┛
//  __________________________________________________
// ";


// #[derive(Debug, Default)]
// pub struct TableOfContents {
//     state: ListState
// }

// impl WidgetRef for TableOfContents {
//     fn render_ref(&self, area: Rect, buf: &mut Buffer) {
//         let mut state = self.state.clone();
//         let lv = vertical![==30%, ==70%]
//             .flex(Flex::Center)
//             .split(area)
//         ;
//         let lh = horizontal![==33%, ==33%, ==33%]
//             .flex(Flex::Center)
//             .split(lv[1])
//         ;
//         let subtitle = Paragraph::new(TOC)
//             .centered()
//         ;
//         let items = vec![
//             ListItem::new("1. Agenda Overview"),
//             ListItem::new("2. Sound"),
//             ListItem::new("3. Audio Signal"),
//             ListItem::new("4. From Analog to Digital"),
//             ListItem::new("5. Sampling"),
//             ListItem::new("6. Quantization"),
//             ListItem::new("7. Digital Audio Formats"),
//             ListItem::new("8. Digital Audio Processing and Synthesis"),
//             ListItem::new("9. Faust"),            
//         ];
//         let list = List::new(items)
//             // .highlight_style()
//             .highlight_symbol("> ")
//             .highlight_style(Style::new()
//                 .white()
//                 .on_dark_gray()
//                 .bold()
//             )
//             .highlight_spacing(HighlightSpacing::Always)
//         ;
//         subtitle.render(lv[0], buf);
//         StatefulWidget::render(list, lh[1], buf, &mut state);
//     }
// }

// impl Screen for TableOfContents {
//     fn on_key_event(&mut self, k: KeyEvent) {
//         match k.code {
//             KeyCode::Down | KeyCode::Right  => {
//                 self.state.select_next();
//             }
//             KeyCode::Up | KeyCode::Left => {
//                 self.state.select_previous();

//             }
//             _ => ()
//         }       
//     }
// }