use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{Frame, layout::{Constraint, Layout}, run, style::{Color, Modifier, Style}, widgets::{Block, List, ListItem, Paragraph}};
use rodio::{self, Decoder, Player};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
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

    run(|mut terminal| {
        loop {
            terminal.draw(|frame| draw(frame, &app))?;
            match event::read()? {
              Event::Key(key) if key.kind == KeyEventKind::Press => do_action(map_key(key), &mut app, &player),
              _ => {}
            }
            if app.should_quit() {
                break Ok(())
            }
        }
    })

    // let selected_path = &entries[choice];

    // println!("selected path: {}", selected_path.display());

    // let file = File::open(selected_path).unwrap();

    // player.sleep_until_end();

    // Ok(())
}

fn draw(frame: &mut Frame, app: &App) {
  use Constraint::{Fill, Length, Min};

  let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
  let [title_area, main_area, status_area] = vertical.areas(frame.area());
  let horizontal = Layout::horizontal([Fill(1); 1]);
  let [left_area] = horizontal.areas(main_area);

  let status = if app.is_playing() {
      "playing"
  } else {
      "paused"
  };

  let playlist = app.playlist()
    .iter()
    .filter_map(|path| path.file_name())
    .enumerate()
    .map(|(index, name)| {
        let name = name.to_string_lossy();

        let item = ListItem::new(name);

        let mut style = Style::default();

        if index == app.selected_song_index() {
            style = style.bg(Color::LightBlue);
        }

        if Some(index) == app.playing_song_index() {
            style = style.fg(Color::Green);
        }

        item.style(style)
    })
    .collect::<Vec<_>>();

  frame.render_widget(Block::bordered().title("Aki Music Player"), title_area);
  frame.render_widget(Block::bordered().title(status), status_area);
  frame.render_widget(
       List::new(playlist)
           .block(Block::bordered().title("Playlist")),
       left_area,
   );
}

fn do_action(action: Option<Action>, app: &mut App, player: &Player) {
    match action {
        Some(Action::Play) => {
            player.clear();
            let file = File::open(app.selected_song_path()).unwrap();
            let source = Decoder::try_from(file).unwrap();

            player.append(source);
            player.play();
            app.update(Action::Play);
        },
        Some(Action::TogglePause) => {
            if app.is_playing() {
                player.pause();
            } else {
                player.play();
            }
            app.update(Action::TogglePause);
        }
        Some(action) => {
           app.update(action);
        }
        _ => {}
    }
}
