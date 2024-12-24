mod execute;
mod args;

use std::env;
use std::path::PathBuf;

pub struct ConfigInit {
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
                args::handle_args(&mut config, &flag, args_iter.next());
            }
            _ => match args::is_video(flag) {
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

    let output = execute::execute(&config);
    
    println!("Execution: \n{}", output);
}


