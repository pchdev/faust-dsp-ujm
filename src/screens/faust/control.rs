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
┏━╸┏━┓┏┓╻╺┳╸┏━┓┏━┓╻  ┏━┓
┃  ┃ ┃┃┗┫ ┃ ┣┳┛┃ ┃┃  ┗━┓
┗━╸┗━┛╹ ╹ ╹ ╹┗╸┗━┛┗━╸┗━┛
"};

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Faust: MIDI/OSC controls")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct FaustControls {
    // ------------------------------------------------------------------------
    /// MIDI
    #[faust(example!("controls/midi.dsp"))]
    _midi: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// OSC
    #[faust(example!("controls/osc.dsp"))]
    _osc: (ScreenParagraph, FaustWidget),    
}