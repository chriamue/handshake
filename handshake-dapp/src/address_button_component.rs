use crate::services::get_accounts;
use crate::services::Account;
use futures::FutureExt;
use yew::prelude::*;

pub struct AddressButtonComponent {
    address: Option<String>,
    stage: AddressStage,
}

pub enum AddressMessage {
    Error(anyhow::Error),
    RequestAccounts,
    GotAddress(Vec<Account>),
    SignWithAccount(usize),
}

#[derive(Properties, Clone, PartialEq)]
pub struct AddressButtonProps {
    pub on_address: Callback<String>,
}

pub enum AddressStage {
    Error(String),
    EnterAccount,
    RequestingAccounts,
    SelectAccount(Vec<Account>),
}

impl Component for AddressButtonComponent {
    type Message = AddressMessage;

    type Properties = AddressButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        AddressButtonComponent {
            address: None,
            stage: AddressStage::EnterAccount,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AddressMessage::Error(err) => {
                self.stage = AddressStage::Error(err.to_string());
                true
            }
            AddressMessage::RequestAccounts => {
                self.stage = AddressStage::RequestingAccounts;
                // Logic to fetch address from Polkadot wallet.
                web_sys::console::log_1(&"Requesting accounts".into());
                ctx.link().send_future(get_accounts().map(
                    |accounts_or_err| match accounts_or_err {
                        Ok(accounts) => AddressMessage::GotAddress(accounts),
                        Err(err) => AddressMessage::Error(err),
                    },
                ));
                true
            }
            AddressMessage::GotAddress(accounts) => {
                web_sys::console::log_1(&format!("Got accounts: {:?}", accounts).into());
                if accounts.is_empty() {
                    self.stage = AddressStage::EnterAccount;
                } else {
                    let account: &Account = &accounts[0].clone();
                    self.stage = AddressStage::SelectAccount(accounts);

                    ctx.props().on_address.emit(account.address.clone());
                }
                true
            }
            AddressMessage::SignWithAccount(index) => {
                // You can add the logic here to handle the selected account.
                // For simplicity, I'll just set the stage to EnterAccount again.
                self.stage = AddressStage::EnterAccount;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.stage {
            AddressStage::Error(error_message) => {
                html!(<div class="error"> {"Error: "} {error_message} </div>)
            }
            AddressStage::EnterAccount => {
                let get_accounts_click = ctx.link().callback(|_| AddressMessage::RequestAccounts);
                html!(<>
                    <div>
                        <button onclick={get_accounts_click}> {"=> Select an Account"} </button>
                    </div>
                </>)
            }
            AddressStage::RequestingAccounts => {
                html!(<div>{"Querying extensions for accounts..."}</div>)
            }
            AddressStage::SelectAccount(accounts) => {
                if accounts.is_empty() {
                    html!(<div>{"No Web3 extension accounts found. Install Talisman or the Polkadot.js extension and add an account."}</div>)
                } else {
                    html!(
                        <>
                            <div class="mb"><b>{"Select an account you want to use for signing:"}</b></div>
                            { for accounts.iter().enumerate().map(|(i, account)| {
                                let sign_with_account = ctx.link().callback(move |_| AddressMessage::SignWithAccount(i));
                                html! {
                                    <button onclick={sign_with_account}>
                                        {&account.source} {" | "} {&account.name}<br/>
                                        <small>{&account.address}</small>
                                    </button>
                                }
                            }) }
                        </>
                    )
                }
            }
        }
    }
}
