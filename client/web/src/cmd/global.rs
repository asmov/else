use crate::{error::*, cmd::Cli};

#[derive(Debug, PartialEq, Eq)]
pub struct GoCmd<'input> {
    destination: &'input str 
}

impl<'input> GoCmd<'input> {
    pub const NAME: &'static str = "go";
    pub const USAGE: &'static str = r"Usage: go <destination>
    Moves to the specified route or area.

    <destination> := The name or key of a nearby route, endpoint, or adjoining area.";

    pub fn new(destination: &'input str) -> GoCmd {
        Self {
            destination
        }
    }
}

impl<'input> Cli<'input> for GoCmd<'input> {
    fn name() -> &'static str {
        Self::NAME
    }

    fn usage() -> &'static str {
        Self::USAGE
    }

    fn parse(args: &Vec<&'input str>) -> Result<Self> {
        Self::check_help(args)?;
        Self::check_len(args, 2)?;
       
        Ok(Self {
            destination: args.get(1).unwrap()
        })
    }
}