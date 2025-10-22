use indoc::indoc;

use crate::leafy;
use crate::screens::layouts::Layout;
use crate::screens::{layouts::plainfull::PlainFull, Screen};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
╻ ╻┏━╸╻  ╻  ┏━┓╻
┣━┫┣╸ ┃  ┃  ┃ ┃╹
╹ ╹┗━╸┗━╸┗━╸┗━┛╹
"};


pub struct Myself<'a> {
    layout: PlainFull<'a>,
}

impl<'a> Screen for Myself<'a> {
    fn title(&self) -> &'static str {
        TITLE
    }
    fn description(&self) -> &'static str {
        "Hello!"
    }
    fn layout(&self) -> Option<&dyn Layout> {
        Some(&self.layout)
    }
    fn layout_mut(&mut self) -> Option<&mut dyn Layout> {
        Some(&mut self.layout)
    }
}

impl<'a> Default for Myself<'a> {
    fn default() -> Self {
        Myself {
            layout: PlainFull::default()
                .add_title(TITLE)
                .add_paragraph(indoc! {
                    "• My name is **Pierre**, nice to meet you all!"
                }) 
                .add_paragraph(indoc! {
                    "• First time teaching... but **in your shoes**, *12 years ago* :)"
                }) 
                .add_paragraph(leafy! {
                    "Now in *Inria/INSA* team ***Emeraude***, working as a *research engineer* in *Lyon*, alongside:"
                }) 
                .add_list(vec![
                    "• **Romain Michon**",
                    "• **Stéphane Letz**",
                    "• Yann Orlarey",
                ])
                .add_paragraph(indoc! {
                    "• What I'm working on: "
                })
                .add_list(vec![
                    "• ***Syfala*** (*Faust-to-FPGA toolchain*)",
                    "• ***FloPoCo*** (*generator of arithmetic cores*) for FPGA",
                    "• ***Faust***"
                ])
                .add_paragraph(indoc! {
                    "• **P.S**: I'm not really an expert in DSP :("
                })
                ,   
        }
    }
}