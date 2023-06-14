use crate::effects;
use crate::common::print_progress;
use opencv::{prelude::*, videoio};
use opencv::videoio::{VideoCapture, VideoWriter};
use rand::Rng;

#[derive(Clone)]
enum Topic<'a> {
    Intro {
        person_a: &'a str,
        person_b: &'a str
    },
    Skill {
        // Used to rig ties
        person: &'a str,
        name: &'a str
    },
    Better {
        person: &'a str,
        score_a: i32,
        score_b: i32
    },
    Winner {
        person: &'a str
    }
}

fn random_skill<'a>(skills: &mut Vec<&'a str>) -> &'a str {
    let index: usize = rand::thread_rng().gen_range(0..skills.len());
    let skill: &str = skills[index];
    skills.remove(index);
    skill
}

fn random_skill_from_two<'a>(skills_a: &mut Vec<&'a str>, skills_b: &mut Vec<&'a str>, person_a: &'a str, person_b: &'a str) -> (&'a str, &'a str) {
    if skills_a.is_empty() {
        (random_skill(skills_b), person_b)
    } else if skills_b.is_empty() {
        (random_skill(skills_a), person_a)
    } else {
        if rand::thread_rng().gen_bool(0.5) {
            (random_skill(skills_a), person_a)
        } else {
            (random_skill(skills_b), person_b)
        }
    }
}

pub fn create(writer: &mut VideoWriter, beats: &[f32], mut combined: VideoCapture, mut big_a: VideoCapture, mut big_b: VideoCapture, rig_ties: bool, probability: f32) -> opencv::Result<()> {
    let person_a: &str = "Patrick Bateman";
    let person_b: &str = "Thomas Shelby";
    let mut score_a: i32 = 0;
    let mut score_b: i32 = 0;

    let mut topic: Topic = Topic::Intro { person_a, person_b };
    let mut skills: Vec<&str> = vec![
        "IQ", "BATTLE IQ", "AGILITY", "STRENGTH", "ENDURANCE", "SPEED", "EXPERIENCE",
        "SKILL", "WEAPONS", "POWER", "COMBAT", "STAMINA", "FEATS", "DEFENSE"
    ];

    let mut skills_a: Vec<&str> = Vec::new();
    let mut skills_b: Vec<&str> = Vec::new();

    if rig_ties {
        for _ in 0..(skills.len() / 2) {
            skills_a.push(random_skill(&mut skills));
            skills_b.push(random_skill(&mut skills));
        }
    } else {
        for _ in 0..skills.len() {
            if rand::thread_rng().gen_bool(probability as f64) {
                skills_a.push(random_skill(&mut skills));
            } else {
                skills_b.push(random_skill(&mut skills));
            }
        }
    }

    for i in 1..beats.len() {
        print_progress(i + 1, beats.len());
        let video: &mut VideoCapture = match topic {
            Topic::Intro {..} |
            Topic::Skill {..} => &mut combined,
            Topic::Better { person, .. } |
            Topic::Winner { person } => if person == person_a { &mut big_a } else if person == person_b { &mut big_b } else { &mut combined }
        };
        write_beat_interval(writer, video, beats[i] - beats[i - 1], topic.clone())?;

        topic = match topic {
            Topic::Intro {..} => {
                let (skill, name) = random_skill_from_two(&mut skills_a, &mut skills_b, person_a, person_b);
                Topic::Skill { person: name, name: skill }
            },
            Topic::Skill { person, .. } => {
                if person == person_a {
                    score_a += 1;
                } else {
                    score_b += 1;
                }

                Topic::Better { person, score_a, score_b }
            },
            Topic::Better {..} => if skills_a.is_empty() && skills_b.is_empty() {
                Topic::Winner { person: if score_a > score_b { person_a } else if score_a < score_b { person_b } else { "TIE" } }
            } else {
                let (skill, name) = random_skill_from_two(&mut skills_a, &mut skills_b, person_a, person_b);
                Topic::Skill { person: name, name: skill }
            },
            Topic::Winner {..} => topic
        };
    }
    println!();

    Ok(())
}

fn write_beat_interval(writer: &mut VideoWriter, video: &mut VideoCapture, beat_len: f32, topic: Topic) -> opencv::Result<()> {
    if !matches!(topic, Topic::Winner {..}) {
        video.set(videoio::CAP_PROP_POS_FRAMES, 0.)?;
    }
    let frames: usize = (30. * beat_len) as usize;

    for i in 0..frames {
        let progress: f32 = i as f32 / frames as f32;

        let mut frame: Mat = Mat::default();
        video.read(&mut frame)?;

        let mut adjusted: Mat = frame.clone();

        if !matches!(topic, Topic::Winner {..}) {
            adjusted = effects::shift(&frame,
                (25. * f32::exp(-10. * progress) * f32::cos(1.5 * i as f32 + 0.5)) as i32,
                (25. * f32::exp(-10. * progress) * f32::sin(2. * i as f32)) as i32
            );
        }

        let text: String = match topic {
            Topic::Intro { person_a, person_b } => format!("{}\nVS\n{}", person_a, person_b),
            Topic::Skill { person: _, name } => name.to_string(),
            Topic::Better { person: _, score_a, score_b } => format!("{}-{}", score_a, score_b),
            Topic::Winner { person } => if person == "TIE" { "TIE".to_string()  } else { format!("{} wins", person) }
        };

        let mut y: i32 = -20;
        for line in text.split('\n') {
            y += 50;
            effects::draw_text(&mut adjusted, line, None, Some(y), 2., 3)?;
        }


        writer.write(&adjusted)?;
    }

    Ok(())
}

