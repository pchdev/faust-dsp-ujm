use indoc::indoc;

use crate::{
    screens::{faust::example, layouts::{sidebyside::{side_by_side, SideBySide}, Layout}, Screen}, 
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏━╸╻ ╻┏┓╻┏━╸╺┳╸╻┏━┓┏┓╻┏━┓
┣╸ ┃ ┃┃┗┫┃   ┃ ┃┃ ┃┃┗┫┗━┓
╹  ┗━┛╹ ╹┗━╸ ╹ ╹┗━┛╹ ╹┗━┛
"};

side_by_side!(
    name: FaustFunctions,
    title: TITLE,
    description: "Faust: functions",
    contents: [
        wparagraph: {
            "***Function definitions*** in Faust have the syntax *function(parameter1, parameter2, ...) = expression;*",
            example!("functions/f1.dsp")
        },
        wparagraph: {
            "In order to ***call*** (**execute**) that function, we need to ***replace its parameters (arguments)*** \
            with the values we want to pass as parameters.",
            example!("functions/f2.dsp")
        }, 
        wparagraph: {
            "Arguments are only valid inside of the function definition, \
            we say that they are *local to the **scope** of the function*. \
            This also means that they ***take precedence*** over any other variable \
            or expression that have the same name in the code.",
            example!("functions/f3.dsp")
        },
        wparagraph: {
            "In Faust, anything can be passed as a ***function argument***...",
            example!("functions/f4.dsp")
        },
        wparagraph: {
            "... including functions!",
            example!("functions/f5.dsp")
        }
    ]
);

