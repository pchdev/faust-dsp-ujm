use indoc::indoc;
use macros::Screen;

use crate::{
    screens::{
        layouts::{
            plainfull::PlainFull, 
            sidebyside::SideBySide, 
            Layout, 
            LayoutEnum
        }, 
        Screen, ScreenList, ScreenParagraph
    },
    widgets::{
        aliasing::Aliasing, 
        quantization::Quantization, 
        sampling::SamplingIllustration
    }, 
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
╺┳┓╻┏━╸╻╺┳╸┏━┓╻     ┏━┓╻┏━╸┏┓╻┏━┓╻  
 ┃┃┃┃╺┓┃ ┃ ┣━┫┃     ┗━┓┃┃╺┓┃┗┫┣━┫┃  
╺┻┛╹┗━┛╹ ╹ ╹ ╹┗━╸   ┗━┛╹┗━┛╹ ╹╹ ╹┗━╸
"};


#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Digital Audio Signal (1/2)")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct Digital {
    // ------------------------------------------------------------------------
    /// • To ***digitize*** a continuous signal implies *discretizing* it.
    /// This is made possible by an *Analog-to-Digital Conversion* (**ADC**) process,
    /// which implies two key elements: ***sampling*** and ***quantization.***    
    p0: ScreenParagraph,
    // ------------------------------------------------------------------------    
    /// ***Sampling*** means taking a sample of a signal at a certain 
    /// frequency/rate (***sample rate***).    
    p1: (ScreenParagraph, SamplingIllustration),
    // ------------------------------------------------------------------------    
    /// • **Audio CD**: *44.1 kHz*
    /// • **Pro Audio**: *48/96 kHz*
    /// • **MP3**: *320/256/128/96/64* **kbps**
    p2: ScreenList,
    // ------------------------------------------------------------------------    
    /// • Because of ***aliasing***, the *sampling rate* must be 
    /// **at least two times superior** to
    /// the **highest frequency** we want to represent (*Nyquist-Shannon*).
    p3: (ScreenParagraph, Aliasing)
    // ------------------------------------------------------------------------    
}

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Digital Audio Signal (2/2)")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct Digital2 {
    // ------------------------------------------------------------------------
    /// • Once we take a sample of a signal at a given time, 
    /// we need to determine the ***scale of its value***,
    /// this is called ***quantization***. 
    /// Increasing the scale implies reducing the ***quantization noise***
    /// (*quality vs. storage tradeoff*).    
    p0: (ScreenParagraph, Quantization),
    // ------------------------------------------------------------------------    
    /// • **Audio CD**: 16-bits (65,536 values, 98 dB SNR)
    /// • **Pro Audio**: 24-bits (~16,7 mil., 146 dB SNR)
    /// • ***DSP***: 32/64-bits float (~4,3 bil., 194 dB SNR)
    p1: ScreenList,
    // ------------------------------------------------------------------------
    /// • For **DSP**, it is easier to make computations in 
    /// ***floating-point*** (decimal), and *normalize* the signal between 
    /// *-1.0* and *1.0*.
    p2: ScreenParagraph,
    // ------------------------------------------------------------------------
    /// • Finally, sending a digital signal to audio speakers involves 
    /// the inverse process of an **ADC**: 
    /// *Digital-to-Analog Conversion* (**DAC**).    
    p3: ScreenParagraph,
}

