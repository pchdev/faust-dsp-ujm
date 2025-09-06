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
    widgets::{Block, Padding, Paragraph, Widget},
    DefaultTerminal, Frame
};

/// Got from: https://patorjk.com/software/taag/
/// Font is 'ANSI Shadow'
const FAUST: &'static str = r"
███████╗ █████╗ ██╗   ██╗███████╗████████╗
██╔════╝██╔══██╗██║   ██║██╔════╝╚══██╔══╝
█████╗  ███████║██║   ██║███████╗   ██║   
██╔══╝  ██╔══██║██║   ██║╚════██║   ██║   
██║     ██║  ██║╚██████╔╝███████║   ██║   
╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   
                                          
";

const HEADER: &'static str = r"
 MASTER CCNT(3), DIGICREA(1), Université Jean Monnet, Saint-Etienne, Oct-Nov 2025 
";

const FOOTER: &'static str = r"
 Pierre Cochard - Emeraude Team - Inria, INSA-Lyon, CITI Laboratory 
";

fn main() -> io::Result<()> {
    let mut term = ratatui::init();
    let res = App::default().run(&mut term);
    ratatui::restore();
    return res;
}

#[derive(Default)]
pub struct App {
      index: usize,
    screens: Vec<Box<dyn Widget>>,
       exit: bool,
}

#[derive(Debug, Default)]
pub struct ScreenTitle {
    dummy: u32
}

impl Widget for ScreenTitle {
    fn render(self, area: Rect, buf: &mut Buffer)
    where 
        Self: Sized 
    {
        let mut txt = Text::from(FAUST);
        txt.push_line("");
        txt.push_line(Line::from(
            "Digital Audio Processing and Synthesis")
            .bold()
        );
        let [lv] = Layout::vertical([
                Constraint::Length(txt.height() as u16),
            ])
            .flex(Flex::Center)
            .areas(area)
        ;
        let [lh] = Layout::horizontal([
                Constraint::Length(txt.width() as u16)
            ])
            .flex(Flex::Center)
            .areas(lv)
        ;
        txt.render(lh, buf);
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = App::default();
        app.screens.push(
            Box::new(ScreenTitle::default()),
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
        let widget = self.screens[self.index];
        frame.render_widget(&block, frame.area());
        widget.render(inner, frame.buffer_mut());
    }
}
