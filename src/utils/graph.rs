use chrono::{DateTime, Datelike, FixedOffset};
use std::error::Error;
use termion::{color, terminal_size};

const WIDTH: f32 = 0.5;

pub fn plot_barchart(data: Vec<(DateTime<FixedOffset>, usize)>) -> Result<(), Box<dyn Error>> {
    let max = data.iter().max_by(|x, y| x.1.cmp(&y.1));
    let term_size = terminal_size()?;

    if let Some(max) = max {
        for (day, accesses) in data.iter() {
            println!(
                "{}{}/{}{} | {}\t| {}{}{}",
                color::Fg(color::Magenta),
                day.month(),
                day.day(),
                color::Fg(color::Reset),
                accesses,
                color::Fg(color::LightGreen),
                cal_bar(&(*accesses as f32), &(max.1 as f32), &(term_size.0 as f32)),
                color::Fg(color::Reset)
            );
        }
    }

    Ok(())
}

fn cal_bar(target: &f32, max: &f32, term_size: &f32) -> String {
    let size: usize = ((target / max) * (term_size * WIDTH)) as usize;

    let mut buffer: Vec<&str> = vec![];

    for _ in 0..size {
        buffer.push("■");
    }

    buffer.join("")
}
