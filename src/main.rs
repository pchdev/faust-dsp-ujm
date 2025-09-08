mod screens;

use std::{io, ops::Index};
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
    agenda::Agenda, splash::{Splash}, toc::TableOfContents, Screen
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
        ];
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
//     const HEADER: &'static str = r"
//  MASTER CCNT(3), DIGICREA(1), Université Jean Monnet, Saint-Etienne, Oct-Nov 2025 
// ";

// const FOOTER: &'static str = r"
//  Pierre Cochard - Emeraude Team - Inria, INSA-Lyon, CITI Laboratory 
// ";
    fn draw(&self, frame: &mut Frame) {
        // The main frame block:
        let block = Block::bordered()
            .title(
                line![
                    " Master ", "CCNT".bold(),"(3), ",
                    "DIGICREA".bold(), "(1), ",
                    "Université Jean Monnet".italic(), ", ",
                    "Saint-Etienne".italic(), ", ",
                    "Oct-Nov 2025 ".italic()
                ]
                .centered()
            )
            .title_bottom(
                line![
                    " Pierre Cochard - ",
                    "Emeraude team".bold(), " - ",
                    "Inria".italic(), ", ",
                    "INSA-Lyon".italic(), ", ",
                    "CITI Laboratory ".italic()
                ]
                .centered()
            )
            .border_set(border::DOUBLE)
            .black()
            .on_white()
        ;
        // Get the block's inner area:
        let inner = block.inner(frame.area());
        // Add a vertical layout, leave a small space for navigating slides:
        let lv = vertical![==90%, ==10%]
            .flex(Flex::Center)
            .split(inner)
        ;
        let widget = &self.screens[self.index];
        frame.render_widget(&block, frame.area());
        widget.render_ref(lv[0], frame.buffer_mut());
        
        let lh = horizontal![==33%, ==33%, ==33%]
            .split(lv[1])
        ;
        let p = Text::raw("← Prev")
            .white().on_black()
            // .centered()
        ;

        let [lv2] = vertical![==p.height() as u16]
            .flex(Flex::Center)
            .areas(lh[1]);     
        let [lh2] = horizontal![==p.width() as u16]
            .flex(Flex::Center)
            .areas(lv2);
        // let text = Text::raw("← Prev").centered()
        p.render(lh2, frame.buffer_mut());
        // widget.render(inner, frame.buffer_mut());
    }
}
