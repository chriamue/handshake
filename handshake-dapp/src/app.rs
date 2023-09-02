use crate::connect_to::ConnectTo;
use crate::leaderboard::Leaderboard;
use crate::profile::Profile;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/connect")]
    Connect,
    #[at("/profile")]
    Profile,
    #[at("/leaderboard")]
    Leaderboard,
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
                    pairs
                        .iter()
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
                <Link<Route> to={Route::Home}> <button>{"Home"} </button></Link<Route>>
                <ConnectTo id={id} />
            </div> }
        }
        Route::Profile => {
            html! {
                <>
                <Link<Route> to={Route::Home}> <button>{"Home"} </button></Link<Route>>
                    <Profile />
                </>
            }
        }
        Route::Leaderboard => {
            html! {
            <div>
                <Link<Route> to={Route::Home}> <button>{"Home"} </button></Link<Route>>
                <Link<Route> to={Route::Profile}> <button>{"My Profile"}</button></Link<Route>>
                <Leaderboard />
            </div> }
        }
        Route::Home => {
            html! {
            <div>
                <Link<Route> to={Route::Home}> <button>{"Home"} </button></Link<Route>>
                <Link<Route> to={Route::Profile}> <button>{"My Profile"}</button></Link<Route>>
                <Link<Route> to={Route::Leaderboard}> <button>{"Leaderboard"}</button></Link<Route>>
                <img src="res/welcome.png" width="100%" height="100%" alt="welcome"/>
            </div> }
        }
    }
}
