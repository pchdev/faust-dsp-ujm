
use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    buffer::Buffer, 
    layout::{
        Flex, 
    }, 
    prelude::Rect, 
    widgets::{
        Block, 
        BorderType, 
        Borders, 
        ListState, 
        Paragraph, 
        Widget, WidgetRef, 
    }
};

use indoc::indoc;

use ratatui_macros::{
    horizontal, 
};

use crate::{
    screens::{ContentArea, Screen, leafy}, 
    widgets::{
        particles::Particles, 
        ripple::Ripple, 
        waveform::Waveform
    }
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
╻ ╻┏━╸╻  ╻  ┏━┓╻
┣━┫┣╸ ┃  ┃  ┃ ┃╹
╹ ╹┗━╸┗━╸┗━╸┗━┛╹
"};

#[derive(Debug, Default)]
enum Animation {
    #[default]
    None,
}

#[derive(Debug)]
enum Content<'a> {
    Paragraph(Paragraph<'a>),
    List(Vec<String>, ListState)
}

pub struct Myself<'a> {
    lhs: ContentArea<'a>, 
    rhs: Animation,
}

impl<'a> Default for Myself<'a> {
    fn default() -> Self {
        Myself {
            lhs: ContentArea::default()
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
            rhs: Animation::default(),            
        }
    }
}

impl<'a> WidgetRef for Myself<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        let [lhl, lhr] = horizontal![==50%, ==50%]
            .flex(Flex::Center)
            .areas(area)
        ;
        Block::bordered()
            .borders(Borders::LEFT)
            .border_type(BorderType::Plain)
            .render(lhr, buf)
        ;
        self.lhs.render_ref(lhl, buf);
        match &self.rhs {
            _ => ()
        }
    }
}

impl<'a> Screen for Myself<'a> {
    fn on_key_event(&mut self, k: KeyEvent) {
        match k.code {
            KeyCode::Down | KeyCode::Up => {
                self.lhs.on_key_event(k);
            }
            KeyCode::Enter => {
                match self.lhs.select {
                    _ => ()
                }
            }
            _ => ()
        }
    }
    fn on_tick(&mut self, t: usize) {
        match &mut self.rhs {
            _ => ()
        }
    }
}