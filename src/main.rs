mod cli;
mod core;
mod parse;
mod utils;

use parse::Parse;

fn main() {
    let parse = Parse::default();

    parse.call();
}
