use crate::services::get_azero_id;
use yew::prelude::*;

pub enum Message {
    AzeroIdReceived(Result<String, anyhow::Error>),
}

pub struct AzeroId {
    domain: Option<String>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub account: Option<String>,
}

impl Component for AzeroId {
    type Message = Message;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        if let Some(account) = &ctx.props().account {
            let account = account.clone();
            ctx.link().send_future(async move {
                let domain_result = get_azero_id(account.clone()).await;
                Message::AzeroIdReceived(domain_result)
            });
        }
        AzeroId { domain: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::AzeroIdReceived(Ok(domain_str)) => {
                self.domain = Some(domain_str);
                true
            }
            Message::AzeroIdReceived(Err(_)) => {
                // Handle error here if needed.
                // For this example, we'll just leave the domain as None.
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                { if let Some(domain) = &self.domain {
                    html! {
                        <>
                            <div> {"Azero ID: "}
                                <a href={format!("https://{}.id", domain)} target="_blank">
                                    {domain}
                                </a>
                            </div>
                        </> }
                  } else {
                    html! { <div>{"Fetching Azero ID..."}</div> }
                  }
                }
            </>
        }
    }
}
