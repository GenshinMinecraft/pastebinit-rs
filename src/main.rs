// #![warn(clippy::all, clippy::pedantic)]

use crate::args::upload;
use clap::Parser;
use std::io::{IsTerminal, Read};
use std::process::exit;

mod args;
mod pastebin_server;

fn main() {
    let config = args::Config::parse();

    let mut all_lines = String::new();

    if config.file.is_empty() {
        let mut stdin = std::io::stdin();
        if stdin.is_terminal() {
            println!("Run on a terminal, try to type something and end by `EOF`!");
            loop {
                let mut line = String::new();
                if stdin.read_line(&mut line).is_err() {
                    println!("Error reading from stdin. Skipping.");
                    continue;
                }

                if line.trim() == "EOF" {
                    break;
                }

                all_lines.push_str(&line);
            }
        } else {
            let mut buffer = Vec::new();
            if stdin.read_to_end(&mut buffer).is_err() {
                println!("Error reading from stdin. Skipping.");
            } else {
                all_lines = String::from_utf8_lossy(&buffer).to_string();
            }
        }
    } else {
        match std::fs::read_to_string(&config.file) {
            Ok(content) => all_lines = content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", config.file, e);
                return;
            }
        }
    }

    let url = upload(
        config.server,
        all_lines,
        config.title,
        config.private,
        config.raw,
    )
    .map_err(|e| {
        eprintln!("Error uploading paste: {e}");
        exit(1);
    })
    .ok()
    .unwrap_or_default();

    println!("{url}");
}
