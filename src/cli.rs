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

    #[structopt(name = "track", about = "tracking")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Tracking {
        #[structopt(subcommand)]
        sub: Tracking,
    },
}

#[derive(StructOpt)]
pub enum Tracking {
    #[structopt(name = "create", about = "Create tracking link")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Create {
        #[structopt(long)]
        url: String,
    },

    #[structopt(name = "delete", about = "Delete tracking link and access history")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    Delete {
        #[structopt(long)]
        id: String,
    },

    #[structopt(name = "list", about = "List all tracking links")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    List,

    #[structopt(name = "history", about = "Show access history")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    History {
        #[structopt(long)]
        id: String,
    },
}
