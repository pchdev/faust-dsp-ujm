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
#[screen(description = "Faust: time (1/2)")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct FaustTime {
    // ------------------------------------------------------------------------
    /// Faust has one final composition operator: 
    /// the ***recursive operator*** ('**~**')
    /// which allows to ***connect two signals recursively*** 
    /// (with ***a delay of one sample***).
    #[faust(example!("time/recursive.dsp"))]
    recursive: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// The recursive operator is typically used to implement **counters**, 
    /// and/or ***to represent time***.
    /// With it, we can count for instance *up to 44100 or 48000 samples* 
    /// to ***represent one second of audio time***.
    #[faust(example!("time/counter_samples.dsp"))]
    counter_samples: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// When implementing ***time-related functions***, 
    /// it can prove very useful to ***monitor values from the GUI***.
    #[faust(example!("time/monitoring_values.dsp"))]
    monitoring: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// We can for example use a counter to ***switch our synth's 
    /// waveform automatically every second.***
    #[faust(example!("time/waveform_switch_counter.dsp"))]
    waveform: (ScreenParagraph, FaustWidget),
}

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Faust: time (2/2)")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct FaustPhasor {
    // ------------------------------------------------------------------------
    /// A counter can be used as a base for many many things.
    /// With it, we can for instance build a ***ramp signal***
    /// that *goes from 0 to 1 repeatedly at a certain rate*.
    #[faust(example!("time/phasor.dsp"))]
    phasor: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// • Notice anything? A *ramp* is very much like a 
    /// *sawtooth oscillator*, only with a different range 
    /// (0 to 1 instead of -1 to 1).
    _f0: ScreenParagraph,
    // ------------------------------------------------------------------------
    #[faust(example!("time/waveforms.dsp"))]
    /// With a *ramp* (or *phasor*), we can pretty much already build ***all of 
    /// the simple waveform oscillators***, with a few operations...
    waveforms: (ScreenParagraph, FaustWidget),    
    // ------------------------------------------------------------------------
    #[faust(example!("time/tables.dsp"))]
    /// *Ramps* are often used as ***cursors*** to *read* or 
    /// *write samples* in a ***buffer***.
    /// In Faust, *buffers* are implemented with the *rdtable* & *rwtable*
    /// primitives.
    tables: (ScreenParagraph, FaustWidget),    
        // ------------------------------------------------------------------------
    #[faust(example!("time/soundfile.dsp"))]
    /// To playback a ***sound file***, we also typically use a phasor 
    /// as a *read cursor*. 
    sfile: (ScreenParagraph, FaustWidget),    

}

const DELAYS: &'static str = indoc!{"
╺┳┓┏━╸╻  ┏━┓╻ ╻┏━┓
 ┃┃┣╸ ┃  ┣━┫┗┳┛┗━┓
╺┻┛┗━╸┗━╸╹ ╹ ╹ ┗━┛
"};

#[derive(Screen, Default)]
#[screen(title = DELAYS)]
#[screen(description = "Faust: delays")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct FaustDelays {
    // ------------------------------------------------------------------------
    /// Delaying a signal in Faust can be done using the operator '**@**'.
    /// '***x @ n***' delays a signal 'x' by 'n' **samples**.
    #[faust(example!("time/delays.dsp"))]
    delay: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------* 
    /// ***Delay times*** can usually be changed in real time from **GUI**,
    /// but it will in this case introduce unwanted '*clicks*'.
    /// We also here define a '***Dry/Wet***' control.
    #[faust(example!("time/delays-sliders.dsp"))]
    sliders: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// Another typical control parameter for delays is the '***feedback coefficient***',
    /// which allows the delay to *repeat itself a certain number of times*
    /// before fading away. 
    /// This will require to ***use the recursive operator***.
    #[faust(example!("time/delays-feedback.dsp"))]
    feedback: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
}