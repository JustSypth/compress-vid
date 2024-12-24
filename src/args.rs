use std::path::{Path, PathBuf};

use crate::ConfigInit;

pub fn handle_args(config: &mut ConfigInit, flag: &str, args_iter: Option<&String>) {
    match args_iter {
        Some(value) => match flag {
            "-c" => {
                if let Ok(num) = value.parse::<u32>() {
                    if (1..=100).contains(&num) {
                        config.c = num.to_string();
                    } else {
                        eprintln!("Error: Compress value must be between 1-100!");
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("Error: Compress value must be a number (1-100)!");
                    std::process::exit(1);
                }
            }
            "-p" => {
                let allowed_strings: [&str; 5] = ["ultrafast", "veryfast", "medium", "slow", "veryslow"];
                if allowed_strings.contains(&value.as_str()) {
                    config.p = value.to_string();
                } else {
                    eprintln!(
                        "Error: Invalid preset (-p) \nList of valid presets: {}",
                        allowed_strings.join(", ")
                    );
                    std::process::exit(1);
                }
            }
            _ => (),
        },

        None => {
            eprintln!("Missing argument after {}", flag);
            std::process::exit(1);
        }
    }
}

pub fn is_video(path_str: &str) -> Option<PathBuf> {
    let video_extensions: [&str; 7] = ["mp4", "avi", "mkv", "mov", "flv", "wmv", "webm"];
    let path = Path::new(path_str);

    if path.exists() && path.is_file() {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            if video_extensions.contains(&extension.to_lowercase().as_str()) {
                return Some(path.to_path_buf());
            }
        }
    }

    None
}