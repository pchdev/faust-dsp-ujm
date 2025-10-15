
use std::{collections::HashMap};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer, 
    layout::{
        Flex, Rect
    }, 
    style::{Style, Stylize}, 
    widgets::{
        calendar::{
            CalendarEventStore, Monthly
        }, 
        Block, 
        BorderType, Borders, 
        HighlightSpacing, 
        List, ListItem, ListState, 
        Paragraph, 
        StatefulWidget, Widget, WidgetRef
    }
};
use ratatui_macros::{text, line, span, horizontal, vertical};
use indoc::indoc;
use time::{macros::format_description, Date, Month, Time, UtcDateTime};
use time::macros::time;
use time::ext::NumericalDuration;

use crate::screens::{Focus, Screen};

const TITLE: &'static str = indoc!{"
┏━┓┏━╸┏━╸┏┓╻╺┳┓┏━┓
┣━┫┃╺┓┣╸ ┃┗┫ ┃┃┣━┫
╹ ╹┗━┛┗━╸╹ ╹╺┻┛╹ ╹
__________________
"};


// Utility macros:

macro_rules! strikethrough {
    ($txt:expr) => {
        span!($txt).crossed_out()        
    };
}

macro_rules! item {
    ($e:expr) => {
        ListItem::new($e)
    };
}

macro_rules! md {
    ($e:expr) => {
        tui_markdown::from_str($e)
    };
}


#[derive(Debug)]
struct Events<'a> {
     start: Time,
       end: Time,
     state: ListState,
     items: Vec<ListItem<'a>>,
}

struct MonthSchedule<'a> {
    cursor: Date,
     store: CalendarEventStore,
    events: HashMap<Date, Events<'a>>,
     focus: bool,
}

impl<'a> MonthSchedule<'a> {
    fn new(m: Month) -> Self {
        MonthSchedule {
            cursor: Date::from_calendar_date(2025, m, 1).unwrap(),
             store: CalendarEventStore::default(),
            events: HashMap::default(),
             focus: false,
        }
    }
    fn add_date(mut self, 
             day: u8, 
           start: Time,
             end: Time, 
           items: Vec<ListItem<'a>>
    ) -> Self {
        let dt = self.cursor.replace_day(day).unwrap().with_time(start);
        self.store.add(dt.date(), Style::default().blue().bold());
        self.events.insert(dt.date(), Events {
            start,
            end,
            state: ListState::default(),
            items
        });
        return self;
    }

    fn get_events(&self, date: Date) -> Option<(&Date, &Events<'a>)> {
        return self.events.get_key_value(&date)
    }

    fn get_events_mut(&mut self, date: Date) -> Option<&mut Events<'a>> {
        return self.events.get_mut(&date)
    }
}

impl<'a> WidgetRef for MonthSchedule<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let mut store = self.store.clone();
        store.add(UtcDateTime::now().date(), Style::default().red().bold());
        if self.focus {
            store.add(self.cursor, Style::default().white().on_black());
        }        
        Monthly::new(self.cursor, store)
            .show_month_header(
                Style::default().bold()
            )
            .show_weekdays_header(
                Style::default().italic()
            )  
            .render_ref(area, buf)
        ; 
    }
}

pub struct Agenda<'a> {
    date: Date,
    msched: Vec<MonthSchedule<'a>>,
    focus: Focus,
}

impl<'a> Agenda<'a> {
    /// Update MonthSchedule cursor
    fn update(&mut self) {
        for ms in &mut self.msched {
            if ms.cursor.month() == self.date.month() {
               ms.cursor = self.date;
               ms.focus = true;
            } else {
                ms.focus = false;
            }
        }
    }
    fn get_events(&self) -> Option<(&Date, &Events<'a>)> {
        for ms in &self.msched {
            if self.date == ms.cursor {
                return ms.get_events(self.date)
            }
        }
        None
    }
    fn get_events_mut(&mut self) -> Option<&mut Events<'a>> {
        for ms in &mut self.msched {
            if self.date == ms.cursor {
                return ms.get_events_mut(self.date)
            }
        }
        None        
    }

    fn has_events(&self) -> bool {
        self.get_events().is_some()
    }
}

