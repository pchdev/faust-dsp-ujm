
use ratatui::{
    buffer::Buffer, layout::{
        Constraint, Flex, Layout, Rect
    }, style::Stylize, text::{
        Line, Text
    }, widgets::{Widget, WidgetRef}
};

/// Got from: https://patorjk.com/software/taag/
/// Font is 'ANSI Shadow'
const FAUST: &'static str = r"
███████╗ █████╗ ██╗   ██╗███████╗████████╗
██╔════╝██╔══██╗██║   ██║██╔════╝╚══██╔══╝
█████╗  ███████║██║   ██║███████╗   ██║   
██╔══╝  ██╔══██║██║   ██║╚════██║   ██║   
██║     ██║  ██║╚██████╔╝███████║   ██║   
╚═╝     ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   
                                          
";

const HEADER: &'static str = r"
 MASTER CCNT(3), DIGICREA(1), Université Jean Monnet, Saint-Etienne, Oct-Nov 2025 
";

const FOOTER: &'static str = r"
 Pierre Cochard - Emeraude Team - Inria, INSA-Lyon, CITI Laboratory 
";

#[derive(Debug, Default)]
pub struct Title;

impl WidgetRef for Title {
    fn render_ref(&self, area: Rect, buf: &mut Buffer)
    {
        let mut txt = Text::from(FAUST);
        txt.push_line("");
        txt.push_line(Line::from(
            "Digital Audio Processing and Synthesis")
            .bold()
        );
        let [lv] = Layout::vertical([
                Constraint::Length(txt.height() as u16),
            ])
            .flex(Flex::Center)
            .areas(area)
        ;
        let [lh] = Layout::horizontal([
                Constraint::Length(txt.width() as u16)
            ])
            .flex(Flex::Center)
            .areas(lv)
        ;
        txt.render(lh, buf);
    }
}