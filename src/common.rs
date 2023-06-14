use std::{io, io::Write};

pub fn print_progress(beat_index: usize, nbeats: usize) {
    print!("\rWriting beat interval {}/{}...", beat_index, nbeats);
    io::stdout().flush().unwrap();
}
