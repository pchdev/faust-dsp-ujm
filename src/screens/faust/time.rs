use indoc::indoc;
use macros::Screen;

use crate::{
    screens::{
        faust::example, 
        layouts::{
            sidebyside::SideBySide, 
            plainfull::PlainFull,
            Layout, 
            LayoutEnum
        }, 
        Screen, ScreenParagraph
    }, 
    widgets::faustblock::FaustWidget
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
╺┳╸╻┏┳┓┏━╸
 ┃ ┃┃┃┃┣╸ 
 ╹ ╹╹ ╹┗━╸
"};

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Faust: time")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct FaustTime {
    // ------------------------------------------------------------------------
    /// Faust has one final composition operator: the ***recursive operator*** ('**~**') \
    /// which allows to ***connect two signals recursively*** (with ***a delay of one sample***).
    #[faust(example!("time/recursive.dsp"))]
    recursive: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// The recursive operator is typically used to implement **counters**, and/or ***to represent time***. \
    /// With it, we can count for instance *up to 44100 or 48000 samples* to ***represent one second of \
    /// audio time***.
    #[faust(example!("time/counter_samples.dsp"))]
    counter_samples: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// When implementing ***time-related functions***, it can prove very useful to \
    /// ***monitor values from the GUI***.
    #[faust(example!("time/monitoring_values.dsp"))]
    monitoring: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// We can for example use a counter to ***switch our synth's waveform automatically every second.***
    #[faust(example!("time/waveform_switch_counter.dsp"))]
    waveform: (ScreenParagraph, FaustWidget),
}
