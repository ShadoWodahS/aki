use crossterm::event::{KeyCode, KeyEvent};

pub enum Action {
    Quit,
    TogglePause,
    Next,
    Prev,
    Play,
}

pub fn map_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('p') => Some(Action::TogglePause),
        KeyCode::Char('j') => Some(Action::Next),
        KeyCode::Char('k') => Some(Action::Prev),
        KeyCode::Char('c') => Some(Action::Play),
        _ => None,
    }
}
