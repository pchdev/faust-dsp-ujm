
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{WidgetRef}
};

use indoc::indoc;

use crate::{
    screens::{leafy, Screen, SideBySide}, 
    widgets::{db::Decibels, faustblock::FaustWidget}
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏┓ ┏━┓┏━┓╻┏━╸┏━┓
┣┻┓┣━┫┗━┓┃┃  ┗━┓
┗━┛╹ ╹┗━┛╹┗━╸┗━┛
"};

macro_rules! example {
    ($path:literal) => {
        Box::new(
            FaustWidget::new(
                include_str!(
                    concat!("../../../examples/basics2/", $path)
                )
            )
        )        
    };
}

pub struct FaustBasics2<'a> {
    screen: SideBySide<'a>,
}

impl<'a> WidgetRef for FaustBasics2<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {   
        self.screen.render_ref(area, buf);
    }
}

impl<'a> Screen for FaustBasics2<'a> {
    fn title(&self) -> &'static str {
        "Faust: basics (2)"
    }
    fn on_key_event(&mut self, k: KeyEvent) {
        self.screen.on_key_event(k);
    }
    fn on_tick(&mut self, t: usize) {
        self.screen.on_tick(t);
    }
}

impl<'a> Default for FaustBasics2<'a> {
    fn default() -> Self {
        FaustBasics2 {
            screen: SideBySide::default()
                .add_title(TITLE)
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "In Faust, *connecting DSP functions* can be done using the ***sequential operator*** '**:**'."
                })
                .add_widget(0, example!("sequential.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "The ***split operator*** '**<:**' and ***cable operator*** can also be used to connect a \
                    signal to *multiple targets*."
                })
                .add_widget(1, example!("split-cable.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "On the other hand, the ***merge operator*** '**:>**' can be used to \
                    **merge** (**mix**) **signals together**."
                })
                .add_widget(2, example!("merge-cable.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "*Graphical User Interface* (***GUI***) elements can be added to ***control parameters***: \
                    **sliders** (*hslider*/*vslider*), **buttons**, **switches...**"
                })
                .add_widget(3, example!("sliders.dsp"))
                // ---------------------------------------------------------------------------------------                
                .add_paragraph(leafy! {
                    "They can be used to make a proper ***gain*** control in ***dB*** for example."
                })
                .add_widget(4, example!("gain.dsp"))
                // ---------------------------------------------------------------------------------------                
                .add_paragraph(leafy! {
                    "***Decibels*** (***dB***) are frequently used in audio to represent ***volume***, but they \
                    are sometimes a bit confusing to deal with."
                })
                .add_widget(5, Box::new(Decibels::default()))
                // ---------------------------------------------------------------------------------------                
                ,
        }
    }
}

