mod cli;
mod parse;

use crate::parse::Parse;

fn main() {
    let parse = Parse::default();

    parse.call();
}
