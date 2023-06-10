use opencv::{prelude::*, core, videoio, Result};
use opencv::videoio::{VideoCapture, VideoWriter};

fn main() -> Result<()> {
    let mut video: VideoCapture = VideoCapture::from_file("res/bateman.mp4", videoio::CAP_ANY)?; // 0 is the default camera
    let w: i32 = video.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let h: i32 = video.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
    let mut out = VideoWriter::new("out.mp4",
        VideoWriter::fourcc('m', 'p', '4', 'v')?, 30.,
        core::Size_ { width: w, height: h }, true
    )?;

    for _ in 0..100 {
        let mut frame: Mat = Mat::default();
        video.read(&mut frame)?;

        out.write(&frame)?;
    }

    out.release()?;
    std::process::Command::new("mpv").arg("out.mp4").output().expect("Failed to view output video.");

    Ok(())
}

