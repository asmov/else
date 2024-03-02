use std::borrow::Cow;

use crate::{cmd::Cli, error::*, input::TextInput};
use asmov_else_model as model;
use model::Routing;

#[derive(Debug, PartialEq, Eq)]
pub enum Target {
    None,
    Area(model::UID),
    Route(model::UID),
    Thing(model::UID),
}

/// Typical target strings:
/// - The keyword, name, or direction name of a nearby Route
/// - The keyword or name of a nearby Thing
/// Exact UIDs are also searched.
impl Target {
    /// Searches for all possible targets partially matching a string.
    /// Returns the target and its name for each match. 
    pub fn search<'w>(partial: &str, interface_view: &'w model::InterfaceView) -> Vec<(Cow<'w, str>, Self)> {
        let world_view = interface_view.world_view();
        let area_view = world_view.area_view();

        // UIDs must be exact
        if let Ok(uid) = partial.parse::<model::UID>() {
            if let Some(route_uid) = area_view.route_uids().iter().find(|id| id == &&uid) {
                let route_name = area_view.indexed_route_name(*route_uid, world_view).unwrap();
                return vec![(route_name, Self::Route(*route_uid))]; 
            } else if let Some(thing_uid) = area_view.occupant_uids().iter().find(|id| id == &&uid) {
                let thing_name = area_view.unique_thing_name(*thing_uid, world_view).unwrap();
                return vec![(thing_name, Self::Thing(*thing_uid))];
            } else {
                return Vec::new();
            }
        }



        Vec::new()
    }

    /// Finds a target by an attribute that is unique to it from the perspective of the world view
    /// If the attribute is not unique, an error is returned.
    pub fn find(unique: &str, interface_view: &model::InterfaceView, cmd_field: &'static str) -> Result<Self> {
        //todo
        Err(Error::TargetNotFound { target: cmd_field, search: unique.to_string() })
    }
}

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
        let target = Target::find(&self.destination, interface_view, "Destination")?;
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