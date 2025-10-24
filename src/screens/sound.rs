use indoc::indoc;
use macros::Screen;

use crate::{
    screens::{
        layouts::{
            LayoutEnum,
            plainfull::PlainFull, 
            sidebyside::SideBySide, 
            Layout
        }, Screen, ScreenList, ScreenParagraph
    }, 
    widgets::{
        particles::Particles, 
        ripple::Ripple, 
    }
};

const TITLE: &'static str = indoc!{"
┏━┓┏━┓╻ ╻┏┓╻╺┳┓
┗━┓┃ ┃┃ ┃┃┗┫ ┃┃
┗━┛┗━┛┗━┛╹ ╹╺┻┛
"};



#[derive(Screen, Default)]
#[screen(layout = LayoutEnum::SideBySide)]
#[screen(title = TITLE)]
#[screen(description = "Sound (1/2)")]
pub struct Sound {
    // ------------------------------------------------------------------------
    /// Sound is a ***pressure wave*** that propagates
    /// through a **medium** (*gas*, *liquid* or *solid*).
    _p1: (ScreenParagraph, Ripple),
    // ------------------------------------------------------------------------
    /// Propagation is caused by the **oscillation** (*vibration*) of
    /// the medium's *particles*, around their ***equilibrium*** positions.
    _p2: (ScreenParagraph, Particles),
    // ------------------------------------------------------------------------
    /// Sound has the following **properties**:
    _p3: ScreenParagraph,
    // ------------------------------------------------------------------------
    /// • **Speed**: ~343 m/s in **air**",
    /// • **Amplitude**: in *Pascals* (***Pa***) or *Decibels* (***dB***)",
    /// • **Period**: time between two oscillations",
    /// • **Wavelength**: distance between two oscillations",
    /// • **Frequency**: cycles/sec., in *Hertz* (***Hz***, ***kHz***, ***MHz***)",
    /// • **Spectrum**, or *Timbre*"
    _l0: ScreenList
}

#[derive(Screen, Default)]
#[screen(layout = LayoutEnum::Plainfull)]
#[screen(title = TITLE)]
#[screen(description = "Sound (2/2)")]
pub struct Sound2 {
    // ------------------------------------------------------------------------
    /// • Our ***perception*** of sound is made from the conversion of 
    /// the vibrations reaching our ***eardrums*** to
    /// a *signal of nerve impulses*, transmitted and interpreted by **the brain**."
    _p1: ScreenParagraph,
    // ------------------------------------------------------------------------
    /// • Human ears can typically identify sounds ***from 20 Hz to 20 kHz***.
    _p2: ScreenParagraph,
    // ------------------------------------------------------------------------
    /// • **Bat**: 2000 to 110,000 Hz"
    /// • **Porpoise**: 75 to 150,000 Hz"
    /// • **Cat**: 45 to 64,000 Hz"
    /// • **Dog**: 67 to 45,000 Hz"
    /// • **Chicken**: 125 to 2,000 Hz
    _l1: ScreenList,
}

