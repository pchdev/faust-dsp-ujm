
use indoc::indoc;

use crate::screens::{
        faust::example, layouts::{
            sidebyside::{
                side_by_side, 
                SideBySide
            }, 
            Layout
        }, Screen
    };

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
╺┳╸╻┏┳┓┏━╸
 ┃ ┃┃┃┃┣╸ 
 ╹ ╹╹ ╹┗━╸
"};


side_by_side!(
    name: FaustTime,
    title: TITLE,
    description: "Faust: time",
    contents: [
        wparagraph: {
            "Faust has one final composition operator: the ***recursive operator*** ('**~**') \
            which allows to ***connect two signals recursively*** (with ***a delay of one sample***).",
            example!("time/recursive.dsp")
        },
        wparagraph: {
            "The recursive operator is typically used to implement **counters**, and/or ***to represent time***. \
            With it, we can count for instance *up to 44100 or 48000 samples* to ***represent one second of \
            audio time***.",
            example!("time/counter_samples.dsp")
        },
        wparagraph: {
            "When implementing ***time-related functions***, it can prove very useful to \
            ***monitor values from the GUI***.",
            example!("time/monitoring_values.dsp")
        },
        wparagraph: {
            "We can for example use a counter to ***switch our synth's waveform automatically every second.***",
            example!("time/waveform_switch_counter.dsp")
        }
    ]
);

