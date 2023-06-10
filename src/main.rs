mod video;

fn main() {
    video::create("res/bateman.mp4", "no-audio.mp4", &(0..18).map(|x| x as f32 * 0.67).collect::<Vec<f32>>()).expect("Error in creating video.");
    std::process::Command::new("rm").arg("output.mp4").output().unwrap();
    println!("Adding audio...");
    std::process::Command::new("ffmpeg").arg("-i").arg("no-audio.mp4").arg("-i").arg("res/metamorphosis.mp3").arg("-c:v").arg("copy").arg("-c:a").arg("aac").arg("-strict").arg("experimental")
        .arg("-shortest").arg("output.mp4").output().expect("Failed to overlay audio.");

    println!("Cleaning up...");
    std::process::Command::new("rm").arg("no-audio.mp4").output().unwrap();

    println!("Playing video...");
    std::process::Command::new("mpv").arg("output.mp4").output().expect("Failed to view output video.");
}

