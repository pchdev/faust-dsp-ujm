use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    NextScreen,
    PrevScreen,
    Quit,
    OpenMenu,
    Passthrough,
}

pub fn map_key(ev: KeyEvent) -> Option<Command> {
    if ev.kind != KeyEventKind::Press {
        return None;
    }

    use Command::*;
    use KeyCode::*;

    match ev.code {
        // 👉 left/right
        Right => Some(NextScreen),
        Left => Some(PrevScreen),

        // 🔁 Fallback: Ctrl+Space, Ctrl+H itd. (opcjonalnie)
        Char(' ') if ev.modifiers.contains(KeyModifiers::CONTROL) => Some(NextScreen),
        Null if ev.modifiers.contains(KeyModifiers::CONTROL) => Some(NextScreen),
        Backspace => Some(PrevScreen),

        // ❌ exit
        Char('w') if ev.modifiers.contains(KeyModifiers::CONTROL) => Some(Quit),

        // 📜 Menu
        F(4) => Some(OpenMenu),

        _ => Some(Passthrough),
    }
}
