use structopt::{clap, StructOpt};

#[derive(StructOpt)]
#[structopt(name = "servant", about = "servant is utils cli.")]
#[structopt(setting(clap::AppSettings::ColoredHelp))]
pub struct Cli {
    #[structopt(subcommand)]
    /// servant is a (planned) cli that can do anything.
    /// See https://github.com/yuto51942/servant#readme for usage or --help for help.
    pub sub: Sub,
}

#[derive(StructOpt)]
pub enum Sub {
    #[structopt(name = "nyancat", about = "nyanyanyanyanya")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    /// Show nyancat animation.
    /// It ends with ^c.
    ///
    /// nyancat see more: https://www.nyan.cat/
    NyanCat,

    #[structopt(name = "lang", about = "check installed programming languages")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    /// Displays version information of the programming language installed on the device.
    /// You can use the --language flag to display version information for the specified programming language.
    Lang {
        #[structopt(long)]
        language: Option<String>,
    },

    #[structopt(name = "timer", about = "countdown timer")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    /// It is a countdown timer.
    Timer {
        #[structopt(long)]
        time: usize,
    },

    #[structopt(name = "track", about = "tracking")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    /// Create and manage URLs that allow access tracking.
    /// Inspired by https://gigazine.net/news/20140902-line-hijacker-track/
    ///
    /// When you access the generated URL, the IP address of the access source is saved.
    /// server side source code: https://github.com/yuto51942/access-tracker
    Tracking {
        #[structopt(subcommand)]
        sub: Tracking,
    },

    #[structopt(name = "bench", about = "bench mark")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    /// Benchmark
    ///
    /// Calculate recursive Fibonacci.
    Bench,

    #[structopt(name = "emoji", about = "emoji search")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    /// Emoji searcher.
    Emoji {
        #[structopt(long)]
        query: String,
    },
}

#[derive(StructOpt)]
pub enum Tracking {
    #[structopt(name = "create", about = "Create tracking link")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    /// Create tracking link.
    ///
    /// You can check the created url in `list`.
    /// and can show history in `history`.
    Create {
        #[structopt(long)]
        url: String,
    },

    #[structopt(name = "delete", about = "Delete tracking link and access history")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    /// Delete the tracking URL you created.
    Delete {
        #[structopt(long)]
        id: String,
    },

    #[structopt(name = "list", about = "List all tracking links")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    /// Display the list of created tracking URLs.
    List,

    #[structopt(name = "history", about = "Show access history")]
    #[structopt(setting(clap::AppSettings::ColoredHelp))]
    /// Check the access history of the tracking URL.
    History {
        #[structopt(long)]
        id: String,

        /// Export oneline
        #[structopt(long)]
        oneline: bool,

        /// Show all ip
        #[structopt(long)]
        all: bool,
    },
}
