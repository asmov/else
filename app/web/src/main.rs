use yew::prelude::*;

#[function_component]
fn App() -> Html {
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

fn main() {
    yew::Renderer::<App>::new().render();
}