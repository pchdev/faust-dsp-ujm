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
┏━┓╻ ╻┏┓╻╺┳╸╻ ╻┏━╸┏━┓╻┏━┓
┗━┓┗┳┛┃┗┫ ┃ ┣━┫┣╸ ┗━┓┃┗━┓
┗━┛ ╹ ╹ ╹ ╹ ╹ ╹┗━╸┗━┛╹┗━┛
"};

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Faust: basic synthesis")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct FaustSynthesis {
    // ------------------------------------------------------------------------
    /// Faust has a in its libraries a good collection of '*basic*' **oscillators**, with different
    /// ***waveforms***: *sine*, *triangle*, *sawtooth*, *square*, *etc.* 
    #[faust(example!("synthesis/oscillators.dsp"))]
    oscillators: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// When a Faust program starts to be a little more complex, its always good practice to
    /// ***refactor code*** by using ***custom functions*** with variable **parameters**.
    #[faust(example!("synthesis/functions.dsp"))]
    functions: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// In Faust, **functions** can take ***any element of the language 
    /// as parameters***, including **GUI elements**.
    #[faust(example!("synthesis/functions-gui.dsp"))]
    functions_gui: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// Finally, the ***select*** primitive (an equivalent to ***switch*** in Max), allow to \
    /// select an input from a list. It can be used in this case to *switch between **waveforms***
    #[faust(example!("synthesis/select.dsp"))]
    select: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// Our goal now will be to ***apply this to our previous synthesizer.***
    #[faust(example!("synthesis/switch-waveform.dsp"))]
    switch_waveform: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
}

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Faust: additive synthesis")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct FaustSynthesisAdditive {
    // ------------------------------------------------------------------------
    /// Waveform or Wavetable synthesis is not the only method that we can use
    /// in order to create sound. We can also use ***additive synthesis***,
    /// which is the superposition of oscillators running at different frequencies.
    /// One really fast way to do this in Faust is to use the ***par*** operator,
    /// which puts signal expressions ***in parallel*** dynamically.
    #[faust(example!("synthesis/additive.dsp"))]
    switch_waveform: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------    
}

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Faust: subtractive synthesis")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct FaustSynthesisSubtractive {

}

