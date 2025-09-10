
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
        calendar::{CalendarEventStore, Monthly}, Block, Paragraph, StatefulWidgetRef, Widget, WidgetRef
    }
};
use ratatui_macros::{horizontal, vertical};
use time::{Date, Month, OffsetDateTime};
use crate::screens::Screen;
use indoc::indoc;

const AGENDA: &'static str = indoc!{"
┏━┓┏━╸┏━╸┏┓╻╺┳┓┏━┓
┣━┫┃╺┓┣╸ ┃┗┫ ┃┃┣━┫
╹ ╹┗━┛┗━╸╹ ╹╺┻┛╹ ╹

"};

#[derive(Debug, Default)]
pub struct Agenda;

enum State {}

impl WidgetRef for Agenda {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let [lhl, lhr] = horizontal![==50%, ==50%]
            .flex(Flex::Center)
            .areas(area)
        ;
        let lhlv = vertical![==5%, ==20%,==75%]
            .flex(Flex::Center)
            .split(lhl)
        ;
        let subtitle = Paragraph::new(AGENDA)
            .centered()
        ;
        subtitle.render(lhlv[1], buf);
        let [lhlv2c] = horizontal![==100%]
            .flex(Flex::Center)
            .areas(lhlv[2])
        ;
        let date = OffsetDateTime::now_utc().date()
            .replace_month(Month::October)
            .unwrap()
        ;
        let mut dates = CalendarEventStore::default();
        dates.add(
            Date::from_calendar_date(2025, Month::October, 13).unwrap(), 
            Style::default().red().bold()
        );
        dates.add(
            Date::from_calendar_date(2025, Month::October, 20).unwrap(),
             Style::default()
        );
        let m = Monthly::new(date, dates)
            .block(Block::new())
            .show_month_header(
                Style::default().bold()
            )
            .show_weekdays_header(
                Style::default().italic()
            )
        ;
        m.render(lhlv2c, buf)
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