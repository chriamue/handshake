mod app;
pub use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
