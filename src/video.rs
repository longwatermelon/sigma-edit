use crate::{edit, compare, month};
use opencv::{prelude::*, core, videoio};
use opencv::videoio::{VideoCapture, VideoWriter};
use rand::Rng;
use std::{io::Read, fs, fs::File};

enum Config<'a> {
    Edit {
        input: &'a str,
        cuts: &'a [f32],
        slow: bool
    },
    Compare {
        rig_ties: bool,
        probability: f32
    },
    Month
}

#[derive(Clone)]
struct Song<'a> {
    path: &'a str,
    beats: Vec<f32>
}

#[derive(Clone)]
struct Video<'a> {
    path: &'a str,
    cuts: Vec<f32>
}

impl<'a> Song<'a> {
    pub fn new(path: &'a str, beats: Vec<f32>) -> Self {
        Self { path, beats }
    }
}

impl<'a> Video<'a> {
    pub fn new(path: &'a str, cuts: Vec<f32>) -> Self {
        Self { path, cuts }
    }
}

pub fn produce(output_path: &str) {
    let mut file = File::open("config/config.json").expect("Failed to read config.");
    let mut contents: String = String::new();
    file.read_to_string(&mut contents).unwrap();
    let cfg: serde_json::Value = serde_json::from_str(&contents).expect("Failed to parse json.");

    let song_path: &str = match rand::thread_rng().gen_range(0..3) {
        0 => produce_edit(),
        1 => produce_compare(&cfg),
        2 => produce_month(),
        _ => unreachable!()
    };

    println!("Adding audio...");
    std::process::Command::new("ffmpeg").arg("-i").arg("no-audio.mp4").arg("-i").arg(song_path).arg("-c:v").arg("copy").arg("-c:a").arg("aac").arg("-strict").arg("experimental")
        .arg("-shortest").arg(output_path).output().expect("Failed to overlay audio.");

    println!("Cleaning up...");
    fs::remove_file("no-audio.mp4").expect("Unable to remove no-audio.mp4.");
}

fn create(output: &str, beats: &[f32], cfg: Config) -> opencv::Result<()> {
    let mut out = VideoWriter::new(output,
        VideoWriter::fourcc('m', 'p', '4', 'v')?, 30.,
        core::Size_ { width: 1080, height: 1920 }, true
    )?;

    match cfg {
        Config::Edit { input, cuts, slow } => edit::create(&mut out, &mut VideoCapture::from_file(input, videoio::CAP_ANY)?, beats, cuts, slow)?,
        Config::Compare { rig_ties, probability } => compare::create(&mut out, beats,
            VideoCapture::from_file("res/video/compare/combined.mp4", videoio::CAP_ANY)?,
            VideoCapture::from_file("res/video/compare/bateman.mp4", videoio::CAP_ANY)?,
            VideoCapture::from_file("res/video/compare/shelby.mp4", videoio::CAP_ANY)?,
            rig_ties, probability
        )?,
        Config::Month => month::create(&mut out, beats)?
    }

    out.release()?;
    Ok(())
}

fn t(minute: i32, seconds: i32) -> f32 {
    minute as f32 + seconds as f32 / 60.
}

fn random_song<'a>(options: &[&str]) -> Song<'a> {
    let songs: Vec<Song> = vec![
        Song::new("res/audio/metamorphosis.mp3", vec![vec![0., 2.72], (1..35).map(|x| 2.72 + x as f32 * 0.7).collect()].into_iter().flatten().collect()),
        Song::new("res/audio/neon-blade.mp3", vec![vec![0., 2.68], (1..18).map(|x| 2.68 + x as f32 * 0.635).collect()].into_iter().flatten().collect()),
        Song::new("res/audio/dancin.mp3", vec![vec![0., 3.55], (1..35).map(|x| 3.55 + x as f32 * 0.53).collect()].into_iter().flatten().collect()),
        Song::new("res/audio/mtg.mp3", (0..9).map(|x| x as f32 * 1.89).collect()),
        Song::new("res/audio/murder-in-my-mind.mp3", (0..35).map(|x| x as f32 * 0.4999999).collect()),
        Song::new("res/audio/immaculate.mp3", vec![(0..9).map(|x| x as f32 * 1.).collect::<Vec<f32>>(), (1..18).map(|x| 8. + x as f32 * 0.54).collect()].into_iter().flatten().collect()),
        Song::new("res/audio/miss-you.mp3", vec![vec![0., 3.95], (1..35).map(|x| 3.95 + x as f32 * 0.433333).collect()].into_iter().flatten().collect())
    ];

    loop {
        let song: Song = songs[rand::thread_rng().gen_range(0..songs.len())].clone();

        if options.is_empty() || options.contains(&song.path) {
            return song;
        }
    }
}

