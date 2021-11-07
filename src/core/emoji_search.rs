use emojis;
use std::error::Error;
use termion::color;

pub fn emoji(query: String) -> Result<(), Box<dyn Error>> {
    for element in emojis::search(&query) {
        println!(
            "\t{}\t : {}{}{}",
            element,
            color::Fg(color::LightGreen),
            element.name(),
            color::Fg(color::Reset)
        )
    }

    Ok(())
}
