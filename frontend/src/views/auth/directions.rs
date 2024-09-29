use yew::{function_component, html, Html};
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Directions)]
pub fn directions() -> Html {
    html! {
        <>
        <main>
        <section>
        <div>
            <p>{"This is the Directions Page"}</p>
        </div>
        </section>
        </main>
        </>
    }
}