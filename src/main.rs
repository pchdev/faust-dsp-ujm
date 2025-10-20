mod screens;
mod widgets;
mod keymap;

use std::{
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    style::{Color, Stylize},
    symbols::border,
    widgets::{Block, WidgetRef},
    DefaultTerminal, Frame,
};
use ratatui_macros::line;
use tachyonfx::{fx, EffectManager, Interpolation};

use crate::screens::{
    agenda::Agenda,
    digital::{Digital, Digital2},
    faust::{basics::FaustBasics, basics2::FaustBasics2, intro::FaustIntro},
    myself::Myself,
    signal::{Signal, Signal2},
    sound::{Sound, Sound2},
    splash::Splash,
    Screen,
};
use crate::widgets::popup_menu::PopupMenu;
use crate::keymap::{map_key, Command};

fn main() -> io::Result<()> {
    let mut term = ratatui::init();
    let res = App::new().run(&mut term);
    ratatui::restore();
    res
}

#[derive(Default)]
pub struct App<'a> {
    index: usize,
    screens: Vec<Box<dyn Screen>>,
    menu: PopupMenu<'a>,
    fx: EffectManager<()>,
    exit: bool,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let mut app = App::default();
        app.screens = vec![
            Box::new(Splash::default()),
            Box::new(Myself::default()),
            Box::new(Agenda::default()),
            Box::new(Sound::default()),
            Box::new(Sound2::default()),
            Box::new(Signal::default()),
            Box::new(Signal2::default()),
            Box::new(Digital::default()),
            Box::new(Digital2::default()),
            Box::new(FaustIntro::default()),
            Box::new(FaustBasics::default()),
            Box::new(FaustBasics2::default()),
        ];
        app.menu.populate_from_string(app.screens.iter().map(|s| s.title().into()).collect());

        let fade = fx::fade_to(Color::Cyan, Color::White, (1000, Interpolation::SineIn));
        app.fx.add_effect(fade);
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
                    Event::Key(k) if k.kind == KeyEventKind::Press => self.on_key_event(k),
                    _ => {}
                }
            }

            if last_tick.elapsed() >= tick_rate {
                self.screens[self.index].on_tick(tick_count);
                last_tick = Instant::now();
                tick_count += 1;
            }
        }

        Ok(())
    }

    fn on_key_event(&mut self, k: KeyEvent) {
        if let Some(cmd) = map_key(k) {
            match cmd {
                Command::NextScreen => self.next_screen(),
                Command::PrevScreen => self.prev_screen(),
                Command::Quit => self.exit(),
                Command::OpenMenu => {
                    self.menu.open();
                    self.menu.select(Some(self.index));
                }
                Command::Passthrough => {
                    if self.menu.open {
                        if let Some(i) = self.menu.on_key_event(k) {
                            self.index = i;
                        }
                    } else {
                        self.screens[self.index].on_key_event(k);
                    }
                }
            }
        }
    }

    fn next_screen(&mut self) {
        if self.index < self.screens.len() - 1 {
            self.index += 1;
        }
    }

    fn prev_screen(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let block = Block::bordered()
            .title(
                line![
                    " Master CCNT".bold(),
                    "(3), ",
                    "DIGICREA".bold(),
                    "(1), ",
                    "UJM".italic(),
                    " - ",
                    "Saint-Etienne".italic(),
                    ", ",
                    "Oct-Nov 2025 ".italic()
                ]
                .centered(),
            )
            .title_bottom(
                line![
                    " @pchdev - ",
                    "Emeraude".bold(),
                    " - ",
                    "Inria".italic(),
                    ", ",
                    "INSA-Lyon".italic(),
                    ", ",
                    "CITI Lab ".italic()
                ]
                .left_aligned(),
            )
            .title_bottom(
                line![
                    "← Prev | → Next | Ctrl+W Quit | F4 Menu"
                ]
                .white()
                .on_black()
                .right_aligned(),
            )
            .border_set(border::ROUNDED)
            .black()
            .on_white();

        let inner = block.inner(frame.area());
        frame.render_widget(&block, frame.area());
        self.screens[self.index].render_ref(inner, frame.buffer_mut());
        self.menu.render_ref(frame.area(), frame.buffer_mut());
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}