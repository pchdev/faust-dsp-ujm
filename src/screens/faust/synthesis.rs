use indoc::indoc;

use crate::screens::{
        faust::example, layouts::{
            sidebyside::{
                side_by_side, 
                SideBySide
            }, 
            Layout
        }, Screen
    };

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏━┓╻ ╻┏┓╻╺┳╸╻ ╻┏━╸┏━┓╻┏━┓
┗━┓┗┳┛┃┗┫ ┃ ┣━┫┣╸ ┗━┓┃┗━┓
┗━┛ ╹ ╹ ╹ ╹ ╹ ╹┗━╸┗━┛╹┗━┛
"};

side_by_side!(
    name: FaustSynthesis,
    title: TITLE,
    description: "Faust: basic synthesis",
    contents: [
        wparagraph: {
            "Faust has a in its libraries a good collection of '*basic*' **oscillators**, with different \
            ***waveforms***: *sine*, *triangle*, *sawtooth*, *square*, *etc.* ",
            example!("synthesis/oscillators.dsp")
        },
        wparagraph: {
            "When a Faust program starts to be a little more complex, its always good practice to \
            ***refactor code*** by using ***custom functions*** with variable **parameters**.",
            example!("synthesis/functions.dsp")
        },
        wparagraph: {
            "In Faust, **functions** can take ***any element of the language as parameters***, including \
            **GUI elements**.",
            example!("synthesis/functions-gui.dsp")
        },
        wparagraph: {
            "Finally, the ***select*** primitive (an equivalent to ***switch*** in Max), allow to \
            select an input from a list. It can be used in this case to *switch between **waveforms***",
            example!("synthesis/select.dsp")
        },
        wparagraph: {
            "Our goal now will be to ***apply this to our previous synthesizer.***",
            example!("synthesis/switch-waveform.dsp")
        }
    ]
);

