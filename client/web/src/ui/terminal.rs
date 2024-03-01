use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TerminalProps {
    pub title: AttrValue,
    pub stats: Vec<AttrValue>,
    pub output_entries: ChildrenWithProps<Entry>,
    pub on_submit: Callback<SubmitEvent>,
}

pub enum TerminalMsg {
    NewEntry(String, EntryCategory)
}

pub struct Terminal {

}

impl Component for Terminal {
    type Message = TerminalMsg;
    type Properties = TerminalProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.props().on_submit.clone();

        html! {
            <div class="terminal border rounded p-1 h-full flex flex-col space-y-2">
                <div class="terminal-title border rounded px-2 text-center text-lg">
                    <span>{ctx.props().title.clone()}</span>
                </div>
                <Output>
                    {for ctx.props().output_entries.iter() }
                </Output>
                <form {onsubmit} class="w-full">
                <input class="w-full hover:shadow-md rounded border px-4" placeholder="input text ..." title="Input" />
                </form>
                <div class="terminal-stats flex justify-between items-center px-6 text-sm">
                {
                    ctx.props().stats.iter()
                        .map(|s| html!{ <span class="border rounded px-1">{s.clone()}</span> })
                        .collect::<Html>()
                }
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TerminalMsg::NewEntry(text, category) => {
                let _entry_props = EntryProps{
                    text: AttrValue::Rc(text.into()),
                    category,
                };

                true
            }
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct OutputProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Entry>
}

pub struct OutputMsg {}

pub struct Output {
}

impl Component for Output {
    type Message = OutputMsg;
    type Properties = OutputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div class="h-full border rounded p-1 overflow-y-scroll">
                { for ctx.props().children.iter() } 
            </div>
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TerminalContext {
    Global,
    Inventory
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EntryCategory {
    Standard,
    Technical,
    Warning,
    Error,
    Debug,
    Context(TerminalContext)
}

#[derive(PartialEq, Properties, Clone)]
pub struct EntryProps {
    pub text: AttrValue,
    pub category: EntryCategory
}

#[derive(PartialEq)]
pub struct Entry { }

pub enum EntryMsg {}

impl Component for Entry {
    type Message = EntryMsg;
    type Properties = EntryProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match ctx.props().category {
            EntryCategory::Standard => {
                    if ctx.props().text.is_empty() {
                        html!{<p><br/></p>}
                    } else {
                        html!{<p>{ctx.props().text.clone()}</p>}
                    }
            },
            EntryCategory::Technical => html!{
                <p class="text-lime-800">{ctx.props().text.clone()}</p>
            },
            EntryCategory::Debug => html!{
                <p class="text-lime-600">{ctx.props().text.clone()}</p>
            },
            EntryCategory::Warning => html!{
                <p class="text-orange-800">{ctx.props().text.clone()}</p>
            },
            EntryCategory::Error => html!{
                <p class="text-red-800">{ctx.props().text.clone()}</p>
            },
             EntryCategory::Context(_) => html!{
                <p class="text-slate-600">{ctx.props().text.clone()}</p>
            }
        }

    }
}