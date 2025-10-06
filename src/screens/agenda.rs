
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    buffer::Buffer, 
    layout::{
        Flex, Rect
    }, 
    style::{Style, Stylize}, 
    widgets::{
        calendar::{
            CalendarEventStore, 
            Monthly
        }, 
        Block, 
        BorderType, Borders, 
        HighlightSpacing, 
        List, ListItem, ListState, 
        Paragraph, 
        StatefulWidget, Widget, WidgetRef
    }
};
use ratatui_macros::{text, line, horizontal, vertical};
use time::{Date, Month, OffsetDateTime};
use time::ext::NumericalDuration;
use indoc::indoc;

use crate::screens::Screen;

const AGENDA: &'static str = indoc!{"
┏━┓┏━╸┏━╸┏┓╻╺┳┓┏━┓
┣━┫┃╺┓┣╸ ┃┗┫ ┃┃┣━┫
╹ ╹┗━┛┗━╸╹ ╹╺┻┛╹ ╹
__________________
"};

#[derive(Debug)]
pub struct Agenda {
    date: Date,
    state: ListState,
    selection: bool,
}

impl Default for Agenda {
    fn default() -> Self {
        Self {
            date: OffsetDateTime::now_local()
                .unwrap()
                .date()
                .replace_day(1).unwrap()
                .replace_month(Month::October)
                .unwrap(),
            state: ListState::default(),
            selection: false
        }
    }
}

