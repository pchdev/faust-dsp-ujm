
use indoc::indoc;

use crate::{
    leafy,
    screens::{layouts::{sidebyside::SideBySide, Layout}, Screen}, 
    widgets::faustblock::FaustWidget
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
    layout: SideBySide<'a>,
}

impl<'a> Screen for FaustTime<'a> {
    fn title(&self) -> &'static str {
        TITLE
    }
    fn description(&self) -> &'static str {
        "Faust: time"
    }
    fn layout(&self) -> Option<&dyn Layout> {
        Some(&self.layout)
    }
    fn layout_mut(&mut self) -> Option<&mut dyn Layout> {
        Some(&mut self.layout)
    }
}

impl<'a> Default for FaustTime<'a> {
    fn default() -> Self {
        FaustTime {
            layout: SideBySide::default()
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

