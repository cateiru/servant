use crate::cli::{Cli, Sub};
use structopt::StructOpt;

pub struct Parse {
    cli: Cli,
}

impl Default for Parse {
    fn default() -> Self {
        let cli = Cli::from_args();

        Self { cli: cli }
    }
}

impl Parse {
    pub fn call(&self) {
        match self.cli.sub {
            Sub::NyanCat => {
                println!("TODO: nyan cat.")
            }
        }
    }
}
