use macros::Screen;
use indoc::indoc;

use crate::{
    leafy,
    screens::{
        faust::example, layouts::{
            sidebyside::SideBySide, 
            plainfull::PlainFull,
            Layout, 
            LayoutEnum
        }, Screen, ScreenList, ScreenParagraph
    }, 
    widgets::faustblock::FaustWidget
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏━╸┏┓╻╺┳╸┏━╸┏━┓   ┏━╸┏━┓╻ ╻┏━┓╺┳╸
┣╸ ┃┗┫ ┃ ┣╸ ┣┳┛   ┣╸ ┣━┫┃ ┃┗━┓ ┃ 
┗━╸╹ ╹ ╹ ┗━╸╹┗╸   ╹  ╹ ╹┗━┛┗━┛ ╹ 
"};

// const HELLO440: &'static str = example!("basics/hello440.dsp");

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Faust: intro")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct FaustIntro {
    // ------------------------------------------------------------------------
    /// **Faust** (*Functional Audio Stream*) is a programming language
    /// specifically made for ***audio DSP and synthesis***.
    /// It was created by **Yann Orlarey**, **Dominique Fober** & 
    /// **Stéphane Letz** at **GRAME** in 2002.
    p0: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------    
    /// **+** *Functional* paradigm,
    /// **+** Declarative, math-like syntax,
    /// **+** Produces optimized code for many architectures,
    /// **-** Not recommended (yet) for multi-rate (FFT)
    p1: ScreenList,
    // ------------------------------------------------------------------------
    /// • **Plugins**: VST, CLAP, AudioUnit
    /// • **Software**: Max, Pd, SuperCollider, Csound
    /// • **OS**: Linux, macOS, Windows, Android, iOS
    /// • **Embedded**: Bela, Teensy, Daisy, ESP32, FPGA
    /// • **Code**: C/C++, Rust, WASM, Java...
    p2: ScreenList,
    // ------------------------------------------------------------------------
    /// Tutorials & documentation available at: ***https://faust.grame.fr***
    p3: ScreenParagraph,
    // ------------------------------------------------------------------------
    /// Dedicated online IDE: ***https://faustide.grame.fr***
    p4: ScreenParagraph,
}

