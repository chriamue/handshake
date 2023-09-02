pub mod address_button_component;
mod app;
pub mod env;
pub mod services;
pub mod profile;
pub use app::App;


fn main() {
    yew::Renderer::<App>::new().render();
}
