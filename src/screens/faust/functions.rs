
use crossterm::event::{KeyEvent};

use ratatui::{
    buffer::Buffer, 
    prelude::Rect, 
    widgets::{WidgetRef}
};

use indoc::indoc;

use crate::{
    leafy,
    screens::{layouts::{sidebyside::SideBySide, Layout}, Screen}, 
    widgets::faustblock::FaustWidget
};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
┏━╸╻ ╻┏┓╻┏━╸╺┳╸╻┏━┓┏┓╻┏━┓
┣╸ ┃ ┃┃┗┫┃   ┃ ┃┃ ┃┃┗┫┗━┓
╹  ┗━┛╹ ╹┗━╸ ╹ ╹┗━┛╹ ╹┗━┛
"};

macro_rules! example {
    ($path:literal) => {
        Box::new(
            FaustWidget::new(
                include_str!(
                    concat!("../../../examples/functions/", $path)
                )
            )
        )        
    };
}

pub struct FaustFunctions<'a> {
    layout: SideBySide<'a>,
}

impl<'a> Screen for FaustFunctions<'a> {
    fn title(&self) -> &'static str {
        TITLE
    }
    fn description(&self) -> &'static str {
        "Faust: functions"
    }
    fn layout(&self) -> Option<&dyn Layout> {
        Some(&self.layout)
    }
    fn layout_mut(&mut self) -> Option<&mut dyn Layout> {
        Some(&mut self.layout)
    }
}

impl<'a> Default for FaustFunctions<'a> {
    fn default() -> Self {
        FaustFunctions {
            layout: SideBySide::default()
                .add_title(TITLE)
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "***Function definitions*** in Faust have the syntax *function(parameter1, parameter2, ...) = expression;*"
                })
                .add_widget(0, example!("f1.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "In order to ***call*** (**execute**) that function, we need to ***replace its parameters (arguments)*** \
                    with the values we want to pass as parameters."
                })
                .add_widget(1, example!("f2.dsp"))
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "Arguments are only valid inside of the function definition, \
                    we say that they are *local to the **scope** of the function*. \
                    This also means that they ***take precedence*** over any other variable \
                    or expression that have the same name in the code."
                })
                .add_widget(2, example!("f3.dsp"))                
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "In Faust, anything can be passed as a ***function argument***..."
                })
                .add_widget(3, example!("f4.dsp"))                
                // ---------------------------------------------------------------------------------------
                .add_paragraph(leafy! {
                    "... including functions!"
                })
                .add_widget(4, example!("f5.dsp"))                
                ,
        }
    }
}

