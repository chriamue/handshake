use crate::services::get_num_accounts;
use anyhow::anyhow;
use yew::prelude::*;

pub struct NumAccountsComponent {
    num_accounts: Option<u128>,
    error: Option<String>,
}

pub enum Message {
    Error(anyhow::Error),
    ReceivedNumAccounts(u128),
}

impl Component for NumAccountsComponent {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match get_num_accounts().await {
                Ok(accounts) => {
                    web_sys::console::log_1(&format!("Num Accounts: {:?}", accounts).into());
                    Message::ReceivedNumAccounts(accounts.parse().unwrap())
                }
                Err(_) => Message::Error(anyhow!("Failed to fetch num accounts.".to_string())),
            }
        });
        NumAccountsComponent {
            num_accounts: None,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ReceivedNumAccounts(count) => {
                self.num_accounts = Some(count);
            }
            Message::Error(err) => self.error = Some(err.to_string()),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { self.num_accounts.unwrap_or_default() }
            </div>
        }
    }
}
