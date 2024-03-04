use web_sys::{wasm_bindgen::JsCast, HtmlElement, HtmlInputElement};
use yew::{platform::spawn_local, prelude::*, virtual_dom::VChild};
use asmov_else_model as model;
use model::{area, view::world, Descriptive, Identifiable, Routing};
use crate::{target::*, cmd::{self, global::LookCmd, Cli}, error::*, input::ParsedInput, net, ui::terminal::{EntryCategory, EntryProps, Terminal}};

use super::terminal;

pub enum AppMsg {
    Disconnected,
    Start,
    Connected,
    Frame(model::Frame),
    TerminalOutput(String, EntryCategory),
    Synchronized(model::InterfaceView, model::Frame),
    Input(String),
    Ready,
}

#[derive(Default, Properties, PartialEq, Debug)]
pub struct Props {
}

pub struct Stats {
    device: AttrValue,
    left_hand: AttrValue,
    permeability: AttrValue,
    right_hand: AttrValue,
    frame: AttrValue
}

pub struct App {
    terminal_title: AttrValue,
    terminal_output_entries: Vec<VChild<terminal::Entry>>,
    stats: Stats,
    stats_output: Vec<AttrValue>,
    ready: bool,
    frame: model::Frame,
    interface_view: Option<model::InterfaceView>
}

impl Component for App {
    type Message = AppMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let stats = Stats {
            device: AttrValue::Static("/dev/tty/a0f24d3e"),
            left_hand: AttrValue::Static("LH: Empty"),
            permeability: AttrValue::Static("H: 1000 R: 1000 A: 1000"),
            right_hand: AttrValue::Static("RH: Empty"),
            frame: AttrValue::Static("syncing")
        };

        let stats_output = vec![];

        let app = Self{
            terminal_title: AttrValue::Static("Terminal"),
            terminal_output_entries: Vec::new(),
            stats,
            stats_output,
            ready: false,
            frame: 0,
            interface_view: None,
        };

        ctx.link().send_message(AppMsg::Start);
        
        app
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_submit = ctx.link().callback(move |event: SubmitEvent| {
            event.prevent_default();

            let input: HtmlInputElement = event
                .target_dyn_into::<HtmlElement>().unwrap()
                .first_element_child().unwrap()
                .dyn_into::<HtmlInputElement>().unwrap();

            let value = input.value();
            input.set_value("");

            AppMsg::Input(value)
        });

