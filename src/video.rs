use crate::edit;
use opencv::{prelude::*, core, videoio, Result};
use opencv::videoio::{VideoCapture, VideoWriter};
use std::{io, io::Write};

pub enum Config<'a> {
    Edit{
        cuts: &'a [f32],
        slow: bool
    }
}

pub fn create(input: &str, output: &str, beats: &[f32], cfg: Config) -> Result<()> {
    let mut video: VideoCapture = VideoCapture::from_file(input, videoio::CAP_ANY)?; // 0 is the default camera
    let w: i32 = video.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let h: i32 = video.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
    let mut out = VideoWriter::new(output,
        VideoWriter::fourcc('m', 'p', '4', 'v')?, 30.,
        core::Size_ { width: w, height: h }, true
    )?;

    for i in 1..beats.len() {
        print!("\r({}/{}) Writing beat interval {:.2} to {:.2}...", i, beats.len() - 1, beats[i - 1], beats[i]);
        io::stdout().flush().unwrap();

        let beat_len: f32 = beats[i] - beats[i - 1];
        match cfg {
            Config::Edit { cuts, slow } => edit::write(&mut out, &mut video, beat_len, cuts, slow)?
        }
    }
    println!();

    out.release()?;
    Ok(())
}

