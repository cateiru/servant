use structopt::{clap, StructOpt};

#[derive(StructOpt)]
#[structopt(name = "servant", about = "servant is utils cli.")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Cli {
    #[structopt(subcommand)]
    pub sub: Sub,
}

#[derive(StructOpt)]
pub enum Sub {
    #[structopt(name = "nyancat", about = "nyanyanyanyanya")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    NyanCat,
}
