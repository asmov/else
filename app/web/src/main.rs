use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let mut log: Vec<String> = Vec::new();
    log.push("Hello".to_string());
    html! {
        <div id="app">
            <div id="terminal">
                <div id="log">
                    {log.iter().collect::<Html>()}
                </div>
                <div id="input">
                        <input type="input" placeholder="input text ..." title="Input" />
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}