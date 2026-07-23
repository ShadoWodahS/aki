use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent};

pub enum Action {
    Quit,
    TogglePause,
    Next,
    Prev,
    Play,
    FastForward,
    Rewind,
    PlayNext,
    UpdateCurrentPos(Duration),
    UpdateTotalDuration(Duration),
}

pub fn map_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('p') => Some(Action::TogglePause),
        KeyCode::Char('j') => Some(Action::Next),
        KeyCode::Char('k') => Some(Action::Prev),
        KeyCode::Char('c') => Some(Action::Play),
        KeyCode::Char('h') => Some(Action::Rewind),
        KeyCode::Char('l') => Some(Action::FastForward),
        _ => None,
    }
}
