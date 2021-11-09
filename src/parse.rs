//! Command line parser.
//!
//! ### Usage
//!
//! ```rust
//! let parse = Parse::default();
//! parse.call()
//! ```

use crate::cli::{Cli, Sub, Tracking};
use crate::core::{bench, emoji_search, languages, nyancat, timer, tracker};
use std::{env, error::Error, fs::create_dir_all, path::Path};
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
    pub fn call(&self) -> Result<(), Box<dyn Error>> {
        self.dir()?;

        match &self.cli.sub {
            Sub::NyanCat => nyancat::nyancat().unwrap(),
            Sub::Lang { language } => {
                if let Some(lang) = language {
                    languages::selected_languages(&lang);
                } else {
                    languages::languages();
                }
            }
            Sub::Timer { time } => timer::timer(&time).unwrap(),
            Sub::Tracking { sub } => {
                let _home = env::var("HOME");
                let path_str: String;
                if let Ok(home) = _home {
                    path_str = format!("{}/.servant/tracker", home);
                } else {
                    path_str = ".servant/tracker".to_string();
                }

                let path = &Path::new(&path_str);

                let tracker = tracker::Tracker::new(path);

                match sub {
                    Tracking::Create { url } => {
                        tracker.create(url)?;
                    }
                    Tracking::Delete { id } => {
                        tracker.delete(id)?;
                    }
                    Tracking::History {
                        id,
                        oneline,
                        all,
                        graph,
                    } => {
                        tracker.history(id, *oneline, *all, *graph)?;
                    }
                    Tracking::List => {
                        tracker.list()?;
                    }
                }
            }

            Sub::Bench => {
                bench::bench();
            }

            Sub::Emoji { query } => {
                emoji_search::emoji(query.to_string())?;
            }
        };

        Ok(())
    }

    /// Create cache dir
    /// dir name of `.servant`
    ///
    /// If the `HOME` environment variable is set, create it on that path.
    fn dir(&self) -> Result<(), Box<dyn Error>> {
        let _home = env::var("HOME");
        let path_str: String;
        if let Ok(home) = _home {
            path_str = format!("{}/.servant", home);
        } else {
            path_str = ".servant".to_string();
        }

        let path = &Path::new(&path_str);

        if !path.is_dir() {
            create_dir_all(path)?;
        }
        Ok(())
    }
}
