use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    run,
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph},
};
use rodio::{self, Decoder, Player, Source};
use std::{
    fs::{self, File},
    io::{self},
    path::PathBuf,
    time::Duration,
    env,
};

mod action;
use crate::action::{
    Action::{self},
    map_key, map_key_in_search_mode,
};
mod app;
use crate::app::{App, PlayMode};

fn get_playlist() -> io::Result<Vec<PathBuf>> {
    let args: Vec<String> = env::args().collect();

    let musics_dir = args.get(1).map(String::as_str).unwrap_or("./musics");

    let audio_extensions = [
        "mp3", "wav", "flac", "aac", "ogg", 
        "m4a", "wma", "alac", "ape", "opus"
    ];

    let playlist: Vec<PathBuf> = fs::read_dir(musics_dir)?
        .filter_map(|res| res.ok().map(|e| e.path()))
        .filter(|path| {
            if !path.is_file() {
                return false;
            }

            if let Some(ext) = path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    let ext_lower = ext_str.to_lowercase();
                    return audio_extensions.contains(&ext_lower.as_str());
                }
            }
            false
        })
        .collect();
    Ok(playlist)
}

fn main() -> io::Result<()> {
    let mut app = App::new(get_playlist()?);

    let device = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = Player::connect_new(device.mixer());

    run(|terminal| {
        loop {
            terminal.draw(|frame| draw(frame, &app))?;

            if event::poll(Duration::from_millis(100))? {
                match event::read()? {
                    Event::Key(key) if key.kind == KeyEventKind::Press => {
                        do_action_by_key(key, &mut app, &player)
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

    let vertical = Layout::vertical([Length(1), Min(0), Length(3), Length(1)]);
    let [title_area, main_area, status_area, search_area] = vertical.areas(frame.area());
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
        .highlight_style(Style::default().bg(Color::DarkGray));

    let playlist_area_height = (left_area.height - 2) as usize;

    let mut playlist_state = ListState::default();

    playlist_state.select(Some(app.selected_song_index()));
    *playlist_state.offset_mut() = (app.selected_song_index().max(playlist_area_height / 2) - playlist_area_height / 2).min(app.playlist().len() - playlist_area_height);

    let (total_secs, current_secs) = (app.total_duration().as_secs(), app.current_pos().as_secs());

    let status = if app.is_playing() { "▶️" } else { "⏯️" };

    let play_mode = match app.play_mode() {
        PlayMode::RepeatAll => "🔁",
        PlayMode::ShuffleAll => "🔀",
        PlayMode::RepeatOne => "🔂",
    };
    
    let status = format!(
        "{} {} {:02}:{:02}/{:02}:{:02}",
        status,
        play_mode,
        current_secs / 60,
        current_secs % 60,
        total_secs / 60,
        total_secs % 60
    );

    let gauge = Gauge::default()
        .block(Block::new().title(status).borders(Borders::ALL))
        .gauge_style(Style::new().white().on_black().italic())
        .percent(
            current_secs
                .checked_mul(100)
                .and_then(|f| f.checked_div(total_secs))
                .map(|v| v as u16)
                .unwrap_or(0)
                .clamp(0, 100)

        );

    frame.render_widget(Block::new().title("Aki Music Player🦄"), title_area);
    frame.render_widget(gauge, status_area);
    frame.render_stateful_widget(playlist_widget, left_area, &mut playlist_state);
    if app.search_mode() {
        let search_box = Paragraph::new(format!("Search: {}", app.search_str())).block(
            Block::default()
        );
        frame.render_widget(search_box, search_area);
    }
}

fn do_action_by_key(key: KeyEvent, app: &mut App, player: &Player) {
    let action = if app.search_mode() {
        map_key_in_search_mode(key)
    } else {
        map_key(key)
    };
    do_action(action, app, player);
}

fn do_action(action: Option<Action>, app: &mut App, player: &Player) {
    if app.search_mode() {
        match action {
            Some(Action::Search) => {
                app.update(Action::Search);
                app.update(Action::SearchNext(false));
            }
            Some(Action::SearchNext(_)) => {
                app.update(Action::SearchNext(true));
            }
            Some(action) => app.update(action),
            _ => {}
        }
    } else {
        match action {
            Some(Action::Play) => {
                player.clear();
                let file = File::open(app.selected_song_path()).unwrap();
                let source = Decoder::try_from(file).unwrap();
                if let Some(total_duration) = source.total_duration() {
                    app.update(Action::UpdateTotalDuration(total_duration));
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
                    if let Some(path) = app.playing_song_path() {
                        let file = File::open(path).unwrap();
                        let source = Decoder::try_from(file).unwrap();
                        if let Some(total_duration) = source.total_duration() {
                            app.update(Action::UpdateTotalDuration(total_duration));
                        }

                        player.append(source);
                        player.play();
                    }
                }
            }
            Some(Action::Delete) => {
                if app.playing_song_index() == Some(app.selected_song_index()) {
                    player.clear();
                }
                fs::remove_file(app.selected_song_path()).unwrap();
                app.update(Action::Delete);
                app.refresh_playlist(get_playlist().unwrap());
            }
            Some(Action::RefreshPlaylist) => {
                app.refresh_playlist(get_playlist().unwrap());
            }
            Some(action) => {
                app.update(action);
            }
            _ => {}
        }
    }
}
