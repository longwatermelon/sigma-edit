use opencv::{prelude::*, videoio};
use opencv::videoio::VideoCapture;
use opencv::core::Scalar;
use opencv::imgproc::warp_affine;

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

