pub mod global;

//use shellwords;
use crate::{error::*, cmd::global::*, input};


#[derive(Debug, PartialEq, Eq)]
pub enum Cmd<'input> {
    Go(GoCmd<'input>),
}

const SHORT_HELP: &'static str = "-h";
const LONG_HELP: &'static str = "--help";

/// Standard methods for a CLI command
pub trait Cli<'input> where Self: Sized {
    /// The name of the command
    fn name() -> &'static str;
    /// The usage / help string
    fn usage() -> &'static str;
    /// Parses command-line input into a command struct
    fn parse(args: &Vec<&'input str>) -> Result<Self>;

    /// Checks whether "-h" or "--help" was specified and returns a CommandUsage error if so.
    fn check_help(args: &Vec<&'input str>) -> Result<()> {
        if args.len() == 2 {
            match *args.get(1).unwrap() {
                SHORT_HELP | LONG_HELP => Err(Error::CommandUsage{usage: Self::usage()}),
                _ => Ok(())
            }
        } else {
            Ok(())
        }
    }

    /// Ensures that the number of arguments matches the specified size. Returns a CommandUsage error otherwise.
    fn check_len(args: &Vec<&'input str>, len: usize) -> Result<()> {
        if args.len() != len {
            return Err(Error::CommandUsage{usage: Self::usage()})
        } else {
            Ok(())
        }
    }
}

impl<'input> Cmd<'input> {
    pub fn parse(text: &'input str) -> Result<Self> {
        let args = input::split_tokens(text)
            .map_err(|_| Error::GenericInputParsing(text.to_string()))?;

        let cmdname = args.get(0)
            .ok_or_else(|| Error::GenericInputParsing(text.to_string()))?;
        
        let cmd = match *cmdname {
            GoCmd::NAME => Cmd::Go(GoCmd::parse(&args)?),
            _ => return Err(Error::UnknownCommand(text.to_owned()))
        };

        Ok(cmd)
    }
}