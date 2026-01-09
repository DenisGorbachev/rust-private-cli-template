use Subcommand::*;
use clap::Parser;
use errgonomic::map_err;
use thiserror::Error;

mod print_command;

pub use print_command::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Command {
    // #[arg(short, long, value_parser = value_parser!(PathBuf))]
    // root: Option<PathBuf>,
    #[command(subcommand)]
    command: Subcommand,
}

impl Command {
    pub async fn run(self) -> Result<(), CommandRunError> {
        use CommandRunError::*;
        let Self {
            command,
        } = self;
        map_err!(command.run().await, SubcommandRunFailed)
    }
}

#[derive(Error, Debug)]
pub enum CommandRunError {
    #[error("failed to run command")]
    SubcommandRunFailed { source: SubcommandRunError },
}

#[derive(Parser, Clone, Debug)]
pub enum Subcommand {
    Print(PrintCommand),
}

impl Subcommand {
    pub async fn run(self) -> Result<(), SubcommandRunError> {
        use SubcommandRunError::*;
        match self {
            Print(command) => map_err!(command.run().await, PrintCommandRunFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum SubcommandRunError {
    #[error("failed to run print command")]
    PrintCommandRunFailed { source: PrintCommandRunError },
}
