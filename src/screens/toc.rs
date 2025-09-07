use ratatui::{
    buffer::Buffer, layout::{
        Constraint, Flex, Layout, Rect
    }, style::Stylize, text::{
        Line, Text
    }, widgets::{Paragraph, Widget, WidgetRef}
};


const TOC: &'static str = r"


╺┳╸┏━┓┏┓ ╻  ┏━╸   ┏━┓┏━╸   ┏━╸┏━┓┏┓╻╺┳╸┏━╸┏┓╻╺┳╸┏━┓
 ┃ ┣━┫┣┻┓┃  ┣╸    ┃ ┃┣╸    ┃  ┃ ┃┃┗┫ ┃ ┣╸ ┃┗┫ ┃ ┗━┓
 ╹ ╹ ╹┗━┛┗━╸┗━╸   ┗━┛╹     ┗━╸┗━┛╹ ╹ ╹ ┗━╸╹ ╹ ╹ ┗━┛
";

const TOC_PH: &'static str = r"

1. Agenda

2. Sound

3. Signal

4. etc. 
";

#[derive(Debug, Default)]
pub struct TableOfContents;

impl WidgetRef for TableOfContents {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let lv = Layout::vertical([
                Constraint::Percentage(25),
                Constraint::Percentage(75)
            ])
            .flex(Flex::Center)
            .split(area)
        ;
        let lh = Layout::horizontal([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .flex(Flex::Center)
            .split(lv[1])
        ;
        let mut subtitle = Paragraph::new(TOC)
            .centered()
        ;
        let mut txt = Paragraph::new(TOC_PH)
            // .centered()
            .bold()
        ;
        subtitle.render(lv[0], buf);
        txt.render(lh[1], buf);
    }
}