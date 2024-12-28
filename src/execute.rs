use std::process::Command;
use std::path::PathBuf;

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
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(execute_arg)
            .output()
            .expect("Failed to execute ffmpeg.")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(execute_arg)
            .output()
            .expect("Failed to execute ffmpeg.")
    }
}

pub fn execute(config: &ConfigInit) -> String {
    let output_path = get_output_path(config);
    let execute_arg = get_ffmpeg_command(config, &output_path);
    let execute = run_command(&execute_arg);

    if execute.status.success() {
        "Process completed successfully".to_string()
    } else {
        format!("Process failed with status: {}", execute.status)
    }
}

pub fn debug(config: &ConfigInit) -> String {
    let output_path = get_output_path(config);
    let execute_arg = get_ffmpeg_command(config, &output_path);
    let execute = run_command(&execute_arg);

    format!(
        "Standard Output:\n{}\nStandard Error:\n{}",
        String::from_utf8_lossy(&execute.stdout),
        String::from_utf8_lossy(&execute.stderr)
    )
}