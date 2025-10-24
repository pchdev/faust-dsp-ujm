use indoc::indoc;
use macros::Screen;
use ratatui::widgets::{Widget, WidgetRef};

use crate::leafy;
use crate::screens::layouts::{Layout, LayoutEnum};
use crate::screens::{ScreenList, ScreenParagraph};
use crate::screens::{layouts::plainfull::PlainFull, layouts::sidebyside::SideBySide, Screen};

/// Font is 'Future':
const TITLE: &'static str = indoc!{"
╻ ╻┏━╸╻  ╻  ┏━┓╻
┣━┫┣╸ ┃  ┃  ┃ ┃╹
╹ ╹┗━╸┗━╸┗━╸┗━┛╹
"};

#[derive(Screen, Default)]
#[screen(title = TITLE)]
#[screen(layout = LayoutEnum::Plainfull)]
#[screen(description = "Hello!")]
pub struct Myself {
    // ------------------------------------------------------------------------
    /// • My name is **Pierre**, nice to meet you all!
    p0: ScreenParagraph,
    // ------------------------------------------------------------------------    
    /// • First time teaching... but **in your shoes**, *12 years ago* :)
    p1: ScreenParagraph,
    // ------------------------------------------------------------------------    
    /// • Now in *Inria/INSA* team ***Emeraude***, working as 
    /// a *research engineer* in *Lyon*, alongside:
    p2: ScreenParagraph,
    // ------------------------------------------------------------------------    
    /// • **Romain Michon**"
    /// • **Stéphane Letz**"
    /// • Yann Orlarey"
    l0: ScreenList,
    // ------------------------------------------------------------------------    
    /// • What I'm working on:
    p3: ScreenParagraph,
    // ------------------------------------------------------------------------    
    /// • ***Syfala*** (*Faust-to-FPGA toolchain*)
    /// • ***FloPoCo*** (*generator of arithmetic cores*) for FPGA
    /// • ***Faust***    
    l1: ScreenList,
    // ------------------------------------------------------------------------    
    /// • **P.S**: I'm not really an expert in DSP :(
    p4: ScreenParagraph,
}