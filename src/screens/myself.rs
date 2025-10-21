
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::WidgetRef
};

use indoc::indoc;

use crate::leafy;
use crate::screens::{layouts::plainfull::PlainFull, Screen};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
╻ ╻┏━╸╻  ╻  ┏━┓╻
┣━┫┣╸ ┃  ┃  ┃ ┃╹
╹ ╹┗━╸┗━╸┗━╸┗━┛╹
"};


pub struct Myself<'a> {
    screen: PlainFull<'a>,
}

impl<'a> WidgetRef for Myself<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        self.screen.render_ref(area, buf);
    }
}

impl<'a> Screen for Myself<'a> {
    fn title(&self) -> &'static str {
        "Hello!"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        self.screen.on_key_event(k);
    }
    fn on_tick(&mut self, t: usize) {
        self.screen.on_tick(t);
    }
}

impl<'a> Default for Myself<'a> {
    fn default() -> Self {
        Myself {
            screen: PlainFull::default()
                .add_title(TITLE)
                .add_paragraph(indoc! {
                    "• My name is **Pierre**, nice to meet you all!"
                }) 
                .add_paragraph(indoc! {
                    "• First time teaching... but **in your shoes**, *12 years ago* :)"
                }) 
                .add_paragraph(leafy! {
                    "Now in *Inria/INSA* team ***Emeraude***, working as a *research engineer* in *Lyon*, alongside:"
                }) 
                .add_list(vec![
                    "• **Romain Michon**",
                    "• **Stéphane Letz**",
                    "• Yann Orlarey",
                ])
                .add_paragraph(indoc! {
                    "• What I'm working on: "
                })
                .add_list(vec![
                    "• ***Syfala*** (*Faust-to-FPGA toolchain*)",
                    "• ***FloPoCo*** (*generator of arithmetic cores*) for FPGA",
                    "• ***Faust***"
                ])
                .add_paragraph(indoc! {
                    "• **P.S**: I'm not really an expert in DSP :("
                })
                ,   
        }
    }
}