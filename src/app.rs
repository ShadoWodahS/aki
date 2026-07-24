use std::{path::PathBuf, time::Duration};

use crate::action::Action::{self};

pub struct App {
    should_quit: bool,
    is_playing: bool,
    playlist: Vec<PathBuf>,
    selected_song_index: usize,
    playing_song_index: Option<usize>,
    current_pos: Duration,
    total_duration: Duration,
    search_mode: bool,
    search_str: Option<String>,
    is_searching: bool,
    search_match_indexes: Option<Vec<usize>>,
}

impl App {
    pub fn new(playlist: Vec<PathBuf>) -> Self {
        Self {
            should_quit: false,
            is_playing: false,
            playlist,
            selected_song_index: 0,
            playing_song_index: None,
            current_pos: Duration::new(0, 0),
            total_duration: Duration::new(0, 0),
            search_mode: false,
            search_str: None,
            is_searching: false,
            search_match_indexes: None,
        }
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn playlist(&self) -> &[PathBuf] {
        &self.playlist
    }

    pub fn selected_song_index(&self) -> usize {
        self.selected_song_index
    }

    pub fn playing_song_index(&self) -> Option<usize> {
        self.playing_song_index
    }

    pub fn selected_song_path(&self) -> &PathBuf {
        &self.playlist()[self.selected_song_index]
    }

    pub fn playing_song_path(&self) -> Option<&PathBuf> {
        match self.playing_song_index {
            Some(playing_song_index) => Some(&self.playlist()[playing_song_index]),
            _ => None,
        }
    }

    pub fn current_pos(&self) -> Duration {
        self.current_pos
    }

    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }

    pub fn search_mode(&self) -> bool {
        self.search_mode
    }

    pub fn search_str(&self) -> &str {
        if let Some(search_str) = &self.search_str {
            search_str
        } else {
            ""
        }
    }

    pub fn update(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.should_quit = true;
            }

            Action::TogglePause => {
                self.is_playing = !self.is_playing;
            }

            Action::Next => {
                self.selected_song_index = if self.selected_song_index + 1 == self.playlist.len() {
                    0
                } else {
                    self.selected_song_index + 1
                }
            }

            Action::Prev => {
                self.selected_song_index = if self.selected_song_index == 0 {
                    self.playlist.len() - 1
                } else {
                    self.selected_song_index - 1
                }
            }

            Action::Play => {
                self.playing_song_index = Some(self.selected_song_index);
                self.is_playing = true;
            }

            Action::PlayNext => match self.playing_song_index {
                Some(playing_song_index) => {
                    self.playing_song_index = if playing_song_index + 1 == self.playlist.len() {
                        Some(0)
                    } else {
                        Some(playing_song_index + 1)
                    }
                }
                _ => {}
            },

            Action::UpdateCurrentPos(pos) => self.current_pos = pos,
            Action::UpdateTotalDuration(duration) => self.total_duration = duration,

            Action::StartSearch => {
                self.search_mode = true;
                self.search_str = None;
            }

            Action::StopSearch => {
                self.search_mode = false;
                self.is_searching = false;
                self.search_str = None;
            }

            Action::AppendCharForSearchStr(c) => {
                self.search_str = match self.search_str.take() {
                    Some(mut search_str) => {
                        search_str.push(c);
                        Some(search_str)
                    }
                    None => Some(c.to_string()),
                };
            }

            Action::DelCharFromSearchStr => {
                self.search_str = match self.search_str.take() {
                    Some(mut search_str) => {
                        search_str.pop();
                        if search_str.len() == 0 {
                            None
                        } else {
                            Some(search_str)
                        }
                    }
                    None => None,
                }
            }

            Action::Search => match &self.search_str {
                Some(search_str) => {
                    let searched_indexes: Vec<usize> = self
                        .playlist
                        .iter()
                        .enumerate()
                        .filter(|(_, path)| {
                            if let Some(path_str) = path.to_str() {
                                path_str.contains(search_str.as_str())
                            } else {
                                false
                            }
                        })
                        .map(|(i, _)| i)
                        .collect();
                    if searched_indexes.len() > 0 {
                        self.search_match_indexes = Some(searched_indexes);
                        self.is_searching = true;
                    } else {
                        self.search_match_indexes = None;
                        self.search_mode = false;
                    }
                    self.search_mode = false;
                }
                _ => {
                    self.search_match_indexes = None;
                    self.search_mode = false;
                    self.is_searching = false;
                }
            },

            Action::SearchNext(go_next_if_now_selected_search_result) => {
                if self.is_searching {
                    match &self.search_match_indexes {
                        Some(search_match_indexes) => {
                            let index = search_match_indexes
                                .iter()
                                .position(|x| x == &self.selected_song_index);
                            // TODO: We may NOT need this match
                            match index {
                                Some(index) => {
                                    if go_next_if_now_selected_search_result {
                                        if index + 1 == search_match_indexes.len() {
                                            self.selected_song_index = search_match_indexes[0];
                                        } else {
                                            self.selected_song_index =
                                                search_match_indexes[index + 1];
                                        }
                                    }
                                }
                                _ => {
                                    if let Some(index) = search_match_indexes
                                        .iter()
                                        .position(|x| x > &self.selected_song_index)
                                    {
                                        self.selected_song_index = search_match_indexes[index];
                                    } else {
                                        self.selected_song_index = search_match_indexes[0];
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            Action::SearchPrev => {
                if self.is_searching {
                    match &self.search_match_indexes {
                        Some(search_match_indexes) => {
                            let index = search_match_indexes
                                .iter()
                                .position(|x| x == &self.selected_song_index);
                            // TODO: We may NOT need this match
                            match index {
                                Some(index) => {
                                    if index == 0 {
                                        self.selected_song_index =
                                            search_match_indexes[search_match_indexes.len() - 1];
                                    } else {
                                        self.selected_song_index = search_match_indexes[index - 1];
                                    }
                                }
                                _ => {
                                    if let Some(index) = search_match_indexes
                                        .iter()
                                        .enumerate()
                                        .rev()
                                        .find(|(_, x)| x < &&self.selected_song_index)
                                        .map(|(index, _)| index)
                                    {
                                        self.selected_song_index = search_match_indexes[index];
                                    } else {
                                        self.selected_song_index =
                                            search_match_indexes[search_match_indexes.len() - 1];
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }
}
