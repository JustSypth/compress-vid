use std::path::Path;

use crate::ConfigInit;

pub fn compress(config: &mut ConfigInit, flag: &str, value: Option<&String>) {
    match value {
        Some(value) => {
            if let Ok(num) = value.parse::<u32>() {
                if (1..=51).contains(&num) {
                    config.c = num.to_string();
                } else {
                    eprintln!("Error: Compress value must be between 1-51!");
                    std::process::exit(1);
                }
            } else {
                eprintln!("Error: Compress value must be a number (1-51)!");
                std::process::exit(1);
            }
        },

        None => {
            eprintln!("Missing argument after {}", flag);
            std::process::exit(1);
        }
    }
}

pub fn preset(config: &mut ConfigInit, flag: &str, value: Option<&String>) {
    match value {
        Some(value) => {
            let allowed_strings: [&str; 5] = ["ultrafast", "veryfast", "medium", "slow", "veryslow"];
            if allowed_strings.contains(&value.as_str()) {
                config.p = value.to_string();
            } else {
                eprintln!(
                    "Error: Invalid preset \nList of valid presets: {}",
                    allowed_strings.join(", ")
                );
                std::process::exit(1);
            }
        },

        None => {
            eprintln!("Missing argument after {}", flag);
            std::process::exit(1);
        }
    }
}

pub fn is_video(path_str: &str) -> bool {
    let video_extensions: [&str; 7] = ["mp4", "avi", "mkv", "mov", "flv", "wmv", "webm"];
    let path = Path::new(path_str);

    if path.exists() && path.is_file() {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) { //whether the file even has an extension
            return video_extensions.contains(&extension.to_lowercase().as_str()); //whether the file has a vid extension
        }
    }

    false
}
