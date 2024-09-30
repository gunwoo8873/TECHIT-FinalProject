// mod api;
mod pages;
mod router;
mod app;

pub fn main() {
    yew::Renderer::<app::App>::new().render();
}