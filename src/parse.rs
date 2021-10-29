//! Command line parser.
//!
//! ### Usage
//!
//! ```rust
//! let parse = Parse::default();
//! parse.call()
//! ```

use crate::cli::{Cli, Sub, Tracking};
use crate::core::{bench, languages, nyancat, timer, tracker};
use std::{env, path::Path};
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
                let _home = env::var("HOME");
                let path: String;
                if let Ok(home) = _home {
                    path = format!("{}/.servant_tracker", home);
                } else {
                    path = ".servant_tracker".to_string();
                }
                let tracker = tracker::Tracker::new(&Path::new(&path));

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

            Sub::Bench => {
                bench::bench();
            }
        }
    }
}
