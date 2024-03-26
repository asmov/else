use model::Web3AuthAnswer;
use web_sys::{wasm_bindgen::JsCast, HtmlElement, HtmlInputElement};
use yew::{platform::{pinned::mpsc::UnboundedSender, spawn_local}, prelude::*, virtual_dom::VChild};
use asmov_else_model::{self as model, Descriptive, Identifiable, Routing};
use crate::{cmd::{self, AppCmd}, error::*, input::{self, *}, net, ui::terminal::{EntryCategory, EntryProps, Terminal}};
use super::terminal;

pub enum AppMsg {
    Disconnected,
    Start,
    Connect(model::ClientToZoneMessage),
    AuthChallenge(model::AuthChallengeMsg),
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
    interface_view: Option<model::InterfaceView>,
    ui_to_net_tx: Option<UnboundedSender<net::UItoNetMsg>>,
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
            ui_to_net_tx: None
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
            },
            AppMsg::Connect(auth_request) => {
                // message should be an auth request or registration
                #[cfg(debug_assertions)]
                matches!(auth_request, model::ClientToZoneMessage::AuthRequest(_) | model::ClientToZoneMessage::AuthRegister(_));

                let log_callback = ctx.link().callback(|(text, category)| {
                    AppMsg::TerminalOutput(text, category)
                });

                let net_to_ui_callback = ctx.link().callback(|status: net::NetToUIMsg| {
                    match status {
                        net::NetToUIMsg::AuthChallenge(auth_challenge) => AppMsg::AuthChallenge(auth_challenge),
                        net::NetToUIMsg::Connected(interface_uid) => AppMsg::Connected,
                        net::NetToUIMsg::Disconnected => AppMsg::Disconnected,
                        net::NetToUIMsg::Frame(frame) => AppMsg::Frame(frame),
                        net::NetToUIMsg::Synchronized(interface_view, frame) => AppMsg::Synchronized(interface_view, frame),
                    }
                });

                let (ui_to_net_tx, mut ui_to_net_rx) = yew::platform::pinned::mpsc::unbounded::<net::UItoNetMsg>();
                self.ui_to_net_tx = Some(ui_to_net_tx);
                spawn_local(net::zone_connector_task(net_to_ui_callback, ui_to_net_rx, auth_request, log_callback));
            },
            AppMsg::AuthChallenge(auth_challenge) => {
                //todo: do something 
                let auth_answer = match auth_challenge {
                    model::AuthChallengeMsg::Web3(challenge) => {
                        model::AuthAnswerMsg::Web3(Web3AuthAnswer{
                            signature: [0; 64] //todo
                        })
                    }
                };

                self.ui_to_net_tx
                    .as_ref().expect("UI to Net channel should exist")
                    .send_now(net::UItoNetMsg::AuthAnswer(auth_answer)).unwrap();
            },
            AppMsg::Connected => {
                self.to_terminal_output("Synchronizing with zone server.", EntryCategory::Technical);
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
                self.to_terminal_output(&format!("Integrated into world at frame {}.", self.frame), EntryCategory::Technical);
                self.to_terminal_output(&format!("{:?}.", self.interface_view.as_ref().unwrap()), EntryCategory::Debug);
                self.to_terminal_output("", EntryCategory::Standard);

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
                    self.to_terminal_output(&entry, EntryCategory::Standard);
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
            AppMsg::TerminalOutput(msg, category) => self.to_terminal_output(&msg, category),
            AppMsg::Input(text) => self.handle_input(text)
        }

        true
    }
}

pub enum AppAction {
    TerminalOutput(AttrValue, EntryCategory),
    TerminalOutputs(Vec<(AttrValue, EntryCategory)>),
    ModelAction(model::Action)
}

impl AppAction {
    pub fn new_terminal_output(text: &str, category: EntryCategory) -> Self {
        Self::TerminalOutput(AttrValue::Rc(text.into()), category)
    }

    pub fn new_terminal_outputs(entries: Vec<(&str, EntryCategory)>) -> Self {
        let entries = entries.into_iter()
            .map(|(text, category)| (AttrValue::Rc(text.into()), category))
            .collect();

        Self::TerminalOutputs(entries)
    }
}


impl App {
    fn run_cmd(&mut self, cmd: cmd::Cmd) -> Result<()> {
        let app_action = match cmd {
            cmd::Cmd::Look(cmd) => cmd.run(self)?,
            cmd::Cmd::Go(cmd) => cmd.run(self)?,
            _ => Err(Error::Generic(format!("Command not implemented: {}", cmd.name())))?
        };

        self.perform(app_action)
    }

    fn perform(&mut self, actions: Vec<AppAction>) -> Result<()> {
        for action in actions {
            match action {
                AppAction::TerminalOutput(text, category) => self.terminal_output(text, category),
                AppAction::TerminalOutputs(entries) => self.terminal_outputs(entries),
                AppAction::ModelAction(model_action) => {
                    self.to_terminal_output(&format!("{:?}", model_action), EntryCategory::Debug);
                }
            }
        }

        Ok(())
    }

    fn handle_input(&mut self, text: String) {
        let input = match input::ParsedInput::parse(text) {
            Ok(input) => input,
            Err(e) => {
                self.to_terminal_output(&e.to_string(), EntryCategory::Error);
                return;
            }
        };

        match input {
            ParsedInput::Command(command) => self.handle_command(command),
            ParsedInput::Context(_) => todo!("handle_context"),
            ParsedInput::Talk(_) => todo!("handle_talk"),
        }
    }

    fn handle_command(&mut self, command: input::Command) {
        let cmd = match command.process(self.interface_view.as_ref().unwrap()) {
            Ok(cmd) => cmd,
            Err(e) => {
                self.to_terminal_output(&e.to_string(), EntryCategory::Error);
                return;
            }
        };

        let text = format!("{:?}", &cmd);
        self.to_terminal_output(&text, EntryCategory::Debug);

        match self.run_cmd(cmd) {
            Ok(_) => {},
            Err(e) => {
                self.to_terminal_output(&e.to_string(), EntryCategory::Error);
            }
        }
    }

    pub fn interface_view(&self) -> Option<&model::InterfaceView> {
        self.interface_view.as_ref()
    }

    pub fn refresh_stats(&mut self) {
        self.stats_output = vec![
            self.stats.device.clone(),
            self.stats.left_hand.clone(),
            self.stats.permeability.clone(),
            self.stats.right_hand.clone(),
            self.stats.frame.clone() ];
    }

    pub fn terminal_newline(&mut self) {
        self.to_terminal_output("", EntryCategory::Standard);
    }

    pub fn to_terminal_output(&mut self, text: &str, category: EntryCategory) {
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

    pub fn terminal_output(&mut self, text: AttrValue, category: EntryCategory) {
        self.terminal_output_entries.push(VChild::new(EntryProps { text, category }, None));
    }
   
    pub fn terminal_outputs(&mut self, entries: Vec<(AttrValue, EntryCategory)>) {
        for (text, category) in entries {
            self.terminal_output_entries.push(VChild::new(
                EntryProps { text, category },
                None
            ));
        }
    }

    pub fn to_terminal_outputs(&mut self, entries: Vec<&str>, category: EntryCategory) {
        for entry in entries {
            for text in entry.split('\n') {
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

}

