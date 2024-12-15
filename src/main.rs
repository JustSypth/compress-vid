use std::env;
use std::path::Path;
use std::process::Command;

struct ConfigInit {
    c: String,
    p: String,
    video: String
}

fn main() {
    // Default values for the command
    let mut config = ConfigInit {
        c: String::from("28"),
        p: String::from("medium"),
        video: String::from("INVALID"),
    };

    let args: Vec<String> = env::args().collect();
    let mut args_iter = args.iter().skip(1);

    // Loop through args and check if it's a flag or vid
    while let Some(flag) = args_iter.next() {
        match flag.to_lowercase().as_str() {
            "-c" | "-p" => {handle_args(&mut config, &flag, args_iter.next() );},
            _ => {
                if is_video(flag) {
                    config.video = flag.to_string();
                } else {
                    println!("Error: Invalid path or not a video \"{}\"", flag);
                }
            }
        }
    }

    println!("\nConfig: C: {}, P: {} \nVideo: {}", config.c, config.p, config.video);

    let execute = {
        Command::new("sh")
            .arg("-c")
            .arg(format!("ffmpeg -i \"{}\" -vcodec libx264 -crf {} -preset {} -acodec aac -b:a 128k {}-output.mp4", &config.video, &config.c, &config.p, &config.video))
            .output()
            .expect("Failed to execute ffmpeg.")
    };

    let output = format!(
        "Standard Output:\n{}\nStandard Error:\n{}",
        String::from_utf8_lossy(&execute.stdout),
        String::from_utf8_lossy(&execute.stderr)
    );

    println!("Execution: \n{}", output);

    // let command = String::from("ffmpeg -i input.mp4 -vcodec libx264 -crf 23 -preset medium output.mp4");
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
            }
        }
        // If a flag doesn't have a value
        None => eprintln!("Missing argument after {}", flag),
    }
}

fn is_video(path_str: &str) -> bool {
    let video_extensions: [&str; 7] = ["mp4", "avi", "mkv", "mov", "flv", "wmv", "webm"];
    let path = Path::new(path_str);
    let extension = path.extension().and_then(|ext| ext.to_str());

    if !path.exists() || !path.is_file() {
        return false;
    }

    match extension {
        Some(file_ext) => video_extensions.contains(&file_ext.to_lowercase().as_str()),
        None => false,
    }
}