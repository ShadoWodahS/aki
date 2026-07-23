use std::{path::PathBuf, time::Duration};

use crate::action::Action;

pub struct App {
    should_quit: bool,
    is_playing: bool,
    playlist: Vec<PathBuf>,
    selected_song_index: usize,
    playing_song_index: Option<usize>,
    current_pos: Duration,
    total_duration: Duration,
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

            _ => {}
        }
    }
}
