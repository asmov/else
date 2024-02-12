use yew::{prelude::*, virtual_dom::VChild};
use gloo_console::log;
use crate::{net, ui::terminal::{EntryCategory, EntryProps, Terminal}};

use super::terminal;

pub enum AppMsg {
    Ready,
    Connected,
    Disconnected,
    TerminalOutput(String, EntryCategory)
}

#[derive(Default, Properties, PartialEq, Debug)]
pub struct Props {
}

pub struct App {
    terminal_output_entries: Vec<VChild<terminal::Entry>>,
    log: Vec<AttrValue>,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
       let mut app = Self{
            terminal_output_entries: Vec::new(),
            log: vec![
                // World::arrival_description()
                AttrValue::Static("Welcome to Terminal."),
                AttrValue::Static("Connect your interface to begin your journey."),
                AttrValue::Static(""),
                // Region::arrival_description() := None
                // Area::arrival_description()
                AttrValue::Static("A myriad of bright colors race around you and then dissolve into your surroundings as \
                quickly as they appeared."),
                AttrValue::Static(""),
                // Area::description()
                AttrValue::Static("You find yourself in what appears to be an enormous translucent sphere. Beyond that,
                you see only the void of space, littered with clusters of brightly lit stars in all directions. The iridescent wall \
                of the great sphere shimmers with color in tune with the motion and sounds around you. Numerous others, \
                like yourself, hustle and bustle about the area. You hear the soft buzz of the commotion surrounding you; \
                discussions, laughter, the whirring of people casually materializing in and out of existence."),
                AttrValue::Static(""),
                AttrValue::Static("A holographic computer screen materializes in front of you. The dotted blue outline of a \
                hand appears in the center of the screen with instructions below:"),
                AttrValue::Static("Type .connect to begin ..."),
            ]
        };

        ctx.link().send_message(AppMsg::Ready);
        
        app.terminal_output_entries.push(VChild::new(
            EntryProps{
                text: AttrValue::Static("test"),
                category: EntryCategory::Standard
            },
            None
        ));
          
        app
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let stats: Vec<AttrValue> = vec![
            AttrValue::Static("/dev/tty/a0f24d3e"),
            AttrValue::Static("LH: Empty"),
            AttrValue::Static("H: 1000 R: 1000 A: 1000"),
            AttrValue::Static("RH: Empty"),
            AttrValue::Static("23001"),
        ];

       html! {
            <div id="app" class="container h-screen mx-auto p-1">
                <Terminal title="Terminal" stats={stats} output_entries={self.terminal_output_entries.clone()}/>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Ready => {
                let callback = ctx.link().callback(|(text, category)| {
                    AppMsg::TerminalOutput(text, category)
                });

                ctx.link().send_future(async {
                    match net::connect(callback).await {
                        Ok(_) => AppMsg::Connected,
                        Err(_) => AppMsg::Disconnected
                    }
                });

                self.terminal_output_entries.push(VChild::new(
                    EntryProps {
                        text: AttrValue::Static("Ready to party"),
                        category: EntryCategory::Debug
                    },
                    None
                ));
            
                log!("READY");

                //let mut input = ParsedInput::parse("go".to_string())?;
                //log!(input.parse().unwrap_err().to_string());
 
                self.log.push(AttrValue::Static("Ready."));
            },
            AppMsg::Connected => {
                log!("CONNECTED");
                self.log.push(AttrValue::Static("Connected."));
            },
            AppMsg::Disconnected => log!("DISCONNECTED"),
            AppMsg::TerminalOutput(msg, category) => self.terminal_output(msg, category),
        }

        true
    }
}

impl App {
    fn terminal_output(&mut self, text: String, category: EntryCategory) {
        self.terminal_output_entries.push(VChild::new(
            EntryProps {
                text: AttrValue::Rc(text.into()),
                category
            },
            None
        ));
    }
}

