use chrono::{DateTime, Datelike, Duration};
use chrono_tz::Tz;
use std::{error::Error, ops::Sub};
use termion::{color, terminal_size};

const WIDTH: f32 = 0.5;

pub fn plot_barchart(data: Vec<(DateTime<Tz>, usize)>) -> Result<(), Box<dyn Error>> {
    let max = data.iter().max_by(|x, y| x.1.cmp(&y.1));
    let term_size = terminal_size()?;

    if let Some(max) = max {
        let max_index = data.len();

        if max_index != 0 {
            let mut current_day = data[0].0;
            let mut current_access = data[0].1;

            let mut index = 0;

            while index + 1 != max_index {
                let (next_day, next_access) = data[index + 1];

                print(
                    &current_day,
                    &current_access,
                    term_size.0 as f32,
                    max.1 as f32,
                );

                if check_consecutive_dates(&current_day, &next_day) {
                    current_day = next_day;
                    current_access = next_access;
                } else {
                    // trackが0の日もprintする
                    while !check_consecutive_dates(&current_day, &next_day) {
                        current_day = current_day.sub(Duration::days(1));
                        print(&current_day, &0, term_size.0 as f32, max.1 as f32);
                    }
                    current_day = next_day;
                    current_access = next_access;
                }

                index += 1;
            }

            // 最後の行の出力
            print(
                &data[max_index - 1].0,
                &data[max_index - 1].1,
                term_size.0 as f32,
                max.1 as f32,
            );
        }
    }

    Ok(())
}

fn print(date: &DateTime<Tz>, access: &usize, term_size: f32, max: f32) {
    println!(
        "{}{}/{}{}\t| {}\t| {}{}{}",
        color::Fg(color::Magenta),
        date.month(),
        date.day(),
        color::Fg(color::Reset),
        access,
        color::Fg(color::LightGreen),
        cal_bar(&(*access as f32), &max, &(term_size)),
        color::Fg(color::Reset)
    );
}

fn cal_bar(target: &f32, max: &f32, term_size: &f32) -> String {
    let size: usize = ((target / max) * (term_size * WIDTH)) as usize;

    let mut buffer: Vec<&str> = vec![];

    for _ in 0..size {
        buffer.push("■");
    }

    buffer.join("")
}

fn check_consecutive_dates(current: &DateTime<Tz>, next: &DateTime<Tz>) -> bool {
    current.sub(Duration::days(1)).day() == next.day()
}
