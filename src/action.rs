use std::time::Duration;
use crossterm::event::{KeyCode, KeyEvent};

pub enum Action {
    Quit,
    TogglePause,
    Next,
    Prev,
    // TODO: gg needs key buffer with timeout
    First,
    Last,
    Play,
    FastForward,
    Rewind,
    PlayNext,
    UpdateCurrentPos(Duration),
    UpdateTotalDuration(Duration),
    StartSearch,
    DelCharFromSearchStr,
    AppendCharForSearchStr(char),
    StopSearch,
    Search,
    SearchNext(bool),
    SearchPrev,
    Delete,
    RefreshPlaylist,
    CyclePlayMode,
}
pub fn map_key(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('p') => Some(Action::TogglePause),
        KeyCode::Char('j') => Some(Action::Next),
        KeyCode::Char('k') => Some(Action::Prev),
        KeyCode::Char('G') => Some(Action::Last),
        KeyCode::Char('c') => Some(Action::Play),
        KeyCode::Char('h') => Some(Action::Rewind),
        KeyCode::Char('l') => Some(Action::FastForward),
        KeyCode::Char('/') => Some(Action::StartSearch),
        KeyCode::Char('n') => Some(Action::SearchNext(true)),
        KeyCode::Char('N') => Some(Action::SearchPrev),
        KeyCode::Char('d') => Some(Action::Delete),
        KeyCode::Char('r') => Some(Action::RefreshPlaylist),
        KeyCode::Char('m') => Some(Action::CyclePlayMode),
        _ => None,
    }
}

pub fn map_key_in_search_mode(key: KeyEvent) -> Option<Action> {
    match key.code {
        KeyCode::Enter => Some(Action::Search),
        KeyCode::Backspace => Some(Action::DelCharFromSearchStr),
        KeyCode::Esc => Some(Action::StopSearch),
        KeyCode::Char(c) => Some(Action::AppendCharForSearchStr(c)),
        _ => None,
    }
}
