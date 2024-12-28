mod args;
mod cli;
mod execute;

use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ConfigInit {
    c: String,
    p: String,
    video: PathBuf,
    debug: bool,
}

impl ConfigInit {
    fn new() -> Self {
        Self {
            c: String::from("32"),
            p: String::from("medium"),
            video: PathBuf::from("INVALID"),
            debug: false,
        }
    }

    fn has_video(&self) -> bool {
        args::is_video(self.video.to_str().unwrap_or(""))
    }
}

fn main() {
    let mut config = ConfigInit::new();

    handle_args(&mut config);

    if !config.has_video() {
        eprintln!("Error: Missing a video");
        std::process::exit(1);
    }

    if config.debug {
        cli::debug(&config);
        let output = execute::debug(&config);
        println!("{output}");
    } else {
        let output = execute::execute(&config);
        println!("{output}");
    }
    
}

fn handle_args(config: &mut ConfigInit) {
    let args: Vec<String> = env::args().collect();
    let mut args_iter = args.iter().skip(1);

    // Loop through args and check if it's a flag or vid
    while let Some(flag) = args_iter.next() {
        match flag.to_lowercase().as_str() {
            "--help" | "-h" => cli::help(),
            "--version" => cli::version(),
            "--debug" => config.debug = true,
            "-c" | "--crf" => args::compress(config, flag, args_iter.next()),
            "-p" | "--preset" => args::preset(config, flag, args_iter.next()),
            _ => {
                if args::is_video(flag) {
                    config.video = PathBuf::from(flag);
                } else if flag.starts_with("-") {
                    eprintln!("Error: Invalid flag \"{}\"", flag);
                    std::process::exit(1);
                } else {
                    eprintln!("Error: Invalid path or not a video \"{}\"", flag);
                    std::process::exit(1);
                }
            }
        }
    }
}
