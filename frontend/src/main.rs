mod views;
mod router;
mod app;
mod api;
// mod utils;

pub fn main() {
    yew::Renderer::<app::App>::new().render();
}