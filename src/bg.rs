use crate::effects;
use opencv::{prelude::*, videoio};
use opencv::videoio::{VideoCapture, VideoWriter};
use std::io::Write;

pub fn create(writer: &mut VideoWriter) -> opencv::Result<()> {
    // Initial bateman start (4.71s)
    println!("Writing intro clip...");
    let mut bateman_clip: VideoCapture = VideoCapture::from_file("res/video/bg/intro.mp4", videoio::CAP_ANY)?;
    for _ in 0..(30. * 5.3) as i32 {
        let mut frame: Mat = Mat::default();
        bateman_clip.read(&mut frame)?;

        writer.write(&frame)?;
    }

    // Backgrounds
    let bgs: &[Mat] = &[
        opencv::imgcodecs::imread("res/video/bg/0.png", opencv::imgcodecs::ImreadModes::IMREAD_COLOR as i32)?,
        opencv::imgcodecs::imread("res/video/bg/1.png", opencv::imgcodecs::ImreadModes::IMREAD_COLOR as i32)?,
        opencv::imgcodecs::imread("res/video/bg/2.png", opencv::imgcodecs::ImreadModes::IMREAD_COLOR as i32)?,
    ];

    for i in 0..3 {
        print!("\rWriting image clip {}...", i + 1);
        std::io::stdout().flush().unwrap();

        for j in 0..(30. * 2.2) as i32 {
            let progress: f32 = j as f32 / (30. * 2.);
            let adjusted: Mat = effects::shift(&bgs[i],
                (25. * f32::exp(-10. * progress) * f32::cos(1.5 * j as f32 + 0.5)) as i32,
                (25. * f32::exp(-10. * progress) * f32::sin(2. * j as f32)) as i32
            );

            writer.write(&adjusted)?;
        }
    }

    println!();
    Ok(())
}