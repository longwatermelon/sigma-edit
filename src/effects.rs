use opencv::{prelude::*, videoio};
use opencv::videoio::VideoCapture;
use opencv::core::{Point, Scalar};
use opencv::imgproc::{warp_affine, put_text};

pub fn shift(frame: &Mat, xshift: i32, yshift: i32) -> Mat {
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

pub fn slow(video: &mut VideoCapture, begin: i32, i: i32, frames: i32) -> opencv::Result<Mat> {
    video.set(videoio::CAP_PROP_POS_FRAMES, begin as f64 + 2. * (1. / (1. + f64::exp(-1.5 * i as f64 / frames as f64)) + 0.5) * frames as f64)?;
    let mut frame: Mat = Mat::default();
    video.read(&mut frame)?;
    Ok(frame)
}

pub fn draw_text(frame: &mut Mat, text: &str, x: Option<i32>, y: Option<i32>, font_scale: f64, thickness: i32) -> opencv::Result<()> {
    let text_size = opencv::imgproc::get_text_size(text, opencv::imgproc::FONT_HERSHEY_COMPLEX, font_scale, thickness, &mut 0)?;
    let mut xpos: i32 = (frame.cols() - text_size.width) / 2;
    let mut ypos: i32 = (frame.rows() - text_size.height) / 2;

    if let Some(x) = x {
        xpos += x;
    }

    if let Some(y) = y {
        ypos += y;
    }

    put_text(
        frame,
        text,
        Point::new(xpos, ypos),
        opencv::imgproc::FONT_HERSHEY_COMPLEX,
        font_scale,
        Scalar::new(0., 0., 0., 0.),
        thickness + 4,
        0,
        false
    )?;

    put_text(
        frame,
        text,
        Point::new(xpos, ypos),
        opencv::imgproc::FONT_HERSHEY_COMPLEX,
        font_scale,
        Scalar::new(255., 255., 255., 0.),
        thickness,
        0,
        false
    )?;

    Ok(())
}