
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{WidgetRef}
};

use indoc::indoc;

use crate::{
    leafy,
    screens::{layouts::{sidebyside::SideBySide, Layout}, Screen}, 
    widgets::faustblock::FaustWidget
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏━╸┏┓╻╺┳╸┏━╸┏━┓   ┏━╸┏━┓╻ ╻┏━┓╺┳╸
┣╸ ┃┗┫ ┃ ┣╸ ┣┳┛   ┣╸ ┣━┫┃ ┃┗━┓ ┃ 
┗━╸╹ ╹ ╹ ┗━╸╹┗╸   ╹  ╹ ╹┗━┛┗━┛ ╹ 
"};

const HELLO440: &'static str = 
    include_str!("../../../examples/basics/hello440.dsp");

pub struct FaustIntro<'a> {
    layout: SideBySide<'a>,
}

impl<'a> Screen for FaustIntro<'a> {
    fn title(&self) -> &'static str {
        TITLE
    }
    fn description(&self) -> &'static str {
        "Enter Faust!"
    }
    fn layout(&self) -> Option<&dyn Layout> {
        Some(&self.layout)
    }
    fn layout_mut(&mut self) -> Option<&mut dyn Layout> {
        Some(&mut self.layout)
    }
}

impl<'a> Default for FaustIntro<'a> {
    fn default() -> Self {
        FaustIntro {
            layout: SideBySide::default()
                .add_title(TITLE)
                .add_paragraph(leafy! {
                    "**Faust** (*Functional Audio Stream*) is a programming language \
                    specifically made for ***audio DSP and synthesis***. \
                    It was created by **Yann Orlarey**, **Dominique Fober** & **Stéphane Letz** at **GRAME** in 2002."
                })
                .add_list(vec! [
                    "**+** *Functional* paradigm",
                    "**+** Declarative, math-like syntax",
                    "**+** Produces optimized code for many architectures",
                    "**-** Not recommended (yet) for multi-rate (FFT)"
                ])
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
                .add_widget(0, Box::new(FaustWidget::new(HELLO440)))
                ,
        }
    }
}

