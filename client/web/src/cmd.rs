pub mod global;

//use shellwords;
use crate::{error::*, cmd::global::*, input::TextInput};


#[derive(Debug, PartialEq, Eq)]
pub enum Cmd {
    Go(GoCmd),
}

const SHORT_HELP: &'static str = "-h";
const LONG_HELP: &'static str = "--help";

/// Standard methods for a CLI command
pub trait Cli where Self: Sized {
    /// The name of the command
    fn name() -> &'static str;
    /// The usage / help string
    fn usage() -> &'static str;
    /// Parses command-line input into a command struct
    fn parse(input: &TextInput) -> Result<Self>;

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
    fn check_len(input: &TextInput, len: usize) -> Result<()> {
        if input.args().len() != len {
            return Err(Error::CommandUsage{usage: Self::usage()})
        } else {
            Ok(())
        }
    }
}

impl Cmd {
    pub fn parse(input: &TextInput) -> Result<Self> {
        let cmdname = input.args().get(0)
            .ok_or_else(|| Error::GenericInputParsing(input.text().to_string()))?;
        
        let cmd = match cmdname.as_str() {
            GoCmd::NAME => Cmd::Go(GoCmd::parse(input)?),
            _ => return Err(Error::UnknownCommand(input.text().to_string()))
        };

        Ok(cmd)
    }
}