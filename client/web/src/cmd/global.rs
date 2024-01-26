use crate::{cmd::Cli, error::*, input::TextInput};

#[derive(Debug, PartialEq, Eq)]
pub struct GoCmd {
    destination: String 
}

impl GoCmd {
    pub const NAME: &'static str = "go";
    pub const USAGE: &'static str = r"Usage: go <destination>
    Moves to the specified route or area.

    <destination> := The name or key of a nearby route, endpoint, or adjoining area.";

    pub fn new(destination: String) -> GoCmd {
        Self {
            destination
        }
    }
}

impl Cli for GoCmd {
    fn name() -> &'static str {
        Self::NAME
    }

    fn usage() -> &'static str {
        Self::USAGE
    }

    fn parse(input: &TextInput) -> Result<Self> {
        Self::check_help(input)?;
        Self::check_len(input, 2)?;

        let destination = input.args().get(1).unwrap().to_owned();
       
        Ok(Self {
            destination
        })
    }
}