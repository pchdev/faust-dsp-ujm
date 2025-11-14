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
    _oscillators: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// When a Faust program starts to be a little more complex, its always good practice to
    /// ***refactor code*** by using ***custom functions*** with variable **parameters**.
    #[faust(example!("synthesis/functions.dsp"))]
    _functions: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// In Faust, **functions** can take ***any element of the language 
    /// as parameters***, including **GUI elements**.
    #[faust(example!("synthesis/functions-gui.dsp"))]
    _functions_gui: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// Finally, the ***select*** primitive (an equivalent to ***switch*** in Max), allow to
    /// select an input from a list. It can be used in this case to *switch between **waveforms***
    #[faust(example!("synthesis/select.dsp"))]
    _select: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// Our goal now will be to ***apply this to our previous synthesizer.***
    #[faust(example!("synthesis/switch-waveform.dsp"))]
    _switch_waveform: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
}

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Faust: advanced synthesis (1/2)")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct FaustSynthesisAdvanced {
    // ------------------------------------------------------------------------
    /// *Waveform or Wavetable synthesis* is not the only method that we can use
    /// in order to create sound. We can also use ***additive synthesis***,
    /// which is the superposition of oscillators running at different frequencies.
    /// One really fast way to do this in Faust is to use the ***par*** operator,
    /// which puts signal expressions ***in parallel*** dynamically.
    #[faust(example!("synthesis/additive.dsp"))]
    _additive: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------    
    /// In most synthesizers, additive synthesis is done by ***mixing a few 
    /// oscillators together***, with *selectable waveforms*. 
    #[faust(example!("synthesis/additive-classic.dsp"))]
    _additive_classic: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------ 
    /// Usually, ***filtering*** is applied afterwards, to remove
    /// unwanted frequencies (***subtractive synthesis***).
    #[faust(example!("synthesis/subtractive.dsp"))]
    _subtractive: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------ 
    /// **Frequency-modulation** (***FM***) **synthesis** is another well-known
    ///  synthesis method, which implies *inter-modulated oscillator graphs*.
    #[faust(example!("synthesis/fm.dsp"))]
    _fm: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------ 
}

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Faust: advanced synthesis (2/2)")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct FaustSynthesisAdvanced2 {
    // ------------------------------------------------------------------------ 
    /// ***Physical model synthesis*** consists in ***emulating the physical properties
    /// of an instrument*** to reproduce its sound. Different methods exist in order
    /// to achieve this: **modal synthesis**, **FDTD**, **digital waveguides**, *etc.* 
    #[faust(example!("synthesis/pm.dsp"))]
    _pm: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------ 
    /// Faust has in its ***libraries*** ('pm' prefix) a good collection 
    /// of ***physically-modeled instruments***
    /// (*flutes*, *bells*, *guitars*, *voice*, *etc.*). 
    #[faust(example!("synthesis/pm2.dsp"))]
    _pm2: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------ --- 
    /// ***Granular synthesis*** is a method which uses overlapping '***grains***' 
    /// (short fragments of sound, around **1 to 100 milliseconds**)
    /// from a ***buffer***, which are going to be played at *different speeds*, *phases*,
    /// *volume* and *frequency*.
    #[faust(example!("synthesis/granular.dsp"))]
    _granular: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------ 
}

