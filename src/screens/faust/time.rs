
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{WidgetRef}
};

use indoc::indoc;

use crate::{
    screens::{leafy, Screen, SideBySide}, 
    widgets::{faustblock::FaustWidget}
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
╺┳╸╻┏┳┓┏━╸
 ┃ ┃┃┃┃┣╸ 
 ╹ ╹╹ ╹┗━╸
"};

macro_rules! example {
    ($path:literal) => {
        Box::new(
            FaustWidget::new(
                include_str!(
                    concat!("../../../examples/time/", $path)
                )
            )
        )        
    };
}

pub struct FaustTime<'a> {
    screen: SideBySide<'a>,
}

impl<'a> WidgetRef for FaustTime<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        self.screen.render_ref(area, buf);
    }
}

impl<'a> Screen for FaustTime<'a> {
    fn title(&self) -> &'static str {
        "Faust: time"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        self.screen.on_key_event(k);
    }
    fn on_tick(&mut self, t: usize) {
        self.screen.on_tick(t);
    }
}

impl<'a> Default for FaustTime<'a> {
    fn default() -> Self {
        FaustTime {
            screen: SideBySide::default()
                .add_title(TITLE)
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "Faust has one final composition operator: the ***recursive operator*** ('**~**') \
                     which allows to ***connect two signals recursively*** (with ***a delay of one sample***)."
                })
                .add_widget(0, example!("recursive.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "The recursive operator is typically used to implement **counters**, and/or ***to represent time***. \
                    With it, we can count for instance *up to 44100 or 48000 samples* to ***represent one second of \
                    audio time***."
                })
                .add_widget(1, example!("counter_samples.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "When implementing ***time-related functions***, it can prove very useful to \
                    ***monitor values from the GUI***."
                })
                .add_widget(2, example!("monitoring_values.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "We can for example use a counter to ***switch our synth's waveform automatically every second.***"
                })
                .add_widget(3, example!("waveform_switch_counter.dsp"))
                // ---------------------------------------------------------------------------------------
                ,
        }
    }
}

