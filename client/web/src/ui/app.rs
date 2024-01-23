use yew::prelude::*;
use gloo_console::log;
use crate::{net, ui::terminal::Terminal};

pub enum Msg {
    Ready,
    Connected,
    Disconnected,
    Received(String)
}

#[derive(Default, Properties, PartialEq, Debug)]
pub struct Props {}

pub struct App {
    log: Vec<AttrValue>
}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let app = Self{
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
                AttrValue::Static(""),
            ]
        };

        ctx.link().send_message(Msg::Ready);
        
         
        app
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let stats: Vec<AttrValue> = vec![
            AttrValue::Static("/dev/tty/a0f24d3e"),
            AttrValue::Static("LH: Empty"),
            AttrValue::Static("H: 1000 R: 1000 A: 1000"),
            AttrValue::Static("RH: Empty"),
            AttrValue::Static("23001"),
        ];

        

        html! {
            <div id="app" class="container h-screen mx-auto p-1">
                <Terminal title="Terminal" log={self.log.clone()} stats={stats} />
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Ready => {
                ctx.link().send_future(async {
                    match net::connect().await {
                        Ok(_) => Msg::Connected,
                        Err(_) => Msg::Disconnected
                    }
                });

                log!("READY");
                self.log.push(AttrValue::Static("Ready."));
            },
            Msg::Connected => {
                log!("CONNECTED");
                self.log.push(AttrValue::Static("Connected."));
            },
            Msg::Disconnected => log!("DISCONNECTED"),
            Msg::Received(_) => log!("RECEIVED"),
        }

        true
    }
}

