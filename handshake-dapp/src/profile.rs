use crate::address_button_component::AddressButtonComponent;
use crate::env::URL;
use base64::encode;
use qrcode_generator::QrCodeEcc;
use yew::prelude::*;

pub enum ProfileMessage {
    AddressChanged(String),
}

pub struct Profile {
    address: Option<String>,
    qr_code_data: Option<String>,
}

impl Component for Profile {
    type Message = ProfileMessage;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Profile {
            address: None,
            qr_code_data: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProfileMessage::AddressChanged(new_address) => {
                self.address = Some(new_address);
                web_sys::console::log_1(
                    &format!(
                        "Address changed to {}",
                        self.address.as_ref().unwrap_or(&"".to_string())
                    )
                    .to_string()
                    .into(),
                );
                let url = format!(
                    "{}connect?id={}",
                    URL,
                    self.address.as_ref().unwrap_or(&"".to_string())
                );
                let png_data: Vec<u8> =
                    qrcode_generator::to_png_to_vec(url, QrCodeEcc::Low, 1024).unwrap();
                self.qr_code_data = Some(encode(&png_data));

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="profile">
                <AddressButtonComponent on_address={ctx.link().callback(ProfileMessage::AddressChanged)} />
                <img id="currentevent" src="res/currentevent.png" width="100%" height="100%" alt="currentevent"/>
                {
                    match &self.qr_code_data {
                        Some(qr_code_data) => html! {
                            <a href={format!("{}connect?id={}", URL, self.address.as_ref().unwrap_or(&"".to_string()))} target="blank">
                                <img width=320 height=320 src={format!("data:image/png;base64,{}", qr_code_data)} />
                            </a>
                        },
                        None => html! {
                            <></>
                        }
                    }
                }
                <img src="res/connections.png" width="100%" height="100%" alt="connections"/>
            </div>
        }
    }
}
