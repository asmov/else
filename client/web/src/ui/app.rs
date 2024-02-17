use model::Descriptive;
use yew::{platform::spawn_local, prelude::*, virtual_dom::VChild};
use crate::{net, ui::terminal::{EntryCategory, EntryProps, Terminal}};
use elsezone_model as model;

use super::terminal;

pub enum AppMsg {
    Disconnected,
    Start,
    Connected,
    Frame(model::Frame),
    TerminalOutput(String, EntryCategory),
    Synchronized,
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
    terminal_output_entries: Vec<VChild<terminal::Entry>>,
    stats: Stats,
    stats_output: Vec<AttrValue>,
    ready: bool,
    frame: model::Frame
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
            terminal_output_entries: Vec::new(),
            stats,
            stats_output,
            ready: false,
            frame: 0
        };

        ctx.link().send_message(AppMsg::Start);
        
        app
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id="app" class="container h-screen mx-auto p-1">
                <Terminal title="Terminal" stats={self.stats_output.clone()} output_entries={self.terminal_output_entries.clone()}/>
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
                        net::Status::Synchronized => AppMsg::Synchronized
                    }
                });

                spawn_local(net::zone_connector_task(connection_callback, log_callback));

                //let mut input = ParsedInput::parse("go".to_string())?;
                //log!(input.parse().unwrap_err().to_string());
            },
            AppMsg::Connected => {
                self.terminal_output("Synchronizing with zone server.", EntryCategory::Technical);
            },
            AppMsg::Synchronized => {
                if !self.ready {
                    self.ready = true;
                    ctx.link().send_message(AppMsg::Ready);
                }
            }
            AppMsg::Ready => {
                self.terminal_output(&format!("Integrated into world at frame {}.", self.frame), EntryCategory::Technical);

                let terminal = model::hardcoded::terminal::create_terminal();
                let area = terminal.find_area(model::hardcoded::terminal::TERMINAL_AREA_KEY).unwrap();
                let tmp_intro: Vec<&str> = area.description().unwrap()
                    .split('\n')
                    .collect();

                self.terminal_output("", EntryCategory::Standard);

                for text in tmp_intro {
                    self.terminal_output(text, EntryCategory::Standard);
                }

                self.terminal_output("", EntryCategory::Standard);
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
        }

        true
    }
}

impl App {
    fn refresh_stats(&mut self) {
        self.stats_output = vec![
            self.stats.device.clone(),
            self.stats.left_hand.clone(),
            self.stats.permeability.clone(),
            self.stats.right_hand.clone(),
            self.stats.frame.clone() ];
    }

    fn terminal_output(&mut self, text: &str, category: EntryCategory) {
        self.terminal_output_entries.push(VChild::new(
            EntryProps {
                text: AttrValue::Rc(text.into()),
                category
            },
            None
        ));
    }
}

