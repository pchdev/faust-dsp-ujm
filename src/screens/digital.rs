
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{
        WidgetRef, 
    }
};

use indoc::indoc;

use crate::{
    screens::{leafy, Screen, SideBySide}, 
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
╺┳┓╻┏━╸╻╺┳╸┏━┓╻     ┏━┓╻┏━╸┏┓╻┏━┓╻  
 ┃┃┃┃╺┓┃ ┃ ┣━┫┃     ┗━┓┃┃╺┓┃┗┫┣━┫┃  
╺┻┛╹┗━┛╹ ╹ ╹ ╹┗━╸   ┗━┛╹┗━┛╹ ╹╹ ╹┗━╸
"};

pub struct Digital<'a> {
    screen: SideBySide<'a>
}

impl<'a> WidgetRef for Digital<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        self.screen.render_ref(area, buf);
    }
}

impl<'a> Screen for Digital<'a> {
    fn title(&self) -> &'static str {
        "Digital Audio Signal (1/2)"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        self.screen.on_key_event(k);
    }
    fn on_tick(&mut self, t: usize) {
        self.screen.on_tick(t);
    }
}

impl<'a> Default for Digital<'a> {
    fn default() -> Self {
        Digital {
            screen: SideBySide::default()
                .add_title(TITLE)
                .add_paragraph(indoc! {
                    "• To ***digitize*** a continuous signal implies *discretizing* it. \
                    This is made possible by an *Analog-to-Digital Conversion* (**ADC**) process, \
                    which implies two key elements: ***sampling*** and ***quantization.***
                    "
                })
                .add_paragraph(leafy! {
                    "***Sampling*** means taking a sample of a signal at a certain frequency/rate (***sample rate***).
                    "
                })
                .add_list(vec![
                    "• **Audio CD**: *44.1 kHz*",
                    "• **Pro Audio**: *48/96 kHz*",
                    "• **MP3**: *320/256/128/96/64* **kbps**"
                ])
                .add_paragraph(leafy!(
                    "• Because of ***aliasing***, the *sampling rate* must be **at least two times superior** to \
                    the **highest frequency** we want to represent (*Nyquist-Shannon*)."
                )
            )
        }
    }
}

pub struct Digital2<'a> {
    screen: SideBySide<'a>
}

impl<'a> WidgetRef for Digital2<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        self.screen.render_ref(area, buf);
    }
}

impl<'a> Screen for Digital2<'a> {
    fn title(&self) -> &'static str {
        "Digital Audio Signal (2/2)"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        self.screen.on_key_event(k);
    }
    fn on_tick(&mut self, t: usize) {
        self.screen.on_tick(t);
    }
}

impl<'a> Default for Digital2<'a> {
    fn default() -> Self {
        Digital2 {
            screen: SideBySide::default()
                .add_title(TITLE)
                .add_paragraph(leafy! {
                    "• Once we take a sample of a signal at a given time, we need to determine the ***scale of its value***, \
                    this is called ***quantization***. Increasing the scale implies reducing the ***quantization noise*** \
                    (*quality vs. storage tradeoff*).
                    "
                })
                .add_list(vec![
                    "• **Audio CD**: 16-bits (65,536 values, 98 dB SNR)",
                    "• **Pro Audio**: 24-bits (~16,7 mil., 146 dB SNR)",
                    "• ***DSP***: 32/64-bits float (~4,3 bil., 194 dB SNR)"
                ])
                .add_paragraph(indoc! {
                    "• For **DSP**, it is easier to make computations in ***floating-point*** (decimal), \
                    and *normalize* the signal between *-1.0* and *1.0*."
                })
                .add_paragraph(
                    "• Finally, sending a digital signal to audio speakers involves the inverse process of an **ADC**: \
                    *Digital-to-Analog Conversion* (**DAC**).
                    "
                )
        }
    }
}

