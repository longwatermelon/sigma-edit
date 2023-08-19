mod common;
mod effects;
mod short;
mod edit;
mod compare;
mod month;
mod playlist;

use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if fs::remove_dir_all("out").is_err() {
        eprintln!("Unable to remove contents of out/. Does the directory exist?");
        exit(1);
    }

    if fs::create_dir("out").is_err() {
        eprintln!("Unable to create out directory.");
        exit(1);
    }

    let n: i32 = if args.is_empty() {
        1
    } else {
        let res = args[0].parse();
        if let Ok(i) = res {
            i
        } else {
            if args[0] == "playlist" {
                -1
            } else {
                -2
            }
        }
    };

    // Playlist
    if n == -1 {
        println!("Producing playlist.");
        playlist::create();
    }
    else if n == -2 {
        eprintln!("Unrecognized argument '{}'.", args[0]);
        exit(1);
    } else {
        for i in 0..n {
            println!("Producing video {}/{}...", i + 1, n);
            let filename: String = format!("out/{}.mp4", i);
            short::produce(filename.as_str());
        }
    }
}

