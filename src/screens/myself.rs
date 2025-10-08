
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{WidgetRef}
};

use indoc::indoc;


use crate::{
    screens::{leafy, Screen, SideBySide}, 
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
╻ ╻┏━╸╻  ╻  ┏━┓╻
┣━┫┣╸ ┃  ┃  ┃ ┃╹
╹ ╹┗━╸┗━╸┗━╸┗━┛╹
"};

pub struct Myself<'a> {
    screen: SideBySide<'a>,
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
            screen: SideBySide::default()
                .add_title(TITLE)
                .add_paragraph(indoc! {
                    "• My name is **Pierre**, nice to meet you!"
                }) 
                .add_paragraph(indoc! {
                    "• **In your shoes**, *12 years ago* :-)"
                }) 
                .add_paragraph(indoc! {
                    "• Got an internship at ***SCRIME-LaBRI*** in Bordeaux! \
                    Worked for 3 years there after that, as a \
                    *Computer Music Designer* (*RIM*). \
                    Then worked as a \"*freelancer*\", on my own sound installation \
                    and other projects."
                }) 
                .add_paragraph(indoc! {
                    "• Then, ***COVID*** happened T_T. But got into a (*secret*) \
                    project with ***GRAME*** (*Max2FaustTranslator*)."
                }) 
                .add_paragraph(leafy! {
                    "• Now in *Inria/INSA* team ***Emeraude***, working as a *research engineer* in *Lyon*, alongside:"
                }) 
                .add_list(vec![
                    "• Tanguy Risset (Big Boss)",
                    "• Florent de Dinechin (My current Boss)",
                    "• **Romain Michon**",
                    "• Yann Orlarey",
                    "• **Stéphane Letz**",
                    "• and many more..."
                ])
                .add_paragraph(indoc! {
                    "• Working on projects like ***Syfala*** (*Faust-to-FPGA toolchain*), \
                    ***FloPoCo*** (*generator of arithmetic cores*) and of course ***Faust***."
                })
                .add_paragraph(indoc! {
                    "• Not really a musician anymore, not really expert in **DSP** either (sorry)... \
                    I like **code** (and *pixels*), and helping researchers."
                })
                ,   
        }
    }
}