pub mod address_button_component;
mod app;
pub mod azero_id;
pub mod connect_to;
pub mod env;
pub mod leaderboard;
pub mod num_accounts_component;
pub mod num_active_component;
pub mod profile;
pub mod services;
pub mod sign_up;
pub use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
