pub mod address_button_component;
mod app;
pub mod connect_to;
pub mod env;
pub mod num_accounts_component;
pub mod num_active_component;
pub mod profile;
pub mod services;
pub mod sign_up;
pub mod leaderboard;
pub use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
