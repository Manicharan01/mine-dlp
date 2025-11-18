use mine_dlp::{config, downloader};
use std::io::{self, Write};
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config()?;

    print!("Enter the YouTube video link: ");
    io::stdout().flush()?;
    let mut video_link = String::new();
    io::stdin().read_line(&mut video_link)?;

    let video_link = video_link.trim();
    if video_link.is_empty() {
        return Err("Video link cannot be empty.".into());
    }

    println!("Do you want to download the video or audio?");
    println!("1. Video");
    println!("2. Audio");
    print!("Enter a choice according to your preference: ");
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    match choice.trim() {
        "1" | "Video" | "video" => downloader::download_video(
            video_link,
            &config.downloads_folder_path,
            &config.browser_for_cookies,
        )?,
        "2" | "Audio" | "audio" => downloader::download_audio(
            video_link,
            &config.downloads_folder_path,
            &config.browser_for_cookies,
        )?,
        _ => {
            return Err("Invalid choice".into());
        }
    }

    Ok(())
}
