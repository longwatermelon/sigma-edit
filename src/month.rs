use crate::effects;
use crate::common::print_progress;
use opencv::{prelude::*, videoio};
use opencv::videoio::{VideoCapture, VideoWriter};
use rand::Rng;

pub fn create(writer: &mut VideoWriter, beats: &[f32]) -> opencv::Result<()> {
    let months: &[&str] = &["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let mut sigmas: Vec<&str> = vec!["bateman", "batman", "gus", "homelander", "joker", "kratos", "chad", "rickgrimes", "shelby", "tylerdurden", "walter", "wick"];
    let mut names: Vec<&str> = vec!["Patrick Bateman", "Batman", "Gus", "Homelander", "Joker", "Kratos", "Gigachad", "Rick Grimes", "Tommy Shelby", "Tyler Durden", "Walter White", "John Wick"];
    let mut counter: usize = 0;

    let mut name: &str = "";
    let mut shelby_video: VideoCapture = VideoCapture::from_file("res/video/compare/shelby.mp4", videoio::CAP_ANY)?;
    let mut video: VideoCapture = VideoCapture::from_file("res/video/compare/shelby.mp4", videoio::CAP_ANY)?;

    for i in 1..beats.len() {
        // Switch video on even beats
        if i % 2 == 0 && counter != 12 {
            if i != 2 {
                counter += 1;
            }

            if counter != 12 {
                let index: usize = rand::thread_rng().gen_range(0..sigmas.len());
                let sigma: &str = sigmas[index];
                name = names[index];

                video = VideoCapture::from_file(format!("res/video/month/{}.mp4", sigma).as_str(), videoio::CAP_ANY)?;
                sigmas.remove(index);
                names.remove(index);
            }
        }

        print_progress(i + 1, beats.len());

        if counter == 12 {
            write_beat_interval(writer, &mut shelby_video, beats[i] - beats[i - 1], "Which sigma did you get?", "Comment down below", false)?;
        } else {
            write_beat_interval(writer, &mut video, beats[i] - beats[i - 1], if i != 1 { months[counter] } else { "" }, if i % 2 == 1 { name } else { "" }, true)?;
        }
    }
    println!();

    Ok(())
}

fn write_beat_interval(writer: &mut VideoWriter, video: &mut VideoCapture, beat_len: f32, month: &str, name: &str, shake: bool) -> opencv::Result<()> {
    let frames: i32 = (30. * beat_len) as i32;
    // video.set(videoio::CAP_PROP_POS_FRAMES, 0.)?;

    for i in 0..frames {
        let progress: f32 = i as f32 / frames as f32;

        let mut frame: Mat = Mat::default();
        video.read(&mut frame)?;

        let mut adjusted: Mat = if shake {
            effects::shift(&frame,
                (25. * f32::exp(-10. * progress) * f32::cos(1.5 * i as f32 + 0.5)) as i32,
                (25. * f32::exp(-10. * progress) * f32::sin(2. * i as f32)) as i32
            )
        } else {
            frame
        };

        let scale: f64 = if shake { 2.5 } else { 2. };

        if month == "" && name == "" {
            effects::draw_text(&mut adjusted, "Your Birthday Month", None, Some(-50), scale, 3)?;
            effects::draw_text(&mut adjusted, "Your Sigma", None, Some(50), scale, 3)?;
        } else {
            effects::draw_text(&mut adjusted, month, None, None, scale, 3)?;
            effects::draw_text(&mut adjusted, name, None, Some(100), scale, 3)?;
        }

        writer.write(&adjusted)?;
    }

    Ok(())
}

