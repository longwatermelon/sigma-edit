use opencv::{prelude::*, core, videoio, Result};
use opencv::videoio::{VideoCapture, VideoWriter};
use rand::Rng;

pub fn create(input: &str, output: &str, beats: &[f32]) -> Result<()> {
    let mut video: VideoCapture = VideoCapture::from_file(input, videoio::CAP_ANY)?; // 0 is the default camera
    let w: i32 = video.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let h: i32 = video.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
    let mut out = VideoWriter::new(output,
        VideoWriter::fourcc('m', 'p', '4', 'v')?, 30.,
        core::Size_ { width: w, height: h }, true
    )?;

    for i in 1..beats.len() {
        println!("Writing beat interval {:.2} to {:.2}...", beats[i - 1], beats[i]);
        write_beat_interval(&mut out, &mut video, beats[i] - beats[i - 1])?;
    }

    out.release()?;
    Ok(())
}

fn write_beat_interval(writer: &mut VideoWriter, video: &mut VideoCapture, beat_len: f32) -> Result<()> {
    let frames: i32 = (30. * beat_len) as i32;
    let total_frames: i32 = video.get(videoio::CAP_PROP_FRAME_COUNT)? as i32;
    let begin: i32 = rand::thread_rng().gen_range(0..(total_frames - frames));

    video.set(videoio::CAP_PROP_POS_FRAMES, begin as f64)?;
    for _ in 0..frames {
        let mut frame: Mat = Mat::default();
        video.read(&mut frame)?;

        writer.write(&frame)?;
    }

    Ok(())
}

