use yew::prelude::*;
use crate::num_accounts_component::NumAccountsComponent;
use crate::num_active_component::NumActiveComponent;

pub struct Leaderboard;

impl Component for Leaderboard {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Leaderboard {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Leaderboard"}</h1>
                <div style="display: flex; justify-content: space-between;">
                    <div style="flex: 1; margin-right: 20px;">
                        {"Event attendees: "}
                        <br />
                        <NumAccountsComponent />
                    </div>
                    <div style="flex: 1;">
                        {"Active networkers: "}
                        <br />
                        <NumActiveComponent />
                    </div>
                </div>
                <img src="res/leaderboard.png" width="100%" height="100%" alt="leaderboard"/>
            </div>
        }
    }
    
}