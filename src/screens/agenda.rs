
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, layout::{
        Constraint, 
        Flex, 
        Layout, 
        Rect
    }, style::{Style, Stylize}, text::{
        Line, Text
    }, 
    widgets::{
        calendar::{CalendarEventStore, Monthly}, Block, StatefulWidgetRef, Widget, WidgetRef
    }
};
use time::OffsetDateTime;

use crate::screens::Screen;

#[derive(Debug, Default)]
pub struct Agenda;

enum State {}

impl StatefulWidgetRef for Agenda {
    type State = usize;
    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut usize) {
        let date = OffsetDateTime::now_utc().date();
        Monthly::new(date, CalendarEventStore::today(
                Style::new().red().bold())
            )
            .block(Block::new())
            .show_month_header(Style::new().bold())
            .show_weekdays_header(Style::new().italic())
            .render(area, buf)
        ;
    }
}

impl Screen for Agenda {
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Down => {

            }
            KeyCode::Enter => {
                
            } 
            _ => ()
        }
    }
}