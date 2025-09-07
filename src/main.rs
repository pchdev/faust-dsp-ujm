mod screens;

use std::{io, ops::Index};
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind
};

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph, Table, Widget, WidgetRef},
    DefaultTerminal, Frame
};

use crate::screens::{title::Title, toc::TableOfContents, Screen};


const HEADER: &'static str = r"
 MASTER CCNT(3), DIGICREA(1), Université Jean Monnet, Saint-Etienne, Oct-Nov 2025 
";

const FOOTER: &'static str = r"
 Pierre Cochard - Emeraude Team - Inria, INSA-Lyon, CITI Laboratory 
";

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
        app.screens.push(
            Box::new(Title::default()),
        );
        app.screens.push(
            Box::new(TableOfContents::default())
        );
        app
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            term.draw(|frame| self.draw(frame))?;
            self.on_event()?;
        }
        Ok(())
    }
    fn on_event(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(k) if k.kind == KeyEventKind::Press => {
                self.on_key_event(k)
            }
            _ => ()
        }
        Ok(())
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Right | 
            KeyCode::Down  |
            KeyCode::Enter => {
                // Next item:
                if self.index < self.screens.len() -1 {
                    self.index += 1;
                } 
            }
            KeyCode::Left |
            KeyCode::Up |
            KeyCode::Backspace => {
                if self.index > 0 {
                    self.index -= 1;
                }
            }
            KeyCode::Char('q') |
            KeyCode::Esc => {
                self.exit();
            }
            _ => ()
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
    fn draw(&self, frame: &mut Frame) {
        // The main frame block:
        let block = Block::bordered()
            .title(
                Line::from(HEADER).centered()
            )
            .title_bottom(
                Line::from(FOOTER).centered()
            )
            .border_set(border::DOUBLE)
            .black()
            .on_white()
        ;
        // Get the inner area of the block, 
        // and put a vertical layout into it:
        let inner = block.inner(frame.area());
        let widget = &self.screens[self.index];
        frame.render_widget(&block, frame.area());
        widget.render_ref(inner, frame.buffer_mut());
        // widget.render(inner, frame.buffer_mut());
    }
}
