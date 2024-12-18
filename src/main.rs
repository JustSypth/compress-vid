use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

struct ConfigInit {
    c: String,
    p: String,
    video: PathBuf
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
            "-c" | "-p" => {handle_args(&mut config, &flag, args_iter.next() );},
            _ => {
                match is_video(flag) {
                    Some(var) => config.video = var,
                    _ => {
                        eprintln!("Error: Invalid path or not a video \"{}\"", flag);
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    println!("\nConfig: C: {}, P: {} \nVideo: {}", config.c, config.p, config.video.display());

    // Add "-output" to output filename
    let output_path = config.video.with_file_name(format!(
        "{}-output.{}",
        config.video.file_stem().unwrap().to_str().unwrap(),
        config.video.extension().unwrap().to_str().unwrap()
    ));

    let execute = {
        Command::new("sh")
            .arg("-c")
            .arg(format!(
                "ffmpeg -i \"{}\" -vcodec libx264 -crf {} -preset {} -acodec aac -b:a 128k -y \"{}\"",
                &config.video.display(), &config.c, &config.p, &output_path.display()
            ))
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
    //Checks if there's a value after a flag
    match args_iter {
        //If a flag has value
        Some(value) => {

            let path = Path::new(value);
            // If value isn't a file
            if !path.exists() || !path.is_file() {
                match flag {
                    "-c" => {
                        config.c = value.to_string();
                    },
                    "-p" => {
                        config.p = value.to_string();
                    },
                    _ => (),
                }
            }
            // If value is a file
            else {
                eprintln!("Argument after {} should not be a file!", flag);
                std::process::exit(1);
            }
        }
        // If a flag doesn't have a value
        None => {
            eprintln!("Missing argument after {}", flag);
            std::process::exit(1);
        },
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
