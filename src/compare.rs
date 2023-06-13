use crate::effects;
use crate::video::print_progress;
use opencv::{prelude::*, videoio, Result};
use opencv::videoio::{VideoCapture, VideoWriter};
use opencv::core::{Scalar, Point};
use opencv::imgproc::put_text;
use rand::Rng;

#[derive(Clone)]
enum Topic<'a> {
    Intro {
        person_a: &'a str,
        person_b: &'a str
    },
    Skill {
        name: &'a str
    },
    Better {
        score_a: i32,
        score_b: i32
    },
    Winner {
        person: &'a str
    }
}

fn get_skill<'a>(skills: &mut Vec<&'a str>) -> &'a str {
    let skill: &str = skills[0];
    skills.remove(0);
    skill
}

pub fn create(writer: &mut VideoWriter, beats: &[f32], mut combined: VideoCapture, mut big_a: VideoCapture, mut big_b: VideoCapture) -> Result<()> {
    let person_a: &str = "Patrick Bateman";
    let person_b: &str = "Thomas Shelby";
    let mut score_a: i32 = 0;
    let mut score_b: i32 = 0;

    let mut topic: Topic = Topic::Intro { person_a, person_b };
    let mut skills: Vec<&str> = vec![
        "IQ", "BATTLE IQ", "AGILITY", "STRENGTH", "ENDURANCE", "SPEED", "EXPERIENCE",
        "TECH", "SKILL", "INTELLIGENCE", "WEAPONS", "FREE POINT"
    ];

    for i in 1..beats.len() {
        print_progress(i + 1, beats.len());
        let video: &mut VideoCapture = match topic {
            Topic::Intro {..} |
            Topic::Skill {..} => &mut combined,
            Topic::Better { score_a, score_b } => if score_a > score_b { &mut big_a } else { &mut big_b },
            Topic::Winner { person } => if person == person_a { &mut big_a } else { &mut big_b }
        };
        write_beat_interval(writer, video, beats[i] - beats[i - 1], topic.clone())?;

        topic = match topic {
            Topic::Intro {..} => Topic::Skill { name: get_skill(&mut skills) },
            Topic::Skill {..} => {
                let person: &str = if rand::thread_rng().gen_bool(0.5) { person_a } else { person_b };
                if person == person_a {
                    score_a += 1;
                } else {
                    score_b += 1;
                }

                Topic::Better { score_a, score_b }
            },
            Topic::Better {..} => if i >= beats.len() - 5 || skills.is_empty() {
                Topic::Winner { person: if score_a > score_b { person_a } else if score_a < score_b { person_b } else { "TIE" } }
            } else {
                Topic::Skill { name: get_skill(&mut skills) }
            },
            Topic::Winner {..} => break
        };
    }
    println!();

    Ok(())
}

fn write_beat_interval(writer: &mut VideoWriter, video: &mut VideoCapture, beat_len: f32, topic: Topic) -> Result<()> {
    video.set(videoio::CAP_PROP_POS_FRAMES, 0.)?;
    let frames: usize = (30. * beat_len) as usize;

    for i in 0..frames {
        let progress: f32 = i as f32 / frames as f32;

        let mut frame: Mat = Mat::default();
        video.read(&mut frame)?;

        let mut adjusted: Mat = effects::shift(&frame,
            (25. * f32::exp(-10. * progress) * f32::cos(1.5 * i as f32 + 0.5)) as i32,
            (25. * f32::exp(-10. * progress) * f32::sin(2. * i as f32)) as i32
        );

        let text: String = match topic {
            Topic::Intro { person_a, person_b } => format!("{}\nVS\n{}", person_a, person_b),
            Topic::Skill { name } => name.to_string(),
            Topic::Better { score_a, score_b } => format!("{}-{}", score_a, score_b),
            Topic::Winner { person } => if person == "TIE" { "TIE".to_string()  } else { format!("{} wins", person) }
        };

        let mut start_y: i32 = -20;
        for line in text.split('\n') {
            let font_scale = 2.;
            let thickness = 3;
            let font = opencv::imgproc::FONT_HERSHEY_COMPLEX;
            let linetype = opencv::imgproc::LINE_AA;
            let text_size = opencv::imgproc::get_text_size(line, font, font_scale, thickness, &mut 0)?;

            let x = (frame.cols() - text_size.width) / 2;
            let y = start_y + (frame.rows() - text_size.height) / 2;
            start_y += text_size.height + 10;

            put_text(
                &mut adjusted,
                line,
                Point::new(x - 1, y),
                font,
                font_scale,
                Scalar::new(0., 0., 0., 0.),
                thickness + 2,
                linetype,
                false
            )?;

            put_text(
                &mut adjusted,
                line,
                Point::new(x, y),
                font,
                font_scale,
                Scalar::new(255., 255., 255., 0.),
                thickness,
                linetype,
                false
            )?;
        }


        writer.write(&adjusted)?;
    }

    Ok(())
}

