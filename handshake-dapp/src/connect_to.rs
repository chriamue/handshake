use yew::prelude::*;

pub struct ConnectTo {
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: String,
}

impl Component for ConnectTo {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        ConnectTo {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = ctx.props().id.clone();
        html! {
            <div>
                <h1>{"Connecting to "}{id}</h1>
            </div>
        }
    }
}