fn produce_edit<'a>() -> &'a str {
    println!("Video type: Edit");

    let videos: Vec<Video> = vec![
        Video::new("res/video/edit/bateman.mp4",
         vec![t(0, 2), t(0, 7), t(0, 11), t(0, 16), t(0, 22), t(0, 24), t(0, 27), t(0, 30), t(0, 34),
              t(1, 6), t(1, 10), t(1, 14), t(1, 17), t(1, 22), t(1, 25), t(1, 29), t(1, 39), t(1, 40), t(1, 43), t(1, 49), t(1, 56), t(1, 58),
              t(2, 6), t(2, 9), t(2, 14), t(2, 16), t(2, 19), t(2, 30), t(2, 32), t(2, 34), t(2, 40), t(2, 42), t(2, 46), t(2, 52), t(2, 54), t(2, 55),
              t(3, 6), t(3, 11), t(3, 13), t(3, 15), t(3, 17), t(3, 19), t(3, 22), t(3, 29), t(3, 36), t(3, 42), t(3, 55), t(3, 59),
              t(4, 2), t(4, 5), t(4, 11), t(4, 12), t(4, 16), t(4, 20), t(4, 24), t(4, 29), t(4, 34), t(4, 37), t(4, 42), t(4, 46), t(4, 47), t(4, 48), t(4, 52), t(4, 53), t(4, 55), t(4, 56), t(4, 58), t(4, 59),
              t(5, 0), t(5, 1), t(5, 2), t(5, 4), t(5, 8), t(5, 10), t(5, 11), t(5, 14), t(5, 17), t(5, 18), t(5, 21), t(5, 26), t(5, 29), t(5, 34), t(5, 41), t(5, 46), t(5, 51), t(5, 56),
              t(6, 0), t(6, 6), t(6, 10), t(6, 19), t(6, 57),
              t(8, 15), t(8, 20), t(8, 32), t(8, 37), t(8, 50), t(8, 52),
              t(9, 0), t(9, 14)]),
        Video::new("res/video/edit/peaky-blinders.mp4", Vec::new()) // Peaky blinders has less cuts so cuts vector isn't necessary
    ];
    let video: Video = videos[rand::thread_rng().gen_range(0..videos.len())].clone();
    println!("Video: {}", video.path);

    let song: Song = random_song(&[]);
    println!("Music: {}", song.path);

    create("no-audio.mp4", song.beats.as_slice(), Config::Edit {
        input: video.path,
        cuts: video.cuts.as_slice(),
        slow: false
    }).expect("Failed to create video.");

    song.path
}

fn produce_compare<'a>(cfg: &serde_json::Value) -> &'a str {
    println!("Video type: Comparison");

    let song: Song = random_song(&[
        "res/audio/dancin.mp3",
        "res/audio/metamorphosis.mp3",
        "res/audio/miss-you.mp3"
    ]);
    println!("Music: {}", song.path);

    let rig_ties: bool = cfg["rig-ties"].as_bool().unwrap_or(false);
    println!("Tie rigging: {}", rig_ties);

    let probability: f32 = cfg["probability"].as_f64().unwrap_or(0.5) as f32;
    if !rig_ties {
        println!("Probability: {:.2}", probability);
    } else {
        println!("Probability: Not applicable");
    }

    create("no-audio.mp4", song.beats.as_slice(), Config::Compare {
        rig_ties, probability
    }).expect("Failed to create video.");

    song.path
}

fn produce_month<'a>() -> &'a str {
    println!("Video type: Month");

    let song: Song = random_song(&[
        "res/audio/dancin.mp3",
        "res/audio/metamorphosis.mp3",
        "res/audio/miss-you.mp3"
    ]);
    println!("Music: {}", song.path);

    create("no-audio.mp4", song.beats.as_slice(), Config::Month).expect("Failed to create video.");
    song.path
}

