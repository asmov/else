use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div id="app" class="container h-screen mx-auto p-1">
            <Terminal />
        </div>
    }
}

#[function_component]
fn Terminal() -> Html {
    let mut log: Vec<String> = Vec::new();
    log.push("lorem ipsum lorem ipsum lorem ipsum".to_string());
    log.push("ipsum lorem ipsum lorem ipsum lorem ipsum".to_string());
    log.push("Hello".to_string());
    html! {
        <div id="terminal" class="border rounded p-1 h-full flex flex-col space-y-2">
        <div id="titlebar" class="border rounded px-2 text-center text-lg">
            <span>{"This is a title"}</span>
        </div>
        <div id="lines" class="h-full border rounded p-1 overflow-y-scroll">
            <p>{log.iter().map(|s| html!{<p>{s}</p>}).collect::<Html>()}</p>
        </div>
        <input type="input" class="hover:shadow-md rounded border px-4" placeholder="input text ..." title="Input" />
        <div id="footer" class="flex justify-between items-center px-6 text-sm *:border *:rounded *:px-1">
            <span>{"Character Name"}</span>
            <span>{"LH: Shield"}</span>
            <span>{"H: 1000 R: 1000 A: 1000"}</span>
            <span>{"RH: Ice Sword"}</span>
            <span>{"23001"}</span>
        </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}