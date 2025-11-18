use dirs;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

/// Holds the configuration for the application.
#[derive(Debug, Clone)]
pub struct Config {
    /// The path to the folder where downloads will be saved.
    pub downloads_folder_path: String,
    /// The name of the browser to use for cookies.
    pub browser_for_cookies: String,
}

impl Config {
    /// Creates a new, empty `Config` instance.
    pub fn new() -> Self {
        Config {
            downloads_folder_path: String::new(),
            browser_for_cookies: String::new(),
        }
    }
}

fn get_config_file_path() -> Result<PathBuf, io::Error> {
    match dirs::home_dir() {
        Some(path) => Ok(path.join(".mine-dlp")),
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to get home directory",
        )),
    }
}

/// Loads the configuration from the `.mine-dlp` file in the home directory.
///
/// If the file does not exist, it prompts the user for the configuration values
/// and saves them to the file.
pub fn load_config() -> Result<Config, io::Error> {
    let file_path = get_config_file_path()?;
    if !file_path.exists() {
        return prompt_and_save_config();
    }

    let contents = std::fs::read_to_string(&file_path)?;
    let mut config = Config::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();
            match key {
                "downloads" => config.downloads_folder_path = value.to_string(),
                "browser" => config.browser_for_cookies = value.to_string(),
                _ => (),
            }
        }
    }

    if config.downloads_folder_path.is_empty() || config.browser_for_cookies.is_empty() {
        println!("The parameters in the configuration file are empty or incomplete.");
        return prompt_and_save_config();
    }

    Ok(config)
}


fn prompt_for_config_values() -> Result<Config, io::Error> {
    let mut config = Config::new();

    print!("Enter the folder path where the downloaded files should be stored: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut config.downloads_folder_path)?;

    print!("Enter the name of the browser, in which your YouTube is logged in : ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut config.browser_for_cookies)?;

    config.downloads_folder_path = config.downloads_folder_path.trim().to_string();
    config.browser_for_cookies = config.browser_for_cookies.trim().to_string();

    Ok(config)
}


/// Prompts the user for configuration values and saves them to the `.mine-dlp` file.
pub fn prompt_and_save_config() -> Result<Config, io::Error> {
    let config = prompt_for_config_values()?;
    write_config_to_file(&config)?;
    println!("Configuration saved successfully.");
    Ok(config)
}

fn write_config_to_file(config: &Config) -> io::Result<()> {
    let file_path = get_config_file_path()?;
    let mut file = File::create(file_path)?;

    writeln!(file, "browser={}", config.browser_for_cookies)?;
    writeln!(file, "downloads={}", config.downloads_folder_path)?;

    Ok(())
}
