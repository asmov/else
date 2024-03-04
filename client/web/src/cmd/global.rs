use crate::{cmd::Cli, error::*, input::*, target::*};
use asmov_else_model as model;
use model::Identifiable;


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
    pub subject: Option<String>,
    pub processed: Option<ProcessedLookCmd>
}

#[derive(Debug, PartialEq, Eq)]
pub struct ProcessedLookCmd {
    pub subject: Target
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
        let subject = if let Some(subject) = &self.subject {
            Target::find(subject, interface_view, TargetType::all(), "Destination")?
        } else {
            Target::Area(interface_view.world_view().area_view().uid())
        };

        self.processed = Some(ProcessedLookCmd { subject });
        Ok(())
    }
    
    fn processed(&self) -> Option<&Self::ProcessedCmdType> {
        self.processed.as_ref()
    }
}