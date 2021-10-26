use crate::utils::print;
use std::{error::Error, thread::sleep, time::Duration};

pub fn timer(time: &usize) -> Result<(), Box<dyn Error>> {
    for i in 1..=*time {
        for i_min in 1..=10 {
            print::print_line(&format!("\rTime: {}.{:1}   ", time - i, 10 - i_min))?;
            sleep(Duration::from_millis(100));
        }
    }

    println!("\rEnd time.     ");

    Ok(())
}