impl WidgetRef for Agenda {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let [lhl, lhr] = horizontal![==50%, ==50%]
            .flex(Flex::Center)
            .areas(area)
        ;
        // Add a border between the two panes
        let lhrb = Block::bordered()
            .borders(Borders::LEFT)
            .border_type(BorderType::Plain)
            .render(lhr, buf)
        ;
        let lhlv = vertical![==10%, ==15%, ==75%]
            .flex(Flex::Center)
            .split(lhl)
        ;
        let subtitle = Paragraph::new(AGENDA)
            .centered()
        ;
        subtitle.render(lhlv[1], buf);
        let [m_oct, m_nov] = vertical![==40%, ==60%]
            .flex(Flex::Center)
            .vertical_margin(1)
            .horizontal_margin(14)
            .areas(lhlv[2])
        ;
        let m_oct_h = horizontal![==25%, ==50%, ==25%]
            .flex(Flex::Center)
            .split(m_oct)
        ;
        let m_nov_h = horizontal![==25%, ==50%, ==25%]
            .flex(Flex::Center)
            .split(m_nov)
        ;
        let mut oct_dates = CalendarEventStore::default();
        let mut nov_dates = CalendarEventStore::default();
        let mut date = self.date.clone();
        oct_dates.add(
            Date::from_calendar_date(2025, Month::October, 13).unwrap(), 
            Style::default().red().bold()
        );
        oct_dates.add(
            Date::from_calendar_date(2025, Month::October, 20).unwrap(),
            Style::default().blue().bold()
        );
        if date.month() == Month::October {
            oct_dates.add(
                date,
                Style::default().white().on_dark_gray()
            );
        } else {
            date = date.replace_day(1).unwrap();
            date = date.replace_month(Month::October).unwrap();
        }
        Monthly::new(date, oct_dates)
            .show_month_header(
                Style::default().bold()
            )
            .show_weekdays_header(
                Style::default().italic()
            )
            .render(m_oct_h[1], buf)
        ;
        // November
        date = self.date.clone();
        nov_dates.add(
            Date::from_calendar_date(2025, Month::November, 10).unwrap(),
            Style::default().blue().bold()
        );
        nov_dates.add(
            Date::from_calendar_date(2025, Month::November, 17).unwrap(),
            Style::default().blue().bold()
        );
        nov_dates.add(
            Date::from_calendar_date(2025, Month::November, 26).unwrap(),
            Style::default().blue().bold()
        );        
        if date.month() == Month::November {
            nov_dates.add(
                date,
                Style::default().white().on_dark_gray()
            );
        } else {
            date = date.replace_day(1).unwrap();
            date = date.replace_month(Month::November).unwrap();
        }
        Monthly::new(date, nov_dates)
            .show_month_header(
                Style::default().bold()
            )
            .show_weekdays_header(
                Style::default().italic()
            )
            .render(m_nov_h[1], buf)
        ;
        let lhrv = vertical![==15%, ==15%, ==60%, ==10%]
            .flex(Flex::Center)
            .horizontal_margin(10)
            .split(lhr)
        ;
        if self.selection {
            match self.date.to_string().as_str() {
                "2025-10-13" => {
                    let p = Paragraph::new(
                        text![
                            line!["Monday October 13th 2025"]
                                .bold()
                                .underlined()
                                .centered(),
                            line![""],
                            line!["2:30pm to 6:30pm (4:00)"]
                                .centered()
                                .italic()
                        ]
                    );
                    p.render(lhrv[1], buf);

                    let mut state = self.state.clone();
                    let items = vec![
                        ListItem::new("• Sound"),
                        ListItem::new("• Audio Signal"),
                        ListItem::new("• From Analog to Digital"),
                        ListItem::new("• Sampling"),
                        ListItem::new("• Quantization"),
                        ListItem::new("• Digital Audio Formats"),
                        ListItem::new("• Digital Audio Processing and Synthesis"),
                        ListItem::new("• The Faust programming language"),            
                    ];
                    let list = List::new(items)
                        .highlight_symbol("> ")
                        .highlight_style(Style::new()
                            .white()
                            .on_dark_gray()
                            .bold()
                        )
                        .highlight_spacing(HighlightSpacing::Always)
                    ;
                    StatefulWidget::render(list, lhrv[2], buf, &mut state);
                }
                "2025-10-20" => {
                    let p = Paragraph::new(
                        text![
                            line!["Monday October 20th 2025"]
                                .bold()
                                .underlined()
                                .centered(),
                            line![""],
                            line!["2:30pm to 6:30pm (4:00)"]
                                .centered()
                                .italic()
                        ]
                    );
                    p.render(lhrv[1], buf);

                    let mut state = self.state.clone();
                    let items = vec![
                        ListItem::new(tui_markdown::from_str(
                            "• First steps with **Faust**!"
                        )),      
                        ListItem::new(tui_markdown::from_str(
                            "• **IDE** and programming **tools**/**environment**"
                        )),      
                        ListItem::new(tui_markdown::from_str(
                            "• **Library**, **documentation** and **examples**"
                        )),      
                        ListItem::new(tui_markdown::from_str(
                            "• **Simple DSP effects**, from scratch (*ringmod*, *delay*...)"
                        )),
                        ListItem::new(tui_markdown::from_str(
                            "• **Adding GUI elements** for **control** (*sliders*, *buttons*...)"
                        )),
                        ListItem::new(tui_markdown::from_str(
                            "• **Simple synthesis**: *oscillators* and *waveforms*"
                        )),
                    ];
                    let list = List::new(items)
                        .highlight_symbol("> ")
                        .highlight_style(Style::new()
                            .white()
                            .on_dark_gray()
                            .bold()
                        )
                        .highlight_spacing(HighlightSpacing::Always)
                    ;
                    StatefulWidget::render(list, lhrv[2], buf, &mut state);
                }
                "2025-11-10" => {
                    let p = Paragraph::new(
                        text![
                            line!["Monday November 10th 2025"]
                                .bold()
                                .underlined()
                                .centered(),
                            line![""],
                            line!["2:30pm to 6:30pm (4:00)"]
                                .centered()
                                .italic()
                        ]
                    );
                    p.render(lhrv[1], buf);
                    let mut state = self.state.clone();
                    let items = vec![
                        ListItem::new(tui_markdown::from_str(
                            "• **Playing/recording** from/to *buffers* & *sound files*"
                        )),
                        ListItem::new(tui_markdown::from_str(
                            "• **Filtering** & advanced effects"
                        )),
                        ListItem::new(tui_markdown::from_str(
                            "• Building a **simple synthesizer**"
                        )),
                        ListItem::new(tui_markdown::from_str(
                            "• **Personal projects**"
                        )),
                    ];
                    let list = List::new(items)
                        .highlight_symbol("> ")
                        .highlight_style(Style::new()
                            .white()
                            .on_dark_gray()
                            .bold()
                        )
                        .highlight_spacing(HighlightSpacing::Always)
                    ;
                    StatefulWidget::render(list, lhrv[2], buf, &mut state);
                }
                "2025-11-17" => {
                    let p = Paragraph::new(
                        text![
                            line!["Monday November 17th 2025"]
                                .bold()
                                .underlined()
                                .centered(),
                            line![""],
                            line!["2:30pm to 6:30pm (4:00)"]
                                .centered()
                                .italic()
                        ]
                    );
                    p.render(lhrv[1], buf);
                    let mut state = self.state.clone();
                    let items = vec![
                        ListItem::new(tui_markdown::from_str(
                            "• **Personal projects** + *on-demand* info"
                        )),
                    ];
                    let list = List::new(items)
                        .highlight_symbol("> ")
                        .highlight_style(Style::new()
                            .white()
                            .on_dark_gray()
                            .bold()
                        )
                        .highlight_spacing(HighlightSpacing::Always)
                    ;
                    StatefulWidget::render(list, lhrv[2], buf, &mut state);
                }
                "2025-11-26" => {
                    let p = Paragraph::new(
                        text![
                            line!["Monday November 17th 2025"]
                                .bold()
                                .underlined()
                                .centered(),
                            line![""],
                            line!["2:30pm to 6:30pm (4:00)"]
                                .centered()
                                .italic()
                        ]
                    );
                    p.render(lhrv[1], buf);
                    let mut state = self.state.clone();
                    let items = vec![
                        ListItem::new(tui_markdown::from_str(
                            "• **Personal projects**: let's hear them!"
                        )),
                    ];
                    let list = List::new(items)
                        .highlight_symbol("> ")
                        .highlight_style(Style::new()
                            .white()
                            .on_dark_gray()
                            .bold()
                        )
                        .highlight_spacing(HighlightSpacing::Always)
                    ;
                    StatefulWidget::render(list, lhrv[2], buf, &mut state);
                }
                _ => ()
            }
        }
    }
}

impl Screen for Agenda {
    fn title(&self) -> &'static str {
        "Agenda"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Up => {
                if self.selection {
                    self.state.select_previous();                    
                } else {
                    self.date -= 1.weeks();
                }
            }
            KeyCode::Down => {
                if self.selection {
                    self.state.select_next();                  
                } else {
                    self.date += 1.weeks();                      
                }
            }
            KeyCode::Right => {
                if self.selection {
                    // do nothing
                } else {
                    self.date += 1.days();
                }
            }
            KeyCode::Left => {
                if self.selection {                    
                    if k.modifiers.contains(KeyModifiers::CONTROL) {
                        self.selection = false;
                        self.state.select(None);
                    }
                } else {
                    self.date -= 1.days();
                }
            }
            KeyCode::Enter => {
                self.selection = true;
                self.state.select_first();
            } 
            _ => ()
        }
    }
}