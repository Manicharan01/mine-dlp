use dirs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to get home directory");
            return;
        }
    };

    let file_path = home_dir.join(".mine-dlp");

    let mut downloads_folder_path: String = String::new();
    let mut browser_for_cookies: String = String::new();
    if !Path::new(&file_path).exists() {
        print!("Enter the folder path where the downloaded files should be stored: ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut downloads_folder_path)
            .expect("Failed to read line");

        print!("Enter the name of the browser, in which your YouTube is logged in : ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut browser_for_cookies)
            .expect("Failed to read line");

        let downloads_folder = format!("downloads={}", downloads_folder_path.trim());
        let cookie_browser: String = format!("browser={}", browser_for_cookies.trim());

        let file = write_to_file(&cookie_browser, &downloads_folder);
        match file {
            Ok(_) => println!("File created successfully"),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        let path = Path::new(&file_path);
        let contents = std::fs::read_to_string(&path).expect("Error reading file");

        let confs: Vec<&str> = contents.split("\n").collect();
        if confs.is_empty() {
            eprintln!("Configuration file is empty");
            return;
        }

        for conf in confs {
            let key_val: Vec<&str> = conf.split("=").collect();
            if key_val[0] == "downloads" {
                downloads_folder_path = key_val[1].to_string();
            } else if key_val[0] == "browser" {
                browser_for_cookies = key_val[1].to_string();
            }
        }
        if downloads_folder_path.is_empty() || browser_for_cookies.is_empty() {
            println!("The parameters in the configuration file are empty");
            print!("Enter the folder path where the downloaded files should be stored: ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut downloads_folder_path)
                .expect("Failed to read line");

            print!("Enter the name of the browser, in which your YouTube is logged in : ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut browser_for_cookies)
                .expect("Failed to read line");

            let downloads_folder = format!("downloads={}", downloads_folder_path.trim());
            let cookie_browser: String = format!("browser={}", browser_for_cookies.trim());

            let file = write_to_file(&cookie_browser, &downloads_folder);
            match file {
                Ok(_) => println!("File created successfully"),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

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

    println!("Do you want to download the video or audio?");
    println!("1. Video");
    println!("2. Audio");
    println!("Enter a choice according to your preference: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice = choice.trim();

    if choice == "Video" || choice == "video" || choice == "1" {
        let _ = video_and_audio(video_link, &downloads_folder_path, &browser_for_cookies);
    } else if choice == "Audio" || choice == "audio" || choice == "2" {
        let _output = only_audio(video_link, &downloads_folder_path, &browser_for_cookies);
    } else {
        eprintln!("Invalid choice");
        return;
    }
}

pub fn write_to_file(browser: &str, downloads: &str) -> std::io::Result<()> {
    let home_path = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Failed to get home directory");
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get home directory",
            ));
        }
    };
    let file_path = home_path.join(".mine-dlp");
    let path = Path::new(&file_path);
    let mut file = File::create(&path)?;
    let contents = format!("{}\n{}", browser, downloads);
    writeln!(file, "{}", contents)?;
    print!("File created successfully");
    Ok(())
}

pub fn video_and_audio(link: &str, downloads_folder_path: &str, browser_for_cookies: &str) {
    let mut child = Command::new("yt-dlp")
        //.arg("-k") // Keep video files
        .arg("-P")
        .arg(downloads_folder_path) // Output directory
        .arg("-o")
        .arg("%(title)s_%(vcodec)s_%(dynamic_range)s_%(format)s_%(resolution)s.%(ext)s") // Output template
        .arg("--cookies-from-browser")
        .arg(browser_for_cookies.trim()) // Use cookies from the browser
        .arg(link) // URL to download
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute yt-dlp");

    let output = BufReader::new(child.stdout.take().expect("Failed to get stdout"));
    for line in output.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    let status = child.wait().expect("Failed to wait for child");
    if !status.success() {
        eprintln!("Error: yt-dlp exited with status {}", status);
    }
}

pub fn only_audio(link: &str, downloads_folder_path: &str, browser_for_cookies: &str) {
    let mut child = Command::new("yt-dlp")
        .arg("-x") // Extract audio
        .arg("--audio-format")
        .arg("mp3") // Audio format
        .arg("-P")
        .arg(downloads_folder_path) // Output directory
        .arg("-o")
        .arg("%(title)s_%(acodec)s_%(format)s.%(ext)s") // Output template
        .arg("--cookies-from-browser")
        .arg(browser_for_cookies.trim()) // Use cookies from the browser
        .arg(link) // URL to download
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute yt-dlp");

    let output = BufReader::new(child.stdout.take().expect("Failed to get stdout"));
    for line in output.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    let status = child.wait().expect("Failed to wait for child");
    if !status.success() {
        eprintln!("Error: yt-dlp exited with status {}", status);
    }
}
