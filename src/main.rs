mod args;
mod execute;

use std::env;
use std::path::PathBuf;

pub struct ConfigInit {
    c: String,
    p: String,
    video: PathBuf,
}

impl ConfigInit {
    fn has_video(&self) -> bool {
        args::is_video(self.video.to_str().unwrap_or(""))
    }
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
            _ => {
                if args::is_video(flag) {
                    config.video = PathBuf::from(flag);
                } else {
                    eprintln!("Error: Invalid path or not a video \"{}\"", flag);
                    std::process::exit(1);
                }
            }
        }
    }

    // DEBUG
    println!(
        "\nConfig: C: {}, P: {} \nVideo: {}\n",
        config.c,
        config.p,
        config.video.display()
    );

    if config.has_video() {
        let output = execute::execute(&config);
        println!("{output}");
    }
    else {
        eprintln!("Error: Missing a video");
        std::process::exit(1);
    }

    
}
