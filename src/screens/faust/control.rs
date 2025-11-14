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
    /// ***GUI elements*** in Faust can be ordered and gathered into
    /// horizontal/vertical or tab *layouts*, using '**groups**' or a special
    /// **path syntax** in their names. 
    #[faust(example!("controls/groups.dsp"))]
    _groups: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// ***MIDI support*** can be added to any Faust program, simply by adding 
    /// **metadata** on the **GUI elements** that we want to control. 
    #[faust(example!("controls/midi.dsp"))]
    _midi: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// ***Open Sound Control*** (***OSC***) support follows the same *metadata* logic. 
    /// ***OSC*** is a protocol for ***exchanging control data*** between *multimedia devices*,
    /// such as *synthesizers*, *DAWs*, *audio programming languages*, etc.
    #[faust(example!("controls/osc.dsp"))]
    _osc: (ScreenParagraph, FaustWidget),    
    // ------------------------------------------------------------------------
    /// Finally ***HTTP control*** can be enabled in the same way, which allows
    /// *remote-control of a Faust program* within a **browser interface**.
    #[faust(example!("controls/http.dsp"))]
    _http: (ScreenParagraph, FaustWidget),    
}