use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use clap::Parser;

#[derive(Deserialize, Serialize, Debug)]
struct Bookmark {
    alias: String,
    path: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    bookmarks: Vec<Bookmark>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Creates a bookmark for the current directory.
    Add {
        /// The alias for the current directory.
        #[arg(short, long, required = true)]
        alias: String,
    },
    /// Removes a bookmark.
    Remove {
        /// The alias you want to remove.
        #[arg(short, long, required = true)]
        alias: String,
    },
    /// Lists all your bookmarks.
    List,
    /// Edits an existing bookmark.
    Edit {
        /// The alias of the bookmark you want to edit.
        #[arg(short, long, required = true)]
        alias: String,
        /// The new alias for the bookmark.
        #[arg(short, long, required = true)]
        new: String
    },
    /// Jumps to a bookmark (directory).
    Jump {
        /// The bookmark you want to jump to.
        #[arg(short, long, required = true)]
        alias: String
    }
}

// Attempts to load the configuration from a predefined path.
// If the configuration file exists and is valid, it reads the file and deserializes the TOML into a Config struct.
// If the file doesn't exist or an error occurs while reading, it returns a new Config struct with an empty bookmarks vector.
fn load_or_initialize_config() -> Config {
    let config_path = get_config_path();
    match fs::read_to_string(&config_path) {
        Ok(contents) => toml::from_str(&contents).unwrap(),
        Err(_) => Config { bookmarks: Vec::new() },
    }
}

// Takes a reference to a Config struct and serializes it into TOML format.
// It then writes this serialized TOML string to a file at the location specified by get_config_path.
// If the file doesn't exist, it creates a new one. If the directory doesn't exist, it creates a new directory.
// If any operation fails, the function panics with an appropriate message.
fn save_config(config: &Config) {
    let config_path = get_config_path();
    let config_dir = config_path.parent().expect("Failed to get config directory path");

    if !config_dir.exists() {
        fs::create_dir_all(config_dir).expect("Failed to create config directory");
    }

    let toml = toml::to_string(config).expect("Failed to serialize the config");
    let mut file = File::create(&config_path).expect("Failed to create config file");
    file.write_all(toml.as_bytes()).expect("Failed to write to config file");
}

// Constructs and returns the path to the configuration file.
// It determines the user's home directory using the dirs crate and appends the relative path to the 'config.toml' file within the '.pomelo' directory.
// This function panics if it fails to determine the home directory.
fn get_config_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Failed to find home directory");
    home_dir.join(".pomelo").join("config.toml")
}

fn main() {
    let cli = Cli::parse();

    let mut config = load_or_initialize_config();

    match &cli.command {
        Commands::Add { alias } => {
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let bookmark = Bookmark {
                alias: alias.clone(),
                path: current_dir,
            };
            config.bookmarks.push(bookmark);
            save_config(&config);
            println!("Added bookmark with alias '{}'", alias);
        }
        Commands::Remove { alias } => {
           if let Some(index) = config.bookmarks.iter().position(|bookmark| bookmark.alias == *alias) {
            config.bookmarks.remove(index);
            println!("Removed bookmark with alias '{}'", alias);
           } else {
            println!("No bookmark found with alias '{}'", alias);
           }

           save_config(&config)
        }
        Commands::Edit { alias, new } => {
            if let Some(bookmark) = config.bookmarks.iter_mut().find(|b| b.alias == *alias) {
                bookmark.alias = new.clone();
                println!("Updated alias '{}' to '{}'", alias, new);
            } else {
                println!("No bookmark found with alias '{}'", alias);
            }
        
            save_config(&config);
        }
        Commands::List => {
            if config.bookmarks.is_empty() {
                println!("You have no bookmarks.");
            } else {
                println!("Your bookmarks:");
                for (index, bookmark) in config.bookmarks.iter().enumerate() {
                    println!("{}. Alias: '{}', Path: '{}'", index + 1, bookmark.alias, bookmark.path.display());
                }
            }
        }
        Commands::Jump { alias } => {
            // :)
        }
    }
}
