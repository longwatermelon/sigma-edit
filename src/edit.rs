use crate::effects;
use crate::video::print_progress;
use opencv::{prelude::*, videoio};
use opencv::videoio::{VideoCapture, VideoWriter};
use opencv::core::{Scalar, Point};
use opencv::imgproc::put_text;
use rand::Rng;

pub fn create(writer: &mut VideoWriter, video: &mut VideoCapture, beats: &[f32], cuts: &[f32], slow: bool) -> opencv::Result<()> {
    let quotes: Vec<String> = vec![
        "Lone wolf by choice.",
        "Be independent.",
        "March to your own beat.",
        "Embrace solitude, find strength.",
        "Master of my own destiny.",
        "Unconventional and free-spirited.",
        "Silent strength, hidden potential.",
        "Society's labels don't define me.",
        "Reserved but self-assured.",
        "Walk my own path, create my own rules.",
        "Trust no one.",
        "Disregard relationships.",
        "I live life on my own terms.",
        "I'm here to stand out, not fit in.",
        "The less you care, the happier you'll be.",
        "Don't be a follower.",
        "The most powerful weapon is knowledge.",
        "Observe, don't speak."
    ].iter().map(|x| x.to_string()).collect();
    let quote: String = quotes[rand::thread_rng().gen_range(0..quotes.len())].clone();
    let rule_num: i32 = rand::thread_rng().gen_range(1..200);

    for i in 1..beats.len() {
        print_progress(i + 1, beats.len());
        write_beat_interval(writer, video, beats[i] - beats[i - 1], cuts, quote.clone(), rule_num, slow)?;
    }
    println!();

    Ok(())
}

fn write_beat_interval(writer: &mut VideoWriter, video: &mut VideoCapture, beat_len: f32, cuts: &[f32], quote: String, rule_number: i32, slow_video: bool) -> opencv::Result<()> {
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

        let text: String = format!("Sigma Rule #{}: {}", rule_number, quote);
        let font_scale = 1.;
        let thickness = 3;
        let text_size = opencv::imgproc::get_text_size(text.as_str(), 0, font_scale, thickness, &mut 0)?;

        let x = (frame.cols() - text_size.width) / 2;
        let y = (frame.rows() - text_size.height) / 2;

        put_text(
            &mut adjusted,
            text.as_str(),
            Point::new(x, y),
            opencv::imgproc::FONT_HERSHEY_COMPLEX,
            font_scale,
            Scalar::new(255., 255., 255., 0.),
            thickness,
            0,
            false
        )?;

        writer.write(&adjusted)?;
    }

    Ok(())
}

