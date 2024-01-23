use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: AttrValue,
    pub stats: Vec<AttrValue>,
    pub log: Vec<AttrValue>
}

pub struct Msg {

}

pub struct Terminal {

}

impl Component for Terminal {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="terminal border rounded p-1 h-full flex flex-col space-y-2">
                <div class="terminal-title border rounded px-2 text-center text-lg">
                    <span>{ctx.props().title.clone()}</span>
                </div>
                <div class="terminal-output h-full border rounded p-1 overflow-y-scroll">
                    {ctx.props().log.iter().map(|s| if s.is_empty() { html!{ <p><br /></p> } } else { html!{<p>{s}</p>} }).collect::<Html>()}
                </div>
                <input class="terminal-input" type="input" class="hover:shadow-md rounded border px-4" placeholder="input text ..." title="Input" />
                <div class="terminal-stats flex justify-between items-center px-6 text-sm">
                    {ctx.props().stats.iter().map(|s| html!{<span class="border rounded px-1">{s}</span>}).collect::<Html>()}
                </div>
            </div>
        }
    }
}