impl<'a> Screen for Agenda<'a> {
    fn title(&self) -> &'static str {
        "Agenda"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        match self.focus {
            Focus::Lhs => {
                match k.code {
                    KeyCode::Up => {
                        self.date -= 1.weeks();
                        self.update();
                    }
                    KeyCode::Down => {
                        self.date += 1.weeks();
                        self.update();
                    }
                    KeyCode::Left => {
                        self.date -= 1.days();
                        self.update();
                    }
                    KeyCode::Right => {
                        self.date += 1.days();
                        self.update();
                    }
                    KeyCode::Enter => {
                        if self.has_events() {
                           self.focus = Focus::Rhs;
                        }
                    }
                    _ => ()
                }
            }
            Focus::Rhs => {
                match k.code {
                    KeyCode::Up => {
                        match self.get_events_mut() {
                            Some(ev) => {
                                ev.state.select_previous();
                            }
                            None => ()
                        }
                    }
                    KeyCode::Down => {
                        match self.get_events_mut() {
                            Some(ev) => {
                                ev.state.select_next();
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


impl<'a> Default for Agenda<'a> {
    fn default() -> Self {
        Self {
            date: Date::from_calendar_date(2025, Month::October, 1).unwrap(),
            msched: vec![
                MonthSchedule::new(Month::October)
                    .add_date(13, time!(14:30), time!(18:30), vec![
                        item!(strikethrough!("• Sound")),
                        item!(strikethrough!("• Audio Signal")),
                        item!(strikethrough!("• From Analog to Digital")),
                        item!(strikethrough!("• Sampling")),
                        item!(strikethrough!("• Quantization")),
                        item!(strikethrough!("• Digital Audio Formats")),
                        item!(strikethrough!("• Digital Audio Processing and Synthesis")),
                        item!(strikethrough!("• The Faust programming language")), 
                        item!(strikethrough!("• Faust playground!")), 
                    ])
                    .add_date(20, time!(14:30), time!(18:30), vec![
                        item!(md!("• First steps with **Faust**!")),
                        item!(md!("• **IDE** and programming **tools**/**environment**")),
                        item!(md!("• **Library**, **documentation** and **examples**")),
                        item!(md!("• **Simple DSP effects** (*ringmod*, *delay*)")),
                        item!(md!("• **GUI** for **control** (*sliders*, *buttons*...)")),
                        item!(md!("• **Simple synthesis**: *oscillators* and *waveforms*"))
                    ]),
                MonthSchedule::new(Month::November)
                    .add_date(10, time!(14:30), time!(18:30), vec![
                        item!(md!("• **Playing/recording** from/to *buffers* & *sound files*")),
                        item!(md!("• **Filtering** & advanced effects")),
                        item!(md!("• Building a **simple synthesizer**")),
                        item!(md!("• **Personal projects**"))
                    ])
                    .add_date(17, time!(14:30), time!(18:30), vec![
                        item!(md!("• **Personal projects** + *on-demand* info"))
                    ])
                    .add_date(26, time!(14:30), time!(16:30), vec![
                        item!(md!("• **Personal projects** + *on-demand* info"))
                    ]
                )
            ],
            focus: Focus::Lhs,
        }
    }
}


impl<'a> WidgetRef for Agenda<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        // Divide the screen in two horizontally:
        let [lhl, lhr] = horizontal![==50%, ==50%]
            .flex(Flex::Center)
            .areas(area)
        ;
        // Add a border between the two panes:
        Block::bordered()
            .borders(Borders::LEFT)
            .border_type(BorderType::Plain)
            .render(lhr, buf)
        ;
        // Lhs pane vertical layout:
        let lhlv = vertical![==5%, ==20%, ==75%]
            .flex(Flex::Center)
            .split(lhl)
        ;
        // Add a title:
        Paragraph::new(TITLE)
            .centered()
            .render(lhlv[1], buf)
        ;
        // Oct/Nov months layout:
        let [l_oct, l_nov] = vertical![==50%, ==50%]
            .flex(Flex::Center)
            .vertical_margin(1)
            .horizontal_margin(14)
            .areas(lhlv[2])
        ;
        // Center the Monthly widgets horizontally:
        let l_oct_h = horizontal![==10%, ==80%, ==10%]
            .flex(Flex::Center)
            .split(l_oct)
        ;
        let l_nov_h = horizontal![==10%, ==80%, ==10%]
            .flex(Flex::Center)
            .split(l_nov)
        ;
        // Render Monthly widgets:            
        self.msched[0].render_ref(l_oct_h[1], buf);
        self.msched[1].render_ref(l_nov_h[1], buf);

        // Rhs pane vertical layout:
        let lhrv = vertical![==15%, ==15%, ==60%, ==10%]
            .flex(Flex::Center)
            .horizontal_margin(2)
            .split(lhr)
        ;
        if self.focus == Focus::Rhs {
            match self.get_events() {
                Some((dt, ev)) => {
                    let fmtd = dt.format(
                        format_description!["[weekday], [month repr:long] [day]th [year]"]
                    ).unwrap();
                    let fmth_start = ev.start.format(
                        format_description!["[hour repr:12]:[minute][period]"]
                    ).unwrap();
                    let fmth_end = ev.end.format(
                        format_description!["[hour repr:12]:[minute][period]"]
                    ).unwrap();
                    let dur = ev.end - ev.start;
                    let fmth = format!("{fmth_start} - {fmth_end} ({dur})");
                    // Render paragraph with date-time:
                    Paragraph::new(
                        text![
                            line!(fmtd).bold().underlined().centered(),
                            line!(""),
                            line!(fmth).centered().italic()
                        ]
                    ).render(lhrv[1], buf);
                    let mut state = ev.state.clone();
                    let events = List::new(ev.items.clone())
                        .highlight_symbol("> ")
                        .highlight_style(
                            Style::new().white().on_dark_gray().bold()
                        )
                        .highlight_spacing(HighlightSpacing::Always)
                    ;
                    StatefulWidget::render(events, lhrv[2], buf, &mut state);
                }
                None => ()
            }
        }
    }
}

