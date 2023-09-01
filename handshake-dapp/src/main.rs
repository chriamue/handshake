mod app;
pub mod env;
pub use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
