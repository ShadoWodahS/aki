use rodio;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

fn main() -> io::Result<()> {
    let entries: Vec<PathBuf> = fs::read_dir("./musics")?
        .flat_map(|res| res.map(|e| e.path()).ok())
        .collect();

    println!("--- Pick a music --");
    for (index, path) in entries.iter().enumerate() {
        if let Some(file_name) = path.file_name() {
            println!("[{}] {:?}", index + 1, file_name);
        }
    }

    print!("Please input index: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let choice: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= entries.len() => num - 1,
        _ => {
            println!("invalid input");
            return Ok(());
        }
    };

    let selected_path = &entries[choice];

    println!("selected path: {}", selected_path.display());

    let file = File::open(selected_path).unwrap();
    let handler = rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
    let player = rodio::play(&handler.mixer(), file).unwrap();

    player.sleep_until_end();

    Ok(())
}
