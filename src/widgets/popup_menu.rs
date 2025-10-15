
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, layout::{Flex, Rect}, style::{Style, Stylize}, symbols::border, widgets::{Block, Clear, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget, WidgetRef}
};
use ratatui_macros::{horizontal, vertical, line};

#[derive(Debug, Default)]
pub struct PopupMenu<'a> {
    pub open: bool,
    items: Vec<ListItem<'a>>,
    state: ListState,
}

impl<'a> PopupMenu<'a> {
    pub fn populate(&mut self, items: Vec<ListItem<'a>>) {
        self.items = items;
    }
    pub fn populate_from_string(&mut self, items: Vec<String>) {
        self.items = items.iter().enumerate()
            .map(|(n,i)| 
                ListItem::new(format!("{}. {}", n+1, i))
            )
            .collect();
    }
    pub fn open(&mut self) {
        self.open = true;
    }
    pub fn close(&mut self) {
        self.open = false;
    }
    pub fn on_key_event(&mut self, k: KeyEvent) -> Option<usize> {
        match k.code {
            KeyCode::Up => {
                self.state.select_previous();
                return None;
            }
            KeyCode::Down => {
                let i = self.state.selected().unwrap();
                if i < self.items.len()-1 {
                    self.state.select_next();
                }
                return None;
            }
            KeyCode::Enter => {
                let index = self.state.selected().unwrap();
                self.close();
                return Some(index);
            }
            KeyCode::F(4) | KeyCode::Esc => {
                self.close();
                return None;
            }
            _ => None
        }
    }
    pub fn select(&mut self, index: Option<usize>) {
        self.state.select(index);
    }
}

impl<'a> WidgetRef for PopupMenu<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        if !self.open {
            return;
        }
        let lv = vertical![==33%, ==33%, ==33%]
            .flex(Flex::Center)
            .split(area)
        ;
        let lh = horizontal![==33%, ==33%, ==33%]
            .flex(Flex::Center)
            .split(lv[1])
        ;
        let l = List::new(self.items.clone())
            .style(Style::new().black().on_white())
            .highlight_symbol("⤷ ")
            .highlight_style(Style::new().black().on_gray().bold())
            .highlight_spacing(HighlightSpacing::Always)
        ;
        let block = Block::bordered()
            .title(line![" jump to screen: "].centered().bold())
            .border_set(border::ROUNDED)
            .black().on_white()
        ;
        let lvb = vertical![==10%, ==80%, ==10%]
            .flex(Flex::Center)
            .split(block.inner(lh[1]))
        ; 
        let mut state = self.state.clone();
        Clear::default().render(lh[1], buf);
        block.render(lh[1], buf);
        StatefulWidget::render(l, lvb[1], buf, &mut state);
    }
}