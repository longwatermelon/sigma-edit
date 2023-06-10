use opencv::{prelude::*, core, videoio, Result};
use opencv::videoio::{VideoCapture, VideoWriter};
use opencv::core::{Scalar, MatTrait};
use opencv::imgproc::{cvt_color, COLOR_BGR2HSV, COLOR_HSV2BGR, warp_affine};
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

fn saturate(frame: &Mat, saturation_factor: f32) -> Mat {
    let mut hsv_image = Mat::default();
    cvt_color(frame, &mut hsv_image, COLOR_BGR2HSV, 0).unwrap();

    for y in 0..hsv_image.rows() {
        for x in 0..hsv_image.cols() {
            let pixel = hsv_image.at_2d_mut::<opencv::core::Vec3b>(y, x).unwrap();
            let saturation = pixel[1] as i16;
            let adjusted_saturation = (saturation as f32 * saturation_factor) as i16;
            pixel[1] = adjusted_saturation.max(0).min(255) as u8;
        }
    }

    let mut output_image = Mat::default();
    cvt_color(&hsv_image, &mut output_image, COLOR_HSV2BGR, 0).unwrap();
    output_image
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

    let mut saturation: f32 = 3.;

    video.set(videoio::CAP_PROP_POS_FRAMES, begin as f64)?;
    for i in 0..frames {
        let progress: f32 = i as f32 / frames as f32;

        let mut frame: Mat = Mat::default();
        video.read(&mut frame)?;
        let adjusted: Mat = shift(&saturate(&frame, saturation),
            (25. * f32::exp(-10. * progress) * f32::cos(1.5 * i as f32 + 0.5)) as i32,
            (25. * f32::exp(-10. * progress) * f32::sin(2. * i as f32)) as i32
        );

        saturation = 1. + 2. * f32::exp(-(2. * i as f32) / (frames as f32));

        writer.write(&adjusted)?;
    }

    Ok(())
}

