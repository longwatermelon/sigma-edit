use rand::Rng;
use std::fs;
use std::process::Command;
use std::collections::HashMap;

const LAST_BG: i32 = 1;
const TRACK_NUM: i32 = 5;

pub fn create() {
    let paths_iter = fs::read_dir("res/compilation").unwrap();
    let mut paths: Vec<String> = Vec::new();
    for path in paths_iter {
        let p = path.unwrap().path();
        if p.is_file() && p.extension().unwrap() == "mp3" {
            paths.push(p.to_str().unwrap().to_string());
        }
    }

    let audios: Vec<String> = (0..TRACK_NUM).map(|_| {
        let index: usize = rand::thread_rng().gen_range(0..paths.len());
        paths.remove(index)
    }).collect();

    let mut audio_info: HashMap<&str, (&str, &str)> = HashMap::new();
    audio_info.insert("res/compilation/callme.mp3", ("Call Me", "https://youtu.be/1aAY7EI3u8c"));
    audio_info.insert("res/compilation/live-another-day.mp3", ("Live Another Day", "https://youtu.be/7pmUQJ5KAlU"));
    audio_info.insert("res/compilation/metamorphosis.mp3", ("METAMORPHOSIS", "https://youtu.be/lJvRohYSrZM"));
    audio_info.insert("res/compilation/override.mp3", ("Override", "https://youtu.be/NS8DPG62Fto"));
    audio_info.insert("res/compilation/rave.mp3", ("Rave", "https://youtu.be/PTZgxW_3LIQ"));
    audio_info.insert("res/compilation/sahara.mp3", ("Sahara", "https://youtu.be/pIZ0QRWK0zg"));
    audio_info.insert("res/compilation/scopin.mp3", ("Scopin", "https://youtu.be/VXaq77GiyEo"));
    audio_info.insert("res/compilation/gigachad.mp3", ("Gigachad Theme (Phonk House Version)", "https://youtu.be/OVh0bMNSFss"));
    audio_info.insert("res/compilation/immaculate.mp3", ("IMMACULATE", "https://youtu.be/6dz481Zv3KQ"));
    audio_info.insert("res/compilation/neon-blade.mp3", ("NEON BLADE", "https://youtu.be/dvQJIgjlR3I"));
    audio_info.insert("res/compilation/disaster.mp3", ("Disaster", "https://youtu.be/Pnq1_3BXIqA"));
    audio_info.insert("res/compilation/cthulhu.mp3", ("CTHULU", "https://youtu.be/QnoiWy3e0mA"));
    audio_info.insert("res/compilation/rapture.mp3", ("RAPTURE (sped up)", "https://youtu.be/OZRQMYkjE58"));

    let mut desc: String = String::new();
    let mut timestamp: u64 = 0;
    for audio in &audios {
        println!("{}", audio);
        desc.push_str(format!("{}:{:0>2} {} ({})\n", timestamp / 60, timestamp % 60,
                audio_info.get(audio.as_str()).unwrap().0,
                audio_info.get(audio.as_str()).unwrap().1
            ).as_str()
        );
        let seconds: u64 = mp3_duration::from_path(audio.as_str()).unwrap().as_secs();
        timestamp += seconds;
    }

    println!("Audios: {:?}", audios);
    fs::write("output/desc", desc).unwrap();

    let mut ffmpeg_cmd: String = String::from("ffmpeg");
    for audio in &audios {
        ffmpeg_cmd.push_str(format!(" -i {}", audio).as_str());
    }

    println!("Concat audios...");
    ffmpeg_cmd.push_str(format!(" -filter_complex \"[0:a][1:a]concat=n={}:v=0:a=1\" output/audio.mp3", audios.len()).as_str());
    Command::new("sh").args(["-c", ffmpeg_cmd.as_str()]).output().unwrap();

    println!("Overlay image...");
    ffmpeg_cmd = format!(
        "ffmpeg -loop 1 -i res/compilation/backgrounds/{}.png -i output/audio.mp3 -c:v libx264 -tune stillimage -c:a aac -b:a 192k -pix_fmt yuv420p -shortest output/0.mp4",
        rand::thread_rng().gen_range(0..=LAST_BG)
    );
    Command::new("sh").args(["-c", ffmpeg_cmd.as_str()]).output().unwrap();

    println!("Done");
}

