mod video;

use rand::Rng;

use std::fs;

fn t(minute: i32, seconds: i32) -> f32 {
    minute as f32 + seconds as f32 / 60.
}

fn produce_video(output_path: &str) {
    let songs: Vec<(&str, Vec<f32>)> = vec![
        ("res/metamorphosis.mp3", (0..18).map(|x| x as f32 * 0.67).collect()),
        ("res/neon-blade.mp3", vec![vec![0., 2.68], (1..18).map(|x| 2.68 + x as f32 * 0.633).collect()].into_iter().flatten().collect()),
        ("res/dancin.mp3", (0..34).map(|x| x as f32 * 0.53).collect()),
        ("res/mtg.mp3", (0..9).map(|x| x as f32 * 1.89).collect())
    ];
    let song: (&str, Vec<f32>) = songs[rand::thread_rng().gen_range(0..songs.len())].clone();
    println!("Selected random song: {}", song.0);

    video::create("res/bateman.mp4", "no-audio.mp4", &song.1,
            &[t(0, 2), t(0, 7), t(0, 11), t(0, 16), t(0, 22), t(0, 24), t(0, 27), t(0, 30), t(0, 34),
              t(1, 6), t(1, 10), t(1, 14), t(1, 17), t(1, 22), t(1, 25), t(1, 29), t(1, 39), t(1, 40), t(1, 43), t(1, 49), t(1, 56), t(1, 58),
              t(2, 6), t(2, 9), t(2, 14), t(2, 16), t(2, 19), t(2, 30), t(2, 32), t(2, 34), t(2, 40), t(2, 42), t(2, 46), t(2, 52), t(2, 54), t(2, 55),
              t(3, 6), t(3, 11), t(3, 13), t(3, 15), t(3, 17), t(3, 19), t(3, 22), t(3, 29), t(3, 36), t(3, 42), t(3, 55), t(3, 59),
              t(4, 2), t(4, 5), t(4, 11), t(4, 12), t(4, 16), t(4, 20), t(4, 24), t(4, 29), t(4, 34), t(4, 37), t(4, 42), t(4, 46), t(4, 47), t(4, 48), t(4, 52), t(4, 53), t(4, 55), t(4, 56), t(4, 58), t(4, 59),
              t(5, 0), t(5, 1), t(5, 2), t(5, 4), t(5, 8), t(5, 10), t(5, 11), t(5, 14), t(5, 17), t(5, 18), t(5, 21), t(5, 26), t(5, 29), t(5, 34), t(5, 41), t(5, 46), t(5, 51), t(5, 56),
              t(6, 0), t(6, 6), t(6, 10), t(6, 19), t(6, 57),
              t(8, 15), t(8, 20), t(8, 32), t(8, 37), t(8, 50), t(8, 52),
              t(9, 0), t(9, 14)]
    ).expect("Error in creating video.");

    println!("Adding audio...");
    std::process::Command::new("ffmpeg").arg("-i").arg("no-audio.mp4").arg("-i").arg(song.0).arg("-c:v").arg("copy").arg("-c:a").arg("aac").arg("-strict").arg("experimental")
        .arg("-shortest").arg(output_path).output().expect("Failed to overlay audio.");

    println!("Cleaning up...");
    fs::remove_file("no-audio.mp4").expect("Unable to remove no-audio.mp4.");
}

fn main() {
    fs::remove_dir_all("output").expect("Unable to remove contents of output/. Does the directory exist?");
    fs::create_dir("output").expect("Unable to create output directory.");

    for i in 0..5 {
        println!("Producing video {}/5...", i + 1);
        produce_video(format!("output/{}.mp4", i).as_str());
    }
}

