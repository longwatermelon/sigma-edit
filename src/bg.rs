use crate::effects;
use opencv::{prelude::*, videoio};
use opencv::videoio::{VideoCapture, VideoWriter};
use rand::seq::SliceRandom;
use std::io::Write;

const LAST_BG: i32 = 3;

pub fn create(writer: &mut VideoWriter) -> opencv::Result<()> {
    // Initial bateman start (5.4s)
    print!("Writing intro clip...");
    std::io::stdout().flush().unwrap();
    let mut bateman_clip: VideoCapture = VideoCapture::from_file("res/video/bg/intro.mp4", videoio::CAP_ANY)?;
    for _ in 0..(30. * 5.4) as i32 {
        let mut frame: Mat = Mat::default();
        bateman_clip.read(&mut frame)?;

        writer.write(&frame)?;
    }

    // Backgrounds
    let mut bgs: Vec<i32> = (0..=LAST_BG).collect();
    bgs.shuffle(&mut rand::thread_rng());

    fn imread(i: i32) -> Mat {
        opencv::imgcodecs::imread(
                format!("res/video/bg/{}.png", i).as_str(),
                opencv::imgcodecs::ImreadModes::IMREAD_COLOR as i32
        ).unwrap()
    }

    let bgs: &[Mat] = &[imread(bgs[0]), imread(bgs[1]), imread(bgs[2])];

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