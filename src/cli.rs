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

    #[structopt(name = "lang", about = "check installed programming languages")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Lang {
        #[structopt(long)]
        language: Option<String>,
    },

    #[structopt(name = "timer", about = "countdown timer")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Timer {
        #[structopt(long)]
        time: usize,
    },
}
