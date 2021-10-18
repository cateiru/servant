mod cli;
mod core;
mod parse;

use parse::Parse;

fn main() {
    let parse = Parse::default();

    parse.call();
}
