use std::{
    error::Error,
    io::{stdout, Write},
};

pub fn print_line(text: &str) -> Result<(), Box<dyn Error>> {
    stdout().write_all(text.as_bytes())?;

    stdout().flush()?;

    Ok(())
}
