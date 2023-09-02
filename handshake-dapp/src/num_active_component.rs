use crate::services::get_num_handshakes;
use anyhow::anyhow;
use yew::prelude::*;

pub struct NumActiveComponent {
    num_handshakes: Option<u128>,
    error: Option<String>,
}

pub enum Message {
    Error(anyhow::Error),
    ReceivedNumAccounts(u128),
}

impl Component for NumActiveComponent {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match get_num_handshakes().await {
                Ok(handshakes) => {
                    web_sys::console::log_1(&format!("Num Handshakes: {:?}", handshakes).into());
                    Message::ReceivedNumAccounts(handshakes.parse().unwrap())
                }
                Err(_) => Message::Error(anyhow!("Failed to fetch num handshakes.".to_string())),
            }
        });
        NumActiveComponent {
            num_handshakes: None,
            error: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ReceivedNumAccounts(count) => {
                self.num_handshakes = Some(count);
            }
            Message::Error(err) => self.error = Some(err.to_string()),
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { self.num_handshakes.unwrap_or_default() }
            </div>
        }
    }
}
