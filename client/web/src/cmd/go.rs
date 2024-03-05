use model::Identifiable;

use crate::{error::*, target::*, cmd::*, input::*};

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

impl AppCmd for GoCmd {
    fn run(self, app: &App) -> Result<Vec<AppAction>> {
        let interface_view = app.interface_view().unwrap();
        let world_view = interface_view.world_view();
        let route_uid = match self.processed.unwrap().destination {
            Target::Route(route_uid) => route_uid,
            _ => unreachable!("Target should be a route") 
        };
        
        let model_action = model::Action::Go(model::GoAction {
            base: model::BaseAction {
                frame: world_view.frame(),
                thing_uid: interface_view.downlink_uid(),
            },
            origin_area_id: world_view.area_view().uid(),
            route_uid: route_uid,
        });

        Ok(vec![AppAction::ModelAction(model_action)])
    }
}

