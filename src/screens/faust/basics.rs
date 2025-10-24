
use indoc::indoc;
use macros::Screen;

use crate::{
    screens::{
        faust::{self, example},
        layouts::{
            plainfull::PlainFull, sidebyside::SideBySide, Layout, LayoutEnum
        }, 
        Screen, ScreenParagraph
    },
    widgets::{db::Decibels, faustblock::FaustWidget},  
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏┓ ┏━┓┏━┓╻┏━╸┏━┓
┣┻┓┣━┫┗━┓┃┃  ┗━┓
┗━┛╹ ╹┗━┛╹┗━╸┗━┛
"};

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Faust: basics")]
#[screen(layout = LayoutEnum::SideBySide)]
struct FaustBasics {
    // ------------------------------------------------------------------------
    /// Basic program: ***import*** and ***process*** statements.
    #[faust(example!("basics/comments.dsp"))]
    import_process: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------    
    /// Use a ***Sinewave Oscillator*** from the library.
    #[faust(example!("basics/libraries.dsp"))]
    libraries: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------    
    /// In DSP, we represent an audio signal with ***floating-point*** 
    /// (decimal) numbers and *scale* its values ***between -1.0 and +1.0***.
    #[faust(example!("basics/amplitude.dsp"))]
    amplitude: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------    
    /// ***Mixing*** two signals can be done using the '**+**' operator.
    #[faust(example!("basics/mixing.dsp"))]
    mixing: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------    
    /// When multiplying two signals together, we can already make a simple
    /// DSP effect: ***Amplitude Modulation*** (or ***Ring Modulation***).
    #[faust(example!("basics/ampmod.dsp"))]
    ampmod: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------    
    /// Faust can process **multiple channels**, we can use the '**,**' symbol 
    /// (***Parallel Operator***) to put signals *in parallel*.
    #[faust(example!("basics/parallel.dsp"))]
    parallel: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------    
    /// We can declare **custom variables** (***functions***).
    #[faust(example!("basics/custom-functions.dsp"))]
    custom_fn: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------    
}

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Faust: basics (2)")]
#[screen(layout = LayoutEnum::SideBySide)]
struct FaustBasics2 {
    // ------------------------------------------------------------------------
    /// In Faust, *connecting DSP functions* can be done using the 
    /// ***sequential operator*** '**:**'.
    #[faust(example!("basics2/sequential.dsp"))]
    sequential: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------    
    /// The ***split operator*** '**<:**' and ***cable operator*** 
    /// can also be used to connect a signal to *multiple targets*.
    #[faust(example!("basics2/split-cable.dsp"))]
    split: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// On the other hand, the ***merge operator*** '**:>**' can be used to
    /// **merge** (**mix**) **signals together**.
    #[faust(example!("basics2/merge-cable.dsp"))]
    merge: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// *Graphical User Interface* (***GUI***) elements can be added to 
    /// ***control parameters***: **sliders**, **buttons**, **switches**,
    /// **vu-meters**, *etc.*",
    #[faust(example!("basics2/sliders.dsp"))]
    sliders: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// They can be used to make a proper ***gain*** control 
    /// in ***dB*** for example.
    #[faust(example!("basics2/gain.dsp"))]
    gain: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// ***Decibels*** (***dB***) are frequently used in audio to represent 
    /// ***volume***, but they are sometimes a bit confusing to deal with.
    decibels: (ScreenParagraph, Decibels)
    // ------------------------------------------------------------------------
} 