use yew::{function_component, html, Html};
use std::{cell::RefCell, rc::Rc};
use crate::api::types::{User, AppState};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
        // <form action={format!("/views/{}", id)} method="get">
        <header>
             <ul class="header__menu">
                <li><a id="1" aria-label="Menu item 1">{"Home"}</a></li>
                <li><a id="2" aria-label="Menu item 2">{"About"}</a></li>
                <li><a id="3" aria-label="Menu item 3">{"Services"}</a></li>
                <li><a id="4" aria-label="Menu item 4">{"Contact"}</a></li>
                <li><a id="5" aria-label="Menu item 5">{"Help"}</a></li>
            </ul>
            <div>
                //// Dynamic User Signin & Register Feature
            </div>
        </header>
        <main>
            <section>
                <h1>{"Welcome to the Index Page"}</h1>
                <p>{"This is the main content of the page."}</p>
            </section>
        </main>
        <footer>
            <h1>{"© 2024 MyWebsite"}</h1>
        </footer>
        // </form>
        </>
    }
}