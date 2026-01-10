use Subcommand::*;
use errgonomic::map_err;
use thiserror::Error;

#[derive(clap::Parser, Debug)]
#[command(author, version, about)]
pub struct Command {
    #[command(subcommand)]
    command: Subcommand,
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum Subcommand {
    Print(PrintCommand),
}

impl Command {
    pub async fn run(self) -> Result<(), CommandRunError> {
        use CommandRunError::*;
        let Self {
            command,
        } = self;
        match command {
            Print(command) => map_err!(command.run().await, PrintCommandRunFailed),
        }
    }
}

#[derive(Error, Debug)]
pub enum CommandRunError {
    #[error("failed to run print command")]
    PrintCommandRunFailed { source: PrintCommandRunError },
}

mod print_command;

pub use print_command::*;
