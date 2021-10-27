//! Command line parser.
//!
//! ### Usage
//!
//! ```rust
//! let parse = Parse::default();
//! parse.call()
//! ```

use crate::cli::{Cli, Sub, Tracking};
use crate::core::{languages, nyancat, timer, tracker};
use structopt::StructOpt;

pub struct Parse {
    cli: Cli,
}

impl Default for Parse {
    /// Create a default struct.
    fn default() -> Self {
        let cli = Cli::from_args();

        Self { cli: cli }
    }
}

impl Parse {
    /// Call functions.
    pub fn call(&self) {
        match &self.cli.sub {
            Sub::NyanCat => nyancat::nyancat().unwrap(),
            Sub::Lang { language } => {
                if let Some(lang) = language {
                    languages::selected_languages(&lang);
                } else {
                    languages::languages()
                }
            }
            Sub::Timer { time } => timer::timer(&time).unwrap(),
            Sub::Tracking { sub } => {
                let tracker = tracker::Tracker::default();
                match sub {
                    Tracking::Create { url } => {
                        tracker.create(url).unwrap();
                    }
                    Tracking::Delete { id } => {
                        tracker.delete(id).unwrap();
                    }
                    Tracking::History { id } => {
                        tracker.history(id).unwrap();
                    }
                    Tracking::List => {
                        tracker.list().unwrap();
                    }
                }
            }
        }
    }
}
