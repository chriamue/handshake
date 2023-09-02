use yew::prelude::*;
use crate::address_button_component::AddressButtonComponent;

pub enum Message {
    AddressChanged(String),
}

pub struct SignUp {
    address: Option<String>,
}

impl Component for SignUp {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SignUp {
            address: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::AddressChanged(new_address) => {
                self.address = Some(new_address);
                web_sys::console::log_1(&format!("Address changed to {}", self.address.as_ref().unwrap_or(&"".to_string())).to_string().into());
                

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <AddressButtonComponent on_address={ctx.link().callback(Message::AddressChanged)} />
            </div>
        }
    }
}
