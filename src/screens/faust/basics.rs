
use indoc::indoc;
use ratatui::{buffer::Buffer, layout::Rect, widgets::{WidgetRef}};

use crate::{
    screens::{
        faust::example,
        layouts::{
            sidebyside::{side_by_side, SideBySide},
            Layout
        }, 
        Screen
    },
    widgets::db::Decibels,  
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏┓ ┏━┓┏━┓╻┏━╸┏━┓
┣┻┓┣━┫┗━┓┃┃  ┗━┓
┗━┛╹ ╹┗━┛╹┗━╸┗━┛
"};

side_by_side!(
    name: FaustBasics, 
    title: TITLE,
    description: "Faust: basics",
    contents: [
        wparagraph: { 
            "Basic program: ***import*** and ***process*** statements.", 
            example!("basics/comments.dsp")
        },
        wparagraph: {
            "Use a ***Sinewave Oscillator*** from the library.",
            example!("basics/libraries.dsp")
        },
        wparagraph: {
            "In DSP, we represent an audio signal with ***floating-point*** (decimal) numbers \
            and *scale* its values ***between -1.0 and +1.0***.",
            example!("basics/amplitude.dsp")
        },
        wparagraph: {
            "***Mixing*** two signals can be done using the '**+**' operator.",
            example!("basics/mixing.dsp")
        },
        wparagraph: {
            "When multiplying two signals together, we can already make a simple \
            DSP effect: ***Amplitude Modulation*** (or ***Ring Modulation***).",
            example!("basics/ampmod.dsp")
        },
        wparagraph: {
            "Faust can process **multiple channels**, we can use the '**,**' symbol \
            (***Parallel Operator***) to put signals *in parallel*.",
            example!("basics/parallel.dsp")
        },
        wparagraph: {
            "We can declare **custom variables** (***functions***).",
            example!("basics/custom-functions.dsp")
        }
    ]
);  

side_by_side!(
    name: FaustBasics2, 
    title: TITLE,
    description: "Faust: basics (2)", 
    contents: [
        wparagraph: {
            "In Faust, *connecting DSP functions* can be done using the ***sequential operator*** '**:**'.",
            example!("basics2/sequential.dsp")
        },
        wparagraph: {
            "The ***split operator*** '**<:**' and ***cable operator*** can also be used to connect a \
            signal to *multiple targets*.",
            example!("basics2/split-cable.dsp")
        },
        wparagraph: {
            "On the other hand, the ***merge operator*** '**:>**' can be used to \
            **merge** (**mix**) **signals together**.",
            example!("basics2/merge-cable.dsp")
        },
        wparagraph: {
            "*Graphical User Interface* (***GUI***) elements can be added to ***control parameters***: \
            **sliders**, **buttons**, **switches**, **vu-meters**, *etc.*",
            example!("basics2/sliders.dsp")
        },
        wparagraph: {
            "They can be used to make a proper ***gain*** control in ***dB*** for example.",
            example!("basics2/gain.dsp")
        },
        wparagraph: {
            "***Decibels*** (***dB***) are frequently used in audio to represent ***volume***, but they \
            are sometimes a bit confusing to deal with.",
            Box::new(Decibels::default())
        }
    ]
);