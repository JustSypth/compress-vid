use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

struct ConfigInit {
    c: String,
    p: String,
    video: PathBuf,
}

fn main() {
    // Default values for the command
    let mut config = ConfigInit {
        c: String::from("32"),
        p: String::from("medium"),
        video: PathBuf::from("INVALID"),
    };

    let args: Vec<String> = env::args().collect();
    let mut args_iter = args.iter().skip(1);

    // Loop through args and check if it's a flag or vid
    while let Some(flag) = args_iter.next() {
        match flag.to_lowercase().as_str() {
            "-c" | "-p" => {
                handle_args(&mut config, &flag, args_iter.next());
            }
            _ => match is_video(flag) {
                Some(var) => config.video = var,
                _ => {
                    eprintln!("Error: Invalid path or not a video \"{}\"", flag);
                    std::process::exit(1);
                }
            },
        }
    }

    println!(
        "\nConfig: C: {}, P: {} \nVideo: {}",
        config.c,
        config.p,
        config.video.display()
    );

    // Add "-output" to output filename
    let output_path = config.video.with_file_name(format!(
        "{}-output.{}",
        config.video.file_stem().unwrap().to_str().unwrap(),
        config.video.extension().unwrap().to_str().unwrap()
    ));

    let execute_arg = format!(
        "ffmpeg -i '{}' -vcodec libx264 -crf {} -preset {} -acodec aac -b:a 128k -y '{}'",
        &config.video.display(),
        &config.c,
        &config.p,
        &output_path.display()
    );

    let execute = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(&execute_arg)
            .output()
            .expect("Failed to execute ffmpeg.")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&execute_arg)
            .output()
            .expect("Failed to execute ffmpeg.")
    };

    let output = format!(
        "Standard Output:\n{}\nStandard Error:\n{}",
        String::from_utf8_lossy(&execute.stdout),
        String::from_utf8_lossy(&execute.stderr)
    );

    println!("Execution: \n{}", output);
}

fn handle_args(config: &mut ConfigInit, flag: &str, args_iter: Option<&String>) {
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
                let allowed_strings: [&str; 4] = ["ultrafast", "veryfast", "slow", "veryslow"];
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

fn is_video(path_str: &str) -> Option<PathBuf> {
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
