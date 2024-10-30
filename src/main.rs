use std::io::{self, Write};
use std::process::Command;

fn main() {
    // Prompt the user for the video link
    print!("Enter the YouTube video link: ");
    io::stdout().flush().expect("Failed to flush stdout");

    // Read the input from the user
    let mut video_link = String::new();
    io::stdin()
        .read_line(&mut video_link)
        .expect("Failed to read line");

    // Trim whitespace from the input
    let video_link = video_link.trim();

    // Execute yt-dlp with the user-provided link
    let output = Command::new("yt-dlp")
        .arg("-k") // Keep video files
        .arg("-P")
        .arg("~/Videos/YouTube") // Output directory
        .arg("-o")
        .arg("%(title)s_%(vcodec)s_%(dynamic_range)s_%(format)s_%(resolution)s.%(ext)s") // Output template
        .arg("--cookies-from-browser")
        .arg("chromium") // Use cookies from the Chromium browser
        .arg(video_link) // URL to download
        .output() // Execute the command
        .expect("Failed to execute yt-dlp");

    // Handle the command output
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error:\n{}", stderr);
    }
}
