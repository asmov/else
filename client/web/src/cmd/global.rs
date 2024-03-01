use crate::{cmd::Cli, error::*, input::TextInput};

#[derive(Debug, PartialEq, Eq)]
pub struct GoCmd {
    destination: String 
}

impl GoCmd {
    pub fn new(destination: String) -> Self {
        Self { destination }
    }
}

impl Cli for GoCmd {
    const NAME: &'static str = "go";
    const USAGE: &'static str = r"Usage: go <destination>
    Moves to the specified route or area.

    <destination> := The name or key of a nearby route, endpoint, or adjoining area.";

    fn parse(input: &TextInput) -> Result<Self> {
        Self::check_help(input)?;
        Self::check_num_args(input, 1)?;
        let destination = input.args().get(1).unwrap().to_owned();
        Ok(Self { destination })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LookCmd {
    subject: Option<String>
}

impl LookCmd {
    pub fn new(subject: Option<String>) -> Self {
        Self { subject }
    }
}

impl Cli for LookCmd {
    const NAME: &'static str = "look";
    const USAGE: &'static str = r"Usage: look [subject]
    Looks at the specified subject. If omitted, looks at the current area. 

    [subject] := The name or key of a nearby thing or route.";

    fn parse(input: &TextInput) -> Result<Self> {
        Self::check_help(input)?;
        Self::check_num_args_max(input, 1)?;
        let subject = input.args().get(1).map(|s| s.to_owned());
        Ok(Self { subject })
    }
}