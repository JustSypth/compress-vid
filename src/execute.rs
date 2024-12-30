use std::path::PathBuf;
use std::process::Command;

use crate::ConfigInit;

fn get_output_path(config: &ConfigInit) -> PathBuf {
    config.video.with_file_name(format!(
        "{}-output.{}",
        config.video.file_stem().unwrap().to_str().unwrap(),
        config.video.extension().unwrap().to_str().unwrap()
    ))
}

fn get_ffmpeg_command(config: &ConfigInit, output_path: &PathBuf) -> String {
    format!(
        "ffmpeg -i '{}' -vcodec libx264 -crf {} -preset {} -acodec aac -b:a 128k -y '{}'",
        &config.video.display(),
        &config.c,
        &config.p,
        &output_path.display()
    )
}

fn run_command(execute_arg: &str) -> std::process::Output {
    Command::new("sh")
        .arg("-c")
        .arg(execute_arg)
        .output()
        .expect("Failed to execute ffmpeg.")
}

pub fn run(config: &ConfigInit) -> String {
    let output_path = get_output_path(config);
    let execute_arg = get_ffmpeg_command(config, &output_path);

    println!("Processing file...");
    let execute = run_command(&execute_arg);

    if config.debug {
        format!(
            "Standard Output:\n{}\nStandard Error:\n{}",
            String::from_utf8_lossy(&execute.stdout),
            String::from_utf8_lossy(&execute.stderr)
        )
    } else {
        if execute.status.success() {
            "Process completed successfully".to_string()
        } else {
            format!("Process failed with status: {}", execute.status)
        }
    }
}
