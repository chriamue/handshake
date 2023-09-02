use crate::address_button_component::AddressButtonComponent;
use crate::azero_id::AzeroId;
use crate::services::do_handshake;
use crate::services::Account;
use yew::prelude::*;

pub enum Message {
    AccountChanged(Account),
    DoHandshake,
}

pub struct ConnectTo {
    account: Option<Account>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: String,
}

impl Component for ConnectTo {
    type Message = Message;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        ConnectTo { account: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::AccountChanged(account) => {
                self.account = Some(account);
                true
            }
            Message::DoHandshake => {
                web_sys::console::log_1(&"Doing handshake".into());
                web_sys::console::log_1(&format!("Account: {:?}", self.account).into());
                if let Some(account) = &self.account {
                    let source = account.source.clone();
                    let sender_address = account.address.clone();
                    let destination_address = ctx.props().id.clone();
                    let account_clone = account.clone();
                    ctx.link().send_future(async move {
                        match do_handshake(source, sender_address, destination_address).await {
                            Ok(response) => {
                                web_sys::console::log_1(
                                    &format!("Handshake success: {}", response).into(),
                                );
                            }
                            Err(e) => {
                                web_sys::console::log_1(
                                    &format!("Handshake error: {:?}", e).into(),
                                );
                            }
                        }
                        Message::AccountChanged(account_clone)
                    });
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let id = ctx.props().id.clone();
        html! {
            <div>
            <AddressButtonComponent on_account={Some(ctx.link().callback(Message::AccountChanged))} on_address={Callback::noop()}/>

            <div id="connect">
                <img id="currentevent" src="res/currentevent.png" width="100%" height="100%" alt="currentevent"/>
                <div id="connectto">{"Connecting to "}</div>
                <div id="aicon">
                    <img src="res/aicon.png" width="124px" height="124px" alt="aicon"/>
                </div>
                <div id="azeroid">
                <AzeroId account={id.clone()} />
                </div>
                <div id="id">
                {id.clone()}</div>
                <button onclick={ctx.link().callback(|_| Message::DoHandshake)}>{"Handshake"}</button>
            </div>
            </div>
        }
    }
}
