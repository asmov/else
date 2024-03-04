use crate::{cmd::Cli, error::*, input::*, target::*};
use asmov_else_model as model;


#[derive(Debug, PartialEq, Eq)]
pub struct GoCmd {
    destination: String,
    processed: Option<ProcessedGoCmd>
}

impl GoCmd {
    pub fn new(destination: String) -> Self {
        Self { destination, processed: None }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProcessedGoCmd {
    destination: Target
}

impl Cli for GoCmd {
    type ProcessedCmdType = ProcessedGoCmd;
    const NAME: &'static str = "go";
    const USAGE: &'static str = r"Usage: go <destination>
    Moves to the specified route or area.

    <destination> := The name or key of a nearby route, endpoint, or adjoining area.";

    fn parse(input: &TextInput) -> Result<Self> {
        Self::check_help(input)?;
        Self::check_num_args(input, 1)?;
        let destination = input.args().get(1).unwrap().to_owned();
        Ok(Self::new(destination))
    }

    fn process(&mut self, interface_view: &model::InterfaceView) -> Result<()> {
        let target = Target::find(&self.destination, interface_view, &[TargetType::Route], "Destination")?;
        self.processed = Some(ProcessedGoCmd { destination: target });
        Ok(())
    }

    fn processed(&self) -> Option<&Self::ProcessedCmdType> {
        self.processed.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LookCmd {
    subject: Option<String>,
    processed: Option<ProcessedLookCmd>
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProcessedLookCmd {
    subject: Target
}

impl LookCmd {
    pub fn new(subject: Option<String>) -> Self {
        Self { subject, processed: None }
    }
}

impl Cli for LookCmd {
    type ProcessedCmdType = ProcessedLookCmd;
    const NAME: &'static str = "look";
    const USAGE: &'static str = r"Usage: look [subject]
    Looks at the specified subject. If omitted, looks at the current area. 

    [subject] := The name or key of a nearby thing or route.";

    fn parse(input: &TextInput) -> Result<Self> {
        Self::check_help(input)?;
        Self::check_num_args_max(input, 1)?;
        let subject = input.args().get(1).map(|s| s.to_owned());
        Ok(Self::new(subject))
    }
    
    
    fn process(&mut self, interface_view: &model::InterfaceView) -> Result<()> {
        todo!()
    }
    
    fn processed(&self) -> Option<&Self::ProcessedCmdType> {
        self.processed.as_ref()
    }
}