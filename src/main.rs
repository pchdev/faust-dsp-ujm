mod screens;
mod widgets;

use std::{io, time::{Duration, Instant}};
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};

use ratatui::{
    layout::Flex, 
    style::{Style, Stylize}, 
    symbols::border, 
    widgets::{Block, Borders, Clear, HighlightSpacing, List, ListItem, ListState, StatefulWidget}, 
    DefaultTerminal, Frame
};
use ratatui_macros::{horizontal, line, vertical};
use tui_widgets::popup::{Popup, SizedWrapper};

use crate::screens::{
    agenda::Agenda, 
    digital::Digital, 
    faust::Faust, 
    myself::Myself, 
    signal::Signal, 
    sound::Sound, 
    splash::Splash, Screen
};

fn main() -> io::Result<()> {
    let mut term = ratatui::init();
    let res = App::new().run(&mut term);
    ratatui::restore();
    return res;
}

#[derive(Default)]
pub struct App {
      index: usize,
    screens: Vec<Box<dyn Screen>>,
      popup: bool,
      popup_state: ListState,
       exit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut app = App::default();
        app.screens = vec![
            Box::new(Splash::default()),
            Box::new(Myself::default()),
            Box::new(Agenda::default()),
            Box::new(Sound::default()),
            Box::new(Signal::default()),
            Box::new(Digital::default()),
            Box::new(Faust::default()),
        ];
        app
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        let tick_rate = Duration::from_millis(5);
        let mut last_tick = Instant::now();
        let mut tick_count = 0usize;
        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                match event::read()? {
                    Event::Key(k) if k.kind == KeyEventKind::Press => {
                        self.on_key_event(k)
                    }
                    _ => ()
                }
            }
            if last_tick.elapsed() >= tick_rate {
                let screen = &mut self.screens[self.index];
                screen.on_tick(tick_count);
                last_tick = Instant::now();
                tick_count += 1;
            }
        }

        Ok(())
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        if k.modifiers.contains(KeyModifiers::CONTROL) {
            match k.code {
                // Next screen:
                KeyCode::Char(' ') => {
                    if self.index < self.screens.len() -1 {
                       self.index += 1;
                    } 
                }
                // Previous screen:
                KeyCode::Char('h') => {
                    if self.index > 0 {
                       self.index -= 1;
                    }
                }
                // Quit app:
                KeyCode::Char('q') => {
                    self.exit();
                }
                // Otherwise, dispatch to current screen:
                _ => {
                    let screen = &mut self.screens[self.index];
                    screen.on_key_event(k);
                }
            }
        } else {
            if self.popup {
                match k.code {
                    KeyCode::Up => {
                        self.popup_state.select_previous();
                    }
                    KeyCode::Down => {
                        self.popup_state.select_next();
                    }
                    KeyCode::Enter => {
                        self.index = self.popup_state.selected().unwrap();
                        self.popup = false;
                    }
                    KeyCode::F(4) | KeyCode::Esc => {
                        self.popup = false;
                        self.popup_state.select(Some(self.index));
                    }
                    _ => ()
                }
            } else {
                match k.code {
                    KeyCode::F(4) => {
                        // Open popup:
                        self.popup = true;
                        self.popup_state.select(Some(self.index));
                    }
                    _ => {
                        let screen = &mut self.screens[self.index];
                        screen.on_key_event(k);
                    }
                }
            }

        }
    }
    fn animate(&self, frame: &mut Frame) {
        // Let's have a 'dissolve', and a fade-in/fade-out animation
    }

    fn draw(&self, frame: &mut Frame) {
        // The main frame block:
        let block = Block::bordered()
            .title(
                line![
                    " Master CCNT".bold(),"(3), ",
                    "DIGICREA".bold(), "(1), ",
                    "UJM".italic(), " - ",
                    "Saint-Etienne".italic(), ", ",
                    "Oct-Nov 2025 ".italic()]
                .centered()
            )
            .title_bottom(
                line![
                    " @pchdev - ",
                    "Emeraude".bold(), " - ",
                    "Inria".italic(), ", ",
                    "INSA-Lyon".italic(), ", ",
                    "CITI Lab ".italic()]
                .left_aligned()
            )
            .title_bottom(
                line![
                    "[esc] Quit ", 
                    "[bsp] Prev ", 
                    "[spc] Next "]
                .white().on_black()
                .right_aligned()
            )
            .border_set(border::ROUNDED)
            .black()
            .on_white()
        ;
        // Get the block's inner area:
        let inner = block.inner(frame.area());
        let screen = &self.screens[self.index];
        frame.render_widget(&block, frame.area());
        screen.render_ref(inner, frame.buffer_mut());
        if self.popup {
            let lv = vertical![==33%, ==33%, ==33%]
                .flex(Flex::Center)
                .split(frame.area())
            ;
            let lh = horizontal![==33%, ==33%, ==33%]
                .flex(Flex::Center)
                .split(lv[1])
            ;
            let items: Vec<ListItem> = self.screens.iter().enumerate()
                .map(|(n,i)| 
                    ListItem::new(format!("{}. {}", n+1, i.title()))
                )
                .collect()
            ;
            let l = List::new(items)
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
            let mut state = self.popup_state.clone();
            frame.render_widget(Clear, lh[1]);
            frame.render_widget(&block, lh[1]);
            l.render(lvb[1], frame.buffer_mut(), &mut state);
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
