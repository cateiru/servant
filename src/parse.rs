//! Command line parser.
//!
//! ### Usage
//!
//! ```rust
//! let parse = Parse::default();
//! parse.call()
//! ```

use crate::cli::{Cli, Sub};
use crate::core::{languages, nyancat};
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
            Sub::NyanCat => nyancat::nyancat(),
            Sub::Lang { language } => {
                if let Some(lang) = language {
                    languages::selected_languages(&lang);
                } else {
                    languages::languages()
                }
            }
        }
    }
}
