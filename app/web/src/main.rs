use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let stats: Vec<AttrValue> = vec![
        AttrValue::Static("Character Name"),
        AttrValue::Static("LH: Shield"),
        AttrValue::Static("H: 1000 R: 1000 A: 1000"),
        AttrValue::Static("RH: Ice Sword"),
        AttrValue::Static("23001"),
    ];

    let log: Vec<AttrValue> = vec![
        AttrValue::Static("lorem ipsum lorem ipsum lorem ipsum"),
        AttrValue::Static("ipsum lorem ipsum lorem ipsum lorem ipsum"),
        AttrValue::Static("lorem ipsum lorem ipsum lorem ipsum lorem ipsum")
    ];

    html! {
        <div id="app" class="container h-screen mx-auto p-1">
            <Terminal title="This is a title" log={log} stats={stats} />
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
                {props.log.iter().map(|s| html!{<p>{s}</p>}).collect::<Html>()}
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