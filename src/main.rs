use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    run,
    style::{Color, Style},
    widgets::{Block, List, ListItem, ListState},
};
use rodio::{self, Decoder, Player, Source};
use std::{
    fs::{self, File},
    io::{self},
    path::PathBuf,
    time::Duration,
};

mod action;
use crate::action::{Action, map_key};
mod app;
use crate::app::App;

fn main() -> io::Result<()> {
    let playlist: Vec<PathBuf> = fs::read_dir("./musics")?
        .flat_map(|res| res.map(|e| e.path()).ok())
        .collect();

    let mut app = App::new(playlist);

    let device = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = Player::connect_new(&device.mixer());

    run(|terminal| {
        loop {
            terminal.draw(|frame| draw(frame, &app))?;

            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        do_action(map_key(key), &mut app, &player)
                    }
                    _ => {}
                }
            }

            if player.empty() {
                do_action(Some(Action::PlayNext), &mut app, &player);
            } else {
                do_action(
                    Some(Action::UpdateCurrentPos(player.get_pos())),
                    &mut app,
                    &player,
                );
            }

            if app.should_quit() {
                break Ok(());
            }
        }
    })
}

fn draw(frame: &mut Frame, app: &App) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 1]);
    let [left_area] = horizontal.areas(main_area);

    let playlist = app
        .playlist()
        .iter()
        .filter_map(|path| path.file_name())
        .enumerate()
        .map(|(index, name)| {
            let name = name.to_string_lossy();

            let item = ListItem::new(name);

            let mut style = Style::default();

            if Some(index) == app.playing_song_index() {
                style = style.fg(Color::Green);
            }

            item.style(style)
        })
        .collect::<Vec<_>>();

    let playlist_widget = List::new(playlist)
        .block(Block::bordered().title("Playlist"))
        .highlight_style(Style::default().bg(Color::Cyan));

    let mut playlist_state = ListState::default();

    playlist_state.select(Some(app.selected_song_index()));

    let (total_secs, current_secs) = (app.total_duration().as_secs(), app.current_pos().as_secs());

    let status = if app.is_playing() { "▶️" } else { "⏯️" };

    let status = format!(
        "{}   {:02}:{:02}/{:02}:{:02}",
        status,
        current_secs / 60,
        current_secs % 60,
        total_secs / 60,
        total_secs % 60
    );

    frame.render_widget(Block::new().title("Aki Music Player🦄"), title_area);
    frame.render_widget(Block::new().title(status), status_area);
    frame.render_stateful_widget(playlist_widget, left_area, &mut playlist_state);
}

fn do_action(action: Option<Action>, app: &mut App, player: &Player) {
    match action {
        Some(Action::Play) => {
            player.clear();
            let file = File::open(app.selected_song_path()).unwrap();
            let source = Decoder::try_from(file).unwrap();
            match source.total_duration() {
                Some(total_duration) => app.update(Action::UpdateTotalDuration(total_duration)),
                _ => {}
            }

            player.append(source);
            player.play();
            app.update(Action::Play);
        }
        Some(Action::TogglePause) => {
            if app.is_playing() {
                player.pause();
            } else {
                player.play();
            }
            app.update(Action::TogglePause);
        }
        Some(Action::FastForward) => {
            player
                .try_seek(player.get_pos().saturating_add(Duration::from_secs(5)))
                .unwrap();
        }
        Some(Action::Rewind) => {
            player
                .try_seek(player.get_pos().saturating_sub(Duration::from_secs(5)))
                .unwrap();
        }
        Some(Action::PlayNext) => {
            if app.is_playing() {
                app.update(Action::PlayNext);
                match app.playing_song_path() {
                    Some(path) => {
                        let file = File::open(path).unwrap();
                        let source = Decoder::try_from(file).unwrap();
                        match source.total_duration() {
                            Some(total_duration) => {
                                app.update(Action::UpdateTotalDuration(total_duration))
                            }
                            _ => {}
                        }

                        player.append(source);
                        player.play();
                    }
                    _ => {}
                }
            }
        }
        Some(action) => {
            app.update(action);
        }
        _ => {}
    }
}
