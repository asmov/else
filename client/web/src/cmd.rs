pub mod go;
pub mod look;

use asmov_else_model as model;
use crate::{error::*, input::*, ui::app::*};
pub use crate::cmd::{go::GoCmd, look::LookCmd};

#[derive(Debug, PartialEq, Eq)]
pub enum Cmd {
    Go(GoCmd),
    Look(LookCmd)
}

pub trait AppCmd {
    fn run(self, app: &App) -> Result<Vec<AppAction>>;
}

impl Cmd {
    pub fn parse(input: &TextInput) -> Result<Self> {
        let cmdname = input.args().get(0)
            .ok_or_else(|| Error::GenericInputParsing(input.text().to_string()))?;
        
        let cmd = match cmdname.as_str() {
            GoCmd::NAME => Cmd::Go(GoCmd::parse(input)?),
            LookCmd::NAME => Cmd::Look(LookCmd::parse(input)?),
            _ => return Err(Error::UnknownCommand(input.text().to_string()))
        };

        Ok(cmd)
    }

    pub fn process(mut self, interface_view: &model::InterfaceView) -> Result<Self> {
        match &mut self {
            Cmd::Go(cmd) => cmd.process(interface_view)?,
            Cmd::Look(cmd) => cmd.process(interface_view)?
        };

        Ok(self)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Cmd::Go(_) => GoCmd::NAME,
            Cmd::Look(_) => LookCmd::NAME
        }
    }
}

const SHORT_HELP: &'static str = "-h";
const LONG_HELP: &'static str = "--help";

/// Standard methods for a CLI command
pub trait Cli where Self: Sized {
    type ProcessedCmdType: Sized;
    const NAME: &'static str;
    const USAGE: &'static str;

    /// Parses command-line input into a command struct
    fn parse(input: &TextInput) -> Result<Self>;

    /// Processes the raw values of a command into its final form
    fn process(&mut self, interface_view: &model::InterfaceView) -> Result<()>;

    fn processed(&self) -> Option<&Self::ProcessedCmdType>;

    fn is_processed(&self) -> bool {
        self.processed().is_some()
    }

    /// The name of the command
    fn name() -> &'static str { Self::NAME }
    /// The usage / help string
    fn usage() -> &'static str { Self::USAGE }

    /// Checks whether "-h" or "--help" was specified and returns a CommandUsage error if so.
    fn check_help(input: &TextInput) -> Result<()> {
        if input.args().len() == 2 {
            match input.args().get(1).unwrap().as_str() {
                SHORT_HELP | LONG_HELP => Err(Error::CommandUsage{usage: Self::usage()}),
                _ => Ok(())
            }
        } else {
            Ok(())
        }
    }

    /// Ensures that the number of arguments matches the specified size. Returns a CommandUsage error otherwise.
    fn check_num_args(input: &TextInput, num: usize) -> Result<()> {
        if input.args().len() != num+1 {
            return Err(Error::CommandUsage{usage: Self::usage()})
        } else {
            Ok(())
        }
    }

    fn check_num_args_max(input: &TextInput, max_num: usize) -> Result<()> {
        if input.args().len() > max_num+1 {
            return Err(Error::CommandUsage{usage: Self::usage()})
        } else {
            Ok(())
        }
    }
}

