use crate::effects;
use crate::common::print_progress;
use opencv::{prelude::*, videoio};
use opencv::videoio::{VideoCapture, VideoWriter};
use rand::Rng;

pub fn create(writer: &mut VideoWriter, video: &mut VideoCapture, beats: &[f32], cuts: &[f32], slow: bool) -> opencv::Result<()> {
    let jokes: Vec<(String,String)> = vec![
        ("Why did the sigma cross the road?", "Because the grimace shake was on the other side."),
        ("What is it called when a sigma is mewing?", "Sigma Mewing."),
        ("Why don't sigmas joke?", "Because they're mewing."),
        ("Why was the sigma sad?", "Because he broke his edging streak."),
        ("What do you call a sigma with no teeth?", "A toothless sigma."),
        ("What did one sigma say to the other sigma?", "Nothing, he was mewing."),
    ].iter().map(|x| (x.0.to_string(),x.1.to_string())).collect();
    let joke=jokes[rand::thread_rng().gen_range(0..jokes.len())].clone();

    for i in 1..beats.len() {
        print_progress(i + 1, beats.len());
        write_beat_interval(writer, video, beats[i] - beats[i - 1], cuts, if i==1 {joke.0.clone()} else {joke.1.clone()}, slow)?;
    }
    println!();

    Ok(())
}

fn write_beat_interval(writer: &mut VideoWriter, video: &mut VideoCapture, beat_len: f32, cuts: &[f32], text: String, slow_video: bool) -> opencv::Result<()> {
    let frames: i32 = (30. * beat_len) as i32;
    let total_frames: i32 = video.get(videoio::CAP_PROP_FRAME_COUNT)? as i32;

    let mut begin: f32;
    loop {
        begin = rand::thread_rng().gen_range(0..(total_frames - frames)) as f32;

        let mut valid: bool = true;
        for &cut in cuts {
            if begin < cut && cut - begin < frames as f32 {
                valid = false;
                break;
            }
        }

        if valid {
            break;
        }
    }

    video.set(videoio::CAP_PROP_POS_FRAMES, begin as f64)?;
    for i in 0..frames {
        let progress: f32 = i as f32 / frames as f32;

        let mut frame: Mat = Mat::default();
        if slow_video {
            frame = effects::slow(video, begin as i32, i, frames)?;
        } else {
            video.read(&mut frame)?;
        }

        let mut adjusted: Mat = effects::shift(&frame,
            (25. * f32::exp(-10. * progress) * f32::cos(1.5 * i as f32 + 0.5)) as i32,
            (25. * f32::exp(-10. * progress) * f32::sin(2. * i as f32)) as i32
        );

        effects::draw_text(&mut adjusted, text.as_str(), None, None, 1., 3)?;

        writer.write(&adjusted)?;
    }

    Ok(())
}

