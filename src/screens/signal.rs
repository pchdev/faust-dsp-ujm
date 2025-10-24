use indoc::indoc;
use macros::Screen;

use crate::{
    screens::{
        layouts::{
            plainfull::PlainFull, 
            sidebyside::SideBySide, 
            Layout, LayoutEnum
        }, 
        Screen, ScreenParagraph
    },
    widgets::{
        spectrogram::SpectrumCanvas, 
        waveform::Waveform
    }, 
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏━┓╻┏━╸┏┓╻┏━┓╻  
┗━┓┃┃╺┓┃┗┫┣━┫┃  
┗━┛╹┗━┛╹ ╹╹ ╹┗━╸
"};


#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Signal (1/2)")]
#[screen(layout = LayoutEnum::Plainfull)]
pub struct Signal {
    // ------------------------------------------------------------------------
    /// • A ***signal*** describes the evolution of data *over time*.
    /// In our case, the vibration of an entity (
    /// like the *membrane* of a ***microphone***).
    p0: ScreenParagraph,
    // ------------------------------------------------------------------------
    /// • Just like sound waves turning into nerve impulses, 
    /// the analyzed data usually needs to be first converted 
    /// to another *physical unit*, or *domain* (***transduction***) 
    /// in order to adapt to measurement/processing tools.
    p1: ScreenParagraph,
    // ------------------------------------------------------------------------    
    /// • The vibration of a microphone's membrane is, for instance, 
    /// usually converted to ***continuous electrical current***
    /// before it can be processed and/or analyzed. 
    /// In this case, the signal is said to be ***"analog"***.
    p2: ScreenParagraph,    
}

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Signal (2/2)")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct Signal2 {
    // ------------------------------------------------------------------------
    /// With an oscilloscope, we can measure the **amplitude** of a signal 
    /// at a given *point in time* (***time-domain***),
    /// through the visualisation of a ***waveform***.
    pw0: (ScreenParagraph, Waveform),
    // ------------------------------------------------------------------------
    /// • An analog signal can already be processed as it is, with ***analog effects***: 
    /// *tape delay*, *distortion*, *chorus*, *reverberation (spring, plate)*, etc.
    p0: ScreenParagraph,
    // ------------------------------------------------------------------------
    /// On the other hand, it is difficult to extract precise information 
    /// about ***frequency*** and ***spectrum***.
    /// For this purpose, it's far more efficient to switch to 
    /// the ***frequency domain***, which requires the analog signal to 
    /// be turned into a ***digital signal***...
    pw1: (ScreenParagraph, SpectrumCanvas),
    // ------------------------------------------------------------------------
}

