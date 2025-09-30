
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
    widgets::{faustblock::FaustWidget}
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏━╸┏┓╻╺┳╸┏━╸┏━┓   ┏━╸┏━┓╻ ╻┏━┓╺┳╸
┣╸ ┃┗┫ ┃ ┣╸ ┣┳┛   ┣╸ ┣━┫┃ ┃┗━┓ ┃ 
┗━╸╹ ╹ ╹ ┗━╸╹┗╸   ╹  ╹ ╹┗━┛┗━┛ ╹ 
"};

#[derive(Default)]
enum Animation {
    #[default]
    None,
    CodeBlock(FaustWidget)
}

#[derive(Debug)]
enum Content<'a> {
    Paragraph(Paragraph<'a>),
    List(Vec<String>, ListState)
}

pub struct Faust<'a> {
    lhs: ContentArea<'a>, 
    rhs: Animation,
}

impl<'a> Default for Faust<'a> {
    fn default() -> Self {
        Faust {
            lhs: ContentArea::default()
                .add_title(TITLE)
                .add_paragraph(indoc! {
                    "• **Faust** (*Functional Audio Stream*) is a programming language \
                    specifically made for ***audio DSP and synthesis***. \
                    It has been created by Yann Orlarey, Dominique Fober & Stéphane Letz at GRAME in 2002."
                })
                .add_list(vec! [
                    "• *Functional* paradigm",
                    "• Declarative, math-like syntax",
                ])
                .add_paragraph(indoc! {
                    "• Compiles to **many targets** and **architectures**:",
                })
                .add_list(vec![
                    "• **Plugins**: VST, CLAP, AudioUnit",
                    "• **Software**: Max, Pd, SuperCollider, Csound",
                    "• **OS**: Linux, macOS, Windows, Android, iOS",
                    "• **Embedded**: Bela, Teensy, Daisy, ESP32, FPGA",
                    "• **Code**: C/C++, Rust, WASM, Java..."
                ])
                .add_paragraph(leafy! {
                    "Tutorials & documentation available at: ***https://faust.grame.fr***"
                })
                .add_paragraph(leafy! {
                    "Dedicated online IDE: ***https://faustide.grame.fr***"
                })
                ,
            rhs: Animation::CodeBlock(FaustWidget::default()),            
        }
    }
}

impl<'a> WidgetRef for Faust<'a> {
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
            Animation::CodeBlock(f) => {
                f.render_ref(lhr, buf);
            }
            _ => ()
        }
    }
}

impl<'a> Screen for Faust<'a> {
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