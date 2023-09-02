use crate::address_button_component::AddressButtonComponent;
use crate::env::URL;
use base64::encode;
use qrcode_generator::QrCodeEcc;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/connect/:id")]
    Connect { id: String },
}

pub struct App;

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Connect { id } => {
            html! {
            <div>
                <h1>{"Connecting to "}{id}</h1>
            </div> }
        }
        Route::Home => {
            let png_data: Vec<u8> =
                qrcode_generator::to_png_to_vec(&URL, QrCodeEcc::Low, 1024).unwrap();
            let base64_png = encode(&png_data);
            html! {
                <div>
                    <h1>{"Welcome to the Handshake!"}</h1>
                    <img width=320 height=320 src={format!("data:image/png;base64,{}", base64_png)} />
                    <AddressButtonComponent on_address={Callback::noop()} />
                </div>
            }
        }
    }
}
