mod effects;
mod edit;
mod video;

use rand::Rng;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if fs::remove_dir_all("output").is_err() {
        eprintln!("Unable to remove contents of output/. Does the directory exist?");
        exit(1);
    }

    if fs::create_dir("output").is_err() {
        eprintln!("Unable to create output directory.");
        exit(1);
    }

    let n: i32 = if args.is_empty() { 1 } else { args[0].parse().unwrap() };
    for i in 0..n {
        let choice: i32 = rand::thread_rng().gen_range(0..1);
        let filename: String = format!("output/{}.mp4", i);

        println!("Producing video {}/{}...", i + 1, n);
        match choice {
            0 => video::produce_edit(filename.as_str()),
            _ => ()
        }
    }
}

