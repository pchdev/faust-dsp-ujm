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
┏━╸╻ ╻┏┓╻┏━╸╺┳╸╻┏━┓┏┓╻┏━┓
┣╸ ┃ ┃┃┗┫┃   ┃ ┃┃ ┃┃┗┫┗━┓
╹  ┗━┛╹ ╹┗━╸ ╹ ╹┗━┛╹ ╹┗━┛
"};

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(description = "Faust: functions")]
#[screen(layout = LayoutEnum::SideBySide)]
pub struct FaustFunctions {
    // ------------------------------------------------------------------------
    /// ***Function definitions*** in Faust have the syntax
    ///  *function(parameter1, parameter2, ...) = expression;*",
    #[faust(example!("functions/f1.dsp"))]
    f1: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// In order to ***call*** (**execute**) that function, we need to ***replace its parameters (arguments)*** \
    /// with the values we want to pass as parameters.
    #[faust(example!("functions/f2.dsp"))]
    f2: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// Arguments are only valid inside of the function definition, 
    /// we say that they are *local to the **scope** of the function*. 
    /// This also means that they ***take precedence*** over any other variable 
    /// or expression that have the same name in the code.
    #[faust(example!("functions/f3.dsp"))]
    f3: (ScreenParagraph, FaustWidget),
    // ------------------------------------------------------------------------
    /// In Faust, anything can be passed as a ***function argument***...
    #[faust(example!("functions/f4.dsp"))]
    f4: (ScreenParagraph, FaustWidget),    
    // ------------------------------------------------------------------------
    /// ... including functions!
    #[faust(example!("functions/f5.dsp"))]
    f5: (ScreenParagraph, FaustWidget),
}



