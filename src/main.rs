mod common;
mod effects;
mod edit;
mod compare;
mod month;
mod video;

use rand::Rng;
use std::fs;
use std::process::{exit, Command};
use std::collections::HashMap;

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

    let n: i32 = if args.is_empty() {
        1
    } else {
        let res = args[0].parse();
        if let Ok(i) = res {
            i
        } else {
            -1
        }
    };

    if n == -1 && args[0] == "playlist" {
        println!("Producing playlist.");
        let paths_iter = fs::read_dir("res/compilation").unwrap();
        let mut paths: Vec<String> = Vec::new();
        for path in paths_iter {
            let p = path.unwrap().path();
            if p.is_file() {
                paths.push(p.to_str().unwrap().to_string());
            }
        }

        let audios: Vec<String> = (0..6).map(|_| {
            let index: usize = rand::thread_rng().gen_range(0..paths.len());
            paths.remove(index)
        }).collect();

        let mut audio_info: HashMap<&str, (&str, &str)> = HashMap::new();
        audio_info.insert("res/compilation/callme.mp3", ("Call Me by plenka", "https://youtu.be/1aAY7EI3u8c"));
        audio_info.insert("res/compilation/live-another-day.mp3", ("Live Another Day by Kordhell", "https://youtu.be/7pmUQJ5KAlU"));
        audio_info.insert("res/compilation/metamorphosis.mp3", ("METAMORPHOSIS by INTERWORLD", "https://youtu.be/lJvRohYSrZM"));
        audio_info.insert("res/compilation/override.mp3", ("Override by KSLV", "https://youtu.be/NS8DPG62Fto"));
        audio_info.insert("res/compilation/rave.mp3", ("Rave by Dxrk ダーク", "https://youtu.be/PTZgxW_3LIQ"));
        audio_info.insert("res/compilation/sahara.mp3", ("Sahara by Hensonn", "https://youtu.be/pIZ0QRWK0zg"));

        let mut desc: String = String::new();
        let mut timestamp: u64 = 0;
        for audio in &audios {
            desc.push_str(format!("{}:{:0>2} {} ({})\n", timestamp / 60, timestamp % 60,
                    audio_info.get(audio.as_str()).unwrap().0,
                    audio_info.get(audio.as_str()).unwrap().1
                ).as_str()
            );
            let seconds: u64 = mp3_duration::from_path(audio.as_str()).unwrap().as_secs();
            timestamp += seconds;
        }

        println!("Audios: {:?}", audios);
        println!("Description:\n============\nTIMESTAMPS\n{}============", desc);

        let mut ffmpeg_cmd: String = String::from("ffmpeg");
        for audio in &audios {
            ffmpeg_cmd.push_str(format!(" -i {}", audio).as_str());
        }

        ffmpeg_cmd.push_str(format!(" -filter_complex \"[0:a][1:a]concat=n={}:v=0:a=1\" audio.mp3", audios.len()).as_str());
        ffmpeg_cmd.push_str(format!(" && ffmpeg -loop 1 -i res/compilation/backgrounds/0.png -i audio.mp3 -c:v libx264 -tune stillimage -c:a aac -b:a 192k -pix_fmt yuv420p -shortest output/0.mp4").as_str());

        println!("Concat audios...");
        Command::new("sh").args(["-c", ffmpeg_cmd.as_str()]).output().unwrap();

        println!("Overlay image...");
        ffmpeg_cmd = String::from("ffmpeg -loop 1 -i res/compilation/backgrounds/0.png -i audio.mp3 -c:v libx264 -tune stillimage -c:a aac -b:a 192k -pix_fmt yuv420p -shortest output/0.mp4");
        Command::new("sh").args(["-c", ffmpeg_cmd.as_str()]).output().unwrap();

        println!("Done");
        fs::remove_file("audio.mp3").ok();
    } else {
        for i in 0..n {
            println!("Producing video {}/{}...", i + 1, n);
            let filename: String = format!("output/{}.mp4", i);
            video::produce(filename.as_str());
        }
    }
}

