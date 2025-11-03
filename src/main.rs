mod screens;
mod widgets;

use std::{io, time::{Duration, Instant}};
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers
};

use ratatui::{
    style::{Color, Stylize}, 
    symbols::border, 
    widgets::{Block, WidgetRef}, 
    DefaultTerminal, Frame
};
use ratatui_macros::{line, };
use tachyonfx::{fx, EffectManager, Interpolation};

use crate::{screens::{
    Screen, agenda::Agenda, digital::{Digital, Digital2}, faust::{
        basics::{FaustBasics, FaustBasics2}, 
        functions::{FaustFunctions, FaustSignalFunctions}, 
        intro::FaustIntro, 
        synthesis::FaustSynthesis, 
        time::FaustTime
    }, 
    layouts::Layout, 
    myself::Myself, 
    signal::{Signal, Signal2}, 
    sound::{Sound, Sound2}, 
    splash::Splash
}, widgets::popup_menu::PopupMenu
};

fn main() -> io::Result<()> {
    let mut term = ratatui::init();
    let res = App::new().run(&mut term);
    ratatui::restore();
    return res;
}

#[derive(Default)]
pub struct App<'a> {
      index: usize,
    screens: Vec<(Box<dyn Screen>, Option<Box<dyn Layout>>)>,
       menu: PopupMenu<'a>,
         fx: EffectManager<()>,
       exit: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let mut app = App::default();
        app.screens = vec![
            Splash::build(),
            Myself::build(),
            Agenda::build(),            
            Sound::build(),
            Sound2::build(),
            Signal::build(),
            Signal2::build(),
            Digital::build(),
            Digital2::build(),
            FaustIntro::build(),
            FaustBasics::build(),
            FaustBasics2::build(),
            FaustFunctions::build(),
            FaustSignalFunctions::build(),            
            FaustSynthesis::build(),
            FaustTime::build(),
        ];
        // Populate menu popup:
        app.menu.populate_from_string(
            app.screens.iter().map(|s| s.0.description().into()).collect()
        );

        // TODO:
        let fade = fx::fade_to(Color::Cyan, Color::White, (1000, Interpolation::SineIn));
        app.fx.add_effect(fade);
        app
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        let tick_rate = Duration::from_millis(5);
        let mut last_tick = Instant::now();
        let mut tick_count = 0usize;
        while !self.exit {
            term.draw(|frame| {
                self.draw(frame);                   

            })?;
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
                screen.0.on_tick(&mut screen.1, tick_count);
                last_tick = Instant::now();
                tick_count += 1;
            }
        }

        Ok(())
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        if k.modifiers.contains(KeyModifiers::CONTROL) {
            match k.code {
                // Next screen (Ctrl + Esp):
                KeyCode::Char(' ') | KeyCode::Null => {
                    if self.index < self.screens.len() -1 {
                       self.index += 1;
                    } 
                }
                // Previous screen (Ctrl + Backspace):
                KeyCode::Backspace | KeyCode::Char('h') => {
                    if self.index > 0 {
                       self.index -= 1;
                    }
                }
                // Quit app (Ctrl + w):
                KeyCode::Char('w') => {
                    self.exit();
                }
                // Otherwise, dispatch to current screen:
                _ => {
                    let screen = &mut self.screens[self.index];
                    screen.0.on_key_event(&mut screen.1, k);
                }
            }
        } else { 
            // No modifier, pass to menu:
            if self.menu.open {
                match self.menu.on_key_event(k) {
                    Some(index) => {
                        self.index = index;
                    }
                    None => ()
                }
            } else {
                // Open popup, or pass to underlying screen:
                match k.code {
                    KeyCode::F(4) => {
                        // Open popup:
                        self.menu.open();
                        self.menu.select(Some(self.index));
                    }
                    _ => {
                        let screen = &mut self.screens[self.index];
                        screen.0.on_key_event(&mut screen.1, k);
                    }
                }
            }

        }
    }
    fn animate(&self, frame: &mut Frame) {
        todo!()
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
                    "Ctrl + (", 
                    "[w] Quit | ", 
                    "[bsp] Prev | ", 
                    "[spc] Next | ",
                    "[F4] Jump)"
                ]
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
        screen.0.render(&screen.1, inner, frame.buffer_mut());
        self.menu.render_ref(frame.area(), frame.buffer_mut());
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
