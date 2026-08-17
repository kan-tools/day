use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about = "Private repository-development tasks for day")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Xtask,
}

impl Cli {
    pub fn root(&self) -> Result<PathBuf, String> {
        std::env::current_dir()
            .map_err(|error| format!("could not determine the caller's working directory: {error}"))
    }
}

#[derive(Debug, Subcommand)]
pub enum Xtask {
    Validate {
        #[command(subcommand)]
        command: ValidateCommand,
    },
    Evidence {
        #[command(subcommand)]
        command: EvidenceCommand,
    },
    Census {
        #[command(subcommand)]
        command: CensusCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum ValidateCommand {
    Profile {
        name: String,
        #[arg(long)]
        list: bool,
    },
    Rfc {
        #[arg(long)]
        self_test: bool,
    },
    Publication {
        #[arg(long, default_value_t = 0)]
        rfc: u32,
    },
    Vectors {
        path: Option<PathBuf>,
        #[arg(long)]
        self_test: bool,
    },
    Formal {
        #[arg(long)]
        self_test: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum EvidenceCommand {
    BehaviourDiff(TrailingArgs),
    Mutate(TrailingArgs),
    Revert(TrailingArgs),
}

#[derive(Debug, Subcommand)]
pub enum CensusCommand {
    Demonstrations(TrailingArgs),
    Findings(TrailingArgs),
}

#[derive(Debug, Args)]
pub struct TrailingArgs {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub args: Vec<OsString>,
}
