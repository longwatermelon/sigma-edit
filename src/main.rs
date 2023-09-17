mod common;
mod effects;
mod short;
mod edit;
mod compare;
mod month;
mod bg;
mod playlist;

use std::fs;
use std::process::exit;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        args.push(String::from("1"));
    }

    if fs::remove_dir_all("out").is_err() {
        eprintln!("Unable to remove contents of out/. Does the directory exist?");
        exit(1);
    }

    if fs::create_dir("out").is_err() {
        eprintln!("Unable to create out directory.");
        exit(1);
    }

    if args[0] == "playlist" {
        println!("Producing playlist.");
        playlist::create();
    } else if args[0] == "type" {
        println!("Producing video type '{}'...", args[1]);

        let index: i32 = match args[1].as_str() {
            "edit" => 0,
            "comparison" => 1,
            "month" => 2,
            "wallpaper" => 3,
            _ => {
                eprintln!("'{}' is not a valid video type.", args[1]);
                std::process::exit(1);
            }
        };

        short::produce("out/0.mp4", Some(index));
    } else if args[0].parse::<i32>().is_ok() {
        let n: i32 = args[0].parse().unwrap();

        for i in 0..n {
            println!("Producing video {}/{}...", i + 1, n);
            let filename: String = format!("out/{}.mp4", i);
            short::produce(filename.as_str(), None);
        }
    } else {
        println!("Unrecognized argument '{}'.", args[0]);
    }
}