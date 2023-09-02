pub mod address_button_component;
mod app;
pub mod env;
pub mod services;
pub mod profile;
pub mod connect_to;
pub use app::App;


fn main() {
    yew::Renderer::<App>::new().render();
}
