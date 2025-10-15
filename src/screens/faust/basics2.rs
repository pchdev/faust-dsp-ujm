
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{WidgetRef}
};

use indoc::indoc;

use crate::{
    screens::{leafy, Screen, SideBySide}, 
    widgets::faustblock::FaustWidget
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
                    "In Faust, connecting DSP functions can be done using the ***sequential operator*** '**:**'."
                })
                .add_widget(0, example!("sequential.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "We can use the ***split operator*** '**<:**' and ***cable operator*** to connect our \
                    signal to multiple targets."
                })
                .add_widget(1, example!("split-cable.dsp"))
                // ---------------------------------------------------------------------------------------

                ,
        }
    }
}