        html! {
            <div id="app" class="container h-screen mx-auto p-1">
                <Terminal {on_submit} title={self.terminal_title.clone()} stats={self.stats_output.clone()} output_entries={self.terminal_output_entries.clone()}/>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Start => {
                let log_callback = ctx.link().callback(|(text, category)| {
                    AppMsg::TerminalOutput(text, category)
                });

                let connection_callback = ctx.link().callback(|status: net::Status| {
                    match status {
                        net::Status::Connected => AppMsg::Connected,
                        net::Status::Disconnected => AppMsg::Disconnected,
                        net::Status::Frame(frame) => AppMsg::Frame(frame),
                        net::Status::Synchronized(interface_view, frame) => AppMsg::Synchronized(interface_view, frame),
                    }
                });

                spawn_local(net::zone_connector_task(connection_callback, log_callback));

                //let mut input = ParsedInput::parse("go".to_string())?;
                //log!(input.parse().unwrap_err().to_string());
            },
            AppMsg::Connected => {
                self.terminal_output("Synchronizing with zone server.", EntryCategory::Technical);
            },
            AppMsg::Synchronized(interface_view, frame) => {
                self.interface_view = Some(interface_view);
                self.frame = frame;

                let interface_view = self.interface_view.as_ref().unwrap();

                self.stats.device = AttrValue::Rc(interface_view.interface().device_name().into());
                self.refresh_stats();

                if !self.ready {
                    self.ready = true;
                    ctx.link().send_message(AppMsg::Ready);
                }
            }
            AppMsg::Ready => {
                self.terminal_output(&format!("Integrated into world at frame {}.", self.frame), EntryCategory::Technical);
                self.terminal_output(&format!("{:?}.", self.interface_view.as_ref().unwrap()), EntryCategory::Debug);
                self.terminal_output("", EntryCategory::Standard);

                //let terminal = model::hardcoded::terminal::create_terminal();
                //let area = terminal.find_area(model::hardcoded::terminal::TERMINAL_AREA_KEY).unwrap();
                let world_view = self.interface_view.as_ref().unwrap().world_view(); 
                let area_view = world_view.area_view();
                self.terminal_title = AttrValue::Rc(area_view.name().into());

                let mut text: Vec<String> = area_view.description().unwrap()
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect();

                text.push("".to_string());

                for route_uid in area_view.route_uids() {
                    let route = world_view.route(*route_uid).unwrap();
                    let end = match route.end_for_area(area_view.uid()) {
                        Some(end) => end,
                        None => continue
                    };

                    text.push(format!("{} : {}", end.direction(), end.name()));
                }

                for entry in text {
                    self.terminal_output(&entry, EntryCategory::Standard);
                }
            },
            AppMsg::Disconnected => {
                self.ready = false;
                self.stats_output = Vec::new();
            },
            AppMsg::Frame(frame) => {
                self.frame = frame;
                self.stats.frame = AttrValue::Rc(format!("{frame}").into());
                self.refresh_stats();
            },
            AppMsg::TerminalOutput(msg, category) => self.terminal_output(&msg, category),
            AppMsg::Input(text) => {
                match ParsedInput::parse(text) {
                    Ok(input) => {
                        match input {
                            ParsedInput::Command(command) => {
                                match command.process(self.interface_view.as_ref().unwrap()) {
                                    Ok(cmd) => {
                                        let text = format!("{:?}", &cmd);
                                        self.terminal_output(&text, EntryCategory::Debug);
                                        match self.cmd(cmd) {
                                            Ok(_) => {},
                                            Err(e) => {
                                                self.terminal_output(&e.to_string(), EntryCategory::Error);
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        self.terminal_output(&e.to_string(), EntryCategory::Error);
                                    }
                                }
                            },
                            ParsedInput::Context(_) => todo!(),
                            ParsedInput::Talk(_) => todo!(),
                        }
                   },
                    Err(e) => {
                        self.terminal_output(&e.to_string(), EntryCategory::Error);
                    },
                }
            }
        }

        true
    }
}

pub trait AppCmd {
    fn run(self, app: &mut App) -> Result<()>;
}

impl AppCmd for LookCmd {
    fn run(self, app: &mut App) -> Result<()> {
        let world_view = app.interface_view.as_ref().unwrap().world_view();
        let area_view = world_view.area_view();
        let area_uid = area_view.uid();

        let mut output: Vec<String> = Vec::new();
        match self.processed.unwrap().subject {
            Target::Area(_area_uid) => {
                output.push(area_view.name().to_string());
                output.push(area_view.description().unwrap().to_string())
            },
            Target::Route(route_uid) => {
                let route = world_view.route(route_uid).unwrap();
                let end = route.end_for_area(area_uid).unwrap();
                output.push(end.name().to_string());
                output.push(end.description().unwrap().to_string());
            },
            Target::Thing(thing_uid) => {
                let thing_view = world_view.thing_view(thing_uid).unwrap();
                output.push(thing_view.name().to_string());
                output.push(thing_view.description().unwrap().to_string());
            },
        };

        for line in output {
            app.terminal_output(&line, EntryCategory::Standard);
        }

        Ok(())
    }
}

impl App {
    fn cmd(&mut self, cmd: cmd::Cmd) -> Result<()> {
        match cmd {
            cmd::Cmd::Look(look_cmd) => look_cmd.run(self),
            _ => Err(Error::Generic(format!("Command not implemented: {}", cmd.name())))
        }
    }

    fn refresh_stats(&mut self) {
        self.stats_output = vec![
            self.stats.device.clone(),
            self.stats.left_hand.clone(),
            self.stats.permeability.clone(),
            self.stats.right_hand.clone(),
            self.stats.frame.clone() ];
    }

    fn terminal_newline(&mut self) {
        self.terminal_output("", EntryCategory::Standard);
    }

    fn terminal_output(&mut self, text: &str, category: EntryCategory) {
        for text in text.split('\n') {
            self.terminal_output_entries.push(VChild::new(
                EntryProps {
                    text: AttrValue::Rc(text.into()),
                    category
                },
                None
            ));
        }
    }
}

