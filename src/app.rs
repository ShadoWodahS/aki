use std::{path::PathBuf};

use crate::action::Action;

pub struct App {
    should_quit: bool,
    is_playing: bool,
    playlist: Vec<PathBuf>,
    selected_song_index: usize,
    playing_song_index: Option<usize>,
}

impl App {
    pub fn new(playlist: Vec<PathBuf>) -> Self {
        Self {
            should_quit: false,
            is_playing: false,
            playlist,
            selected_song_index: 0,
            playing_song_index: None,
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

    pub fn update(&mut self, action: Action) {
        match action {
            Action::Quit => {
                self.should_quit = true;
            }

            Action::TogglePause => {
                self.is_playing = !self.is_playing;
            }

            Action::Next => {
                self.selected_song_index = if self.selected_song_index() + 1 == self.playlist.len() {
                    0
                } else {
                    self.selected_song_index + 1
                }
            }

            Action::Prev => {
                self.selected_song_index = if self.selected_song_index() == 0 {
                    self.playlist.len() - 1
                } else {
                    self.selected_song_index - 1
                }
            }

            Action::Play => {
                self.playing_song_index = Some(self.selected_song_index);
                self.is_playing = true;
            }
        }
    }
}
