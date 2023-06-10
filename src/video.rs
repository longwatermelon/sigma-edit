use opencv::{prelude::*, core, videoio, Result};
use opencv::videoio::{VideoCapture, VideoWriter};
use opencv::core::Scalar;
use opencv::imgproc::warp_affine;
use rand::Rng;

pub fn create(input: &str, output: &str, beats: &[f32], cuts: &[f32]) -> Result<()> {
    let mut video: VideoCapture = VideoCapture::from_file(input, videoio::CAP_ANY)?; // 0 is the default camera
    let w: i32 = video.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let h: i32 = video.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
    let mut out = VideoWriter::new(output,
        VideoWriter::fourcc('m', 'p', '4', 'v')?, 30.,
        core::Size_ { width: w, height: h }, true
    )?;

    for i in 1..beats.len() {
        println!("Writing beat interval {:.2} to {:.2}...", beats[i - 1], beats[i]);
        write_beat_interval(&mut out, &mut video, beats[i] - beats[i - 1], cuts)?;
    }

    out.release()?;
    Ok(())
}

fn shift(frame: &Mat, xshift: i32, yshift: i32) -> Mat {
    let shift_matrix = Mat::from_slice_2d(&[
        &[1.0, 0.0, xshift as f64],
        &[0.0, 1.0, yshift as f64]
    ]).unwrap();

    let mut shifted_image = frame.clone();
    warp_affine(
        frame,
        &mut shifted_image,
        &shift_matrix,
        frame.size().unwrap(),
        opencv::imgproc::INTER_LINEAR,
        opencv::core::BORDER_CONSTANT,
        Scalar::new(0., 0., 0., 0.),
    ).unwrap();

    shifted_image
}

fn write_beat_interval(writer: &mut VideoWriter, video: &mut VideoCapture, beat_len: f32, cuts: &[f32]) -> Result<()> {
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
        video.read(&mut frame)?;
        let adjusted: Mat = shift(&frame,
            (25. * f32::exp(-10. * progress) * f32::cos(1.5 * i as f32 + 0.5)) as i32,
            (25. * f32::exp(-10. * progress) * f32::sin(2. * i as f32)) as i32
        );

        writer.write(&adjusted)?;
    }

    Ok(())
}

