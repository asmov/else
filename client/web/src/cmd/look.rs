use model::{Descriptive, Identifiable};
use yew::AttrValue;

use crate::{error::*, target::*, cmd::*, input::*, ui::terminal::*};

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

impl AppCmd for LookCmd {
    fn run(self, app: &App) -> Result<Vec<AppAction>> {
        let world_view = app.interface_view().unwrap().world_view();
        let area_view = world_view.area_view();
        let area_uid = area_view.uid();

        let mut output: Vec<&str> = Vec::new();
        match self.processed.unwrap().subject {
            Target::Area(_area_uid) => {
                output.push(area_view.name());
                output.push(area_view.description().unwrap())
            },
            Target::Route(route_uid) => {
                let route = world_view.route(route_uid).unwrap();
                let end = route.end_for_area(area_uid).unwrap();
                output.push(end.name());
                output.push(end.description().unwrap());
            },
            Target::Thing(thing_uid) => {
                let thing_view = world_view.thing_view(thing_uid).unwrap();
                output.push(thing_view.name());
                output.push(thing_view.description().unwrap());
            },
        };

        let entries: Vec<(&str, EntryCategory)> = output.into_iter()
            .map(|s| (s, EntryCategory::Standard))
            .collect();

        Ok(vec![AppAction::new_terminal_outputs(entries)])
    }
}

