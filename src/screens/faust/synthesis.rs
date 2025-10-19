
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{WidgetRef}
};

use indoc::indoc;

use crate::{
    screens::{leafy, Screen, SideBySide}, 
    widgets::{faustblock::FaustWidget}
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏━┓╻ ╻┏┓╻╺┳╸╻ ╻┏━╸┏━┓╻┏━┓
┗━┓┗┳┛┃┗┫ ┃ ┣━┫┣╸ ┗━┓┃┗━┓
┗━┛ ╹ ╹ ╹ ╹ ╹ ╹┗━╸┗━┛╹┗━┛
"};

macro_rules! example {
    ($path:literal) => {
        Box::new(
            FaustWidget::new(
                include_str!(
                    concat!("../../../examples/spectrum/", $path)
                )
            )
        )        
    };
}

pub struct FaustSynthesis<'a> {
    screen: SideBySide<'a>,
}

impl<'a> WidgetRef for FaustSynthesis<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        self.screen.render_ref(area, buf);
    }
}

impl<'a> Screen for FaustSynthesis<'a> {
    fn title(&self) -> &'static str {
        "Faust: basic synthesis"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        self.screen.on_key_event(k);
    }
    fn on_tick(&mut self, t: usize) {
        self.screen.on_tick(t);
    }
}

impl<'a> Default for FaustSynthesis<'a> {
    fn default() -> Self {
        FaustSynthesis {
            screen: SideBySide::default()
                .add_title(TITLE)
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "Faust has a in its libraries a good collection of '*basic*' **oscillators**, with different \
                    ***waveforms***: *sine*, *triangle*, *sawtooth*, *square*, *etc.* "
                })
                .add_widget(0, example!("oscillators.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "When a Faust program starts to be a little more complex, its always good practice to \
                    ***refactor code*** by using ***custom functions*** with variable **parameters**."
                })
                .add_widget(1, example!("functions.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "In Faust, **functions** can take ***any element of the language as parameters***, including \
                    **GUI elements**."
                })
                .add_widget(2, example!("functions-gui.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "Finally, the ***select*** primitive (an equivalent to ***switch*** in Max), allow to \
                    select an input from a list. It can be used in this case to *switch between **waveforms***"
                })
                .add_widget(3, example!("select.dsp"))
                // ---------------------------------------------------------------------------------------                
                .add_paragraph(leafy! {
                    "Our goal now will be to ***apply this to our previous synthesizer.***"
                })
                .add_widget(4, example!("switch-waveform.dsp"))
                // ---------------------------------------------------------------------------------------                
                ,
        }
    }
}

