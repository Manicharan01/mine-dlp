use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};

fn run_yt_dlp_command(args: &[&str]) -> io::Result<()> {
    let mut child = Command::new("yt-dlp")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            eprintln!("{}", line?);
        }
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("yt-dlp exited with status {}", status),
        ));
    }

    Ok(())
}

pub fn download_video(
    link: &str,
    downloads_folder_path: &str,
    browser_for_cookies: &str,
) -> io::Result<()> {
    println!("Downloading video...");
    run_yt_dlp_command(&[
        "-P",
        downloads_folder_path,
        "-o",
        "%(title)s_%(vcodec)s_%(dynamic_range)s_%(format)s_%(resolution)s.%(ext)s",
        "--cookies-from-browser",
        browser_for_cookies.trim(),
        link,
    ])
}

pub fn download_audio(
    link: &str,
    downloads_folder_path: &str,
    browser_for_cookies: &str,
) -> io::Result<()> {
    println!("Downloading audio...");
    run_yt_dlp_command(&[
        "-x",
        "--audio-format",
        "mp3",
        "-P",
        downloads_folder_path,
        "-o",
        "%(title)s_%(acodec)s_%(format)s.%(ext)s",
        "--cookies-from-browser",
        browser_for_cookies.trim(),
        link,
    ])
}
