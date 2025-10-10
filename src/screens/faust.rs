
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{WidgetRef}
};

use indoc::indoc;

use ratatui_macros::{
    horizontal, 
};

use crate::{
    screens::{leafy, ContentArea, Screen, SideBySide}, 
    widgets::faustblock::FaustWidget
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏━╸┏┓╻╺┳╸┏━╸┏━┓   ┏━╸┏━┓╻ ╻┏━┓╺┳╸
┣╸ ┃┗┫ ┃ ┣╸ ┣┳┛   ┣╸ ┣━┫┃ ┃┗━┓ ┃ 
┗━╸╹ ╹ ╹ ┗━╸╹┗╸   ╹  ╹ ╹┗━┛┗━┛ ╹ 
"};

const VIRTUAL_ANALOG: &'static str = 
    include_str!("../../examples/virtualAnalog.dsp");

const HELLO440: &'static str = 
    include_str!("../../examples/hello440.dsp");

pub struct Faust<'a> {
    screen: SideBySide<'a>,
}

impl<'a> WidgetRef for Faust<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        self.screen.render_ref(area, buf);
    }
}

impl<'a> Screen for Faust<'a> {
    fn title(&self) -> &'static str {
        "Enter Faust!"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        self.screen.on_key_event(k);
    }
    fn on_tick(&mut self, t: usize) {
        self.screen.on_tick(t);
    }
}

impl<'a> Default for Faust<'a> {
    fn default() -> Self {
        Faust {
            screen: SideBySide::default()
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
                    "**-** Not recommended for multi-rate (FFT), but improving!"
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

