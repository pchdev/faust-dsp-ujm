mod screens;
mod widgets;

use std::{io, ops::Index, time::{Duration, Instant}};
use color_eyre::owo_colors::OwoColorize;
use crossterm::{event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind
}};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{block::Position, Block, Padding, Paragraph, Table, Widget, WidgetRef},
    DefaultTerminal, Frame
};
use ratatui_macros::{line, horizontal, vertical};

use crate::screens::{
    agenda::Agenda, sound::Sound, splash::Splash, toc::TableOfContents, Screen
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
       exit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut app = App::default();
        app.screens = vec![
            Box::new(Splash::default()),
            Box::new(TableOfContents::default()),
            Box::new(Agenda::default()),
            Box::new(Sound::default())
        ];
        app
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        let tick_rate = Duration::from_millis(16);
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
        match k.code {
            // Next screen:
            KeyCode::Char(' ') => {
                if self.index < self.screens.len() -1 {
                   self.index += 1;
                } 
            }
            // Previous screen:
            KeyCode::Backspace => {
                if self.index > 0 {
                   self.index -= 1;
                }
            }
            // Quit app:
            KeyCode::Esc => {
                self.exit();
            }
            // Otherwise, dispatch to current screen:
            _ => {
                let screen = &mut self.screens[self.index];
                screen.on_key_event(k);
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
    
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}
