use crate::env::URL;
use base64::encode;
use qrcode_generator::QrCodeEcc;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::window;
use crate::profile::Profile;
use crate::connect_to::ConnectTo;
use crate::sign_up::SignUp;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/connect")]
    Connect,
    #[at("/profile")]
    Profile,
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
        Route::Connect => {
            let id = match window().and_then(|win| win.location().search().ok()) {
                Some(query) => {
                    let query = query.trim_start_matches('?');
                    let pairs: Vec<&str> = query.split('&').collect();
                    pairs.iter()
                        .filter_map(|&pair| {
                            let mut split = pair.splitn(2, '=');
                            let key = split.next()?;
                            let value = split.next()?;
                            if key == "id" {
                                Some(value.to_string())
                            } else {
                                None
                            }
                        })
                        .next()
                        .unwrap_or_default()
                }
                None => String::new(),
            };
            html! {
            <div>
                <ConnectTo id={id} />
            </div> }
        }
        Route::Profile => {
            let png_data: Vec<u8> =
                qrcode_generator::to_png_to_vec(&URL, QrCodeEcc::Low, 1024).unwrap();
            let base64_png = encode(&png_data);
            html! {
                <>
                <Link<Route> to={Route::Home}> <button>{"Home"} </button></Link<Route>>
                    <Profile />
                </>
            }
        }
        Route::Home => {
            html! {
            <div>
                <h1>{"Welcome Handshake!"}</h1>
                <Link<Route> to={Route::Home}> <button>{"Home"} </button></Link<Route>>
                <Link<Route> to={Route::Profile}> <button>{"My Profile"}</button></Link<Route>>

                <SignUp />
            </div> }
        }
    }
}
