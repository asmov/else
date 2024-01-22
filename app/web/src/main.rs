use reqwasm::websocket::Message;
use reqwasm::websocket::WebSocketError;
use yew::prelude::*;
use yew::props;
use wasm_bindgen_futures;
use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
use reqwasm::websocket::futures::WebSocket;
use gloo_console::log;
use elsezone_model as model;
use bytes;

#[derive(PartialEq, Properties)]
struct TerminalProps {
    title: AttrValue,
    stats: Vec<AttrValue>,
    log: Vec<AttrValue>
}

#[function_component]
fn Terminal(props: &TerminalProps) -> Html {
    html! {
        <div id="terminal" class="border rounded p-1 h-full flex flex-col space-y-2">
            <div id="titlebar" class="border rounded px-2 text-center text-lg">
                <span>{props.title.clone()}</span>
            </div>
            <div id="lines" class="h-full border rounded p-1 overflow-y-scroll">
                {props.log.iter().map(|s| if s.is_empty() { html!{ <p><br /></p> } } else { html!{<p>{s}</p>} }).collect::<Html>()}
            </div>
            <input type="input" class="hover:shadow-md rounded border px-4" placeholder="input text ..." title="Input" />
            <div id="footer" class={classes!("flex", "justify-between", "items-center", "px-6", "text-sm", "*:border", "*:rounded", "*:px-1")}>
                {props.stats.iter().map(|s| html!{<span class="border rounded px-1">{s}</span>}).collect::<Html>()}
            </div>
        </div>
    }
}

const LOCALHOST_WS_URL: &'static str = "ws://127.0.0.1:6432";

enum Msg {
    Ready,
    Connected,
    Disconnected,
    Received(String)
}

#[derive(Default, Properties, PartialEq, Debug)]
struct Properties {}

struct App {

}

impl App {
    async fn connect() -> Result<(), WebSocketError> {
        let websocket = WebSocket::open(LOCALHOST_WS_URL).unwrap();
        let (mut tx, mut rcv) = websocket.split();
        let msg = "Hello, server. This is client.".to_string();
        tx.send(Message::Text(msg)).await?;
        if let Some(msg) = rcv.next().await {
            match msg {
                Ok(Message::Bytes(b)) => {
                    let world = model::testing::world_from_binary(b).unwrap();
                    log!(format!("RCV: {:?}", world));
                },
                _ => panic!("AHHHHH"),
            }
        }

        Ok(())
    }
}


impl Component for App {
    type Message = Msg;
    type Properties = Properties;

    fn create(ctx: &Context<Self>) -> Self {
        let app = Self{};
        ctx.link().send_message(Msg::Ready);
        
        app
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let stats: Vec<AttrValue> = vec![
            AttrValue::Static("/dev/tty/a0f24d3e"),
            AttrValue::Static("LH: Shield"),
            AttrValue::Static("H: 1000 R: 1000 A: 1000"),
            AttrValue::Static("RH: Ice Sword"),
            AttrValue::Static("23001"),
        ];

        let log: Vec<AttrValue> = vec![
            AttrValue::Static("Welcome to Terminal."),
            AttrValue::Static("Connect your interface to begin your journey."),
            AttrValue::Static(""),
            AttrValue::Static("A myriad of bright colors race around you and then dissolve into your surroundings as \
            quickly as they appeared."),
            AttrValue::Static(""),
            AttrValue::Static("You find yourself in what appears to be an enormous translucent sphere. Beyond that,
            you can see only the void of space, littered with clusters of brightly lit stars in all directions. The walls \
            of the great sphere shimmer with color in tune with the motion and sounds around you."),
        ];

        html! {
            <div id="app" class="container h-screen mx-auto p-1">
                <Terminal title="Terminal" log={log} stats={stats} />
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Ready => {
                ctx.link().send_future(async {
                    match Self::connect().await {
                        Ok(_) => Msg::Connected,
                        Err(_) => Msg::Disconnected
                    }
                });

                log!("READY")
            },
            Msg::Connected => log!("CONNECTED"),
            Msg::Disconnected => log!("DISCONNECTED"),
            Msg::Received(_) => log!("RECEIVED"),
        }
        
        false
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}