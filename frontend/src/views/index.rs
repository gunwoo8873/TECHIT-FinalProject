use yew::{function_component, html, Html};
use yew_router::prelude::*;
use crate::router::Route;
// use std::{cell::RefCell, rc::Rc};
// use crate::api::types::{User, AppState};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
        // <form action={format!("/views/{}", id)} method="get">
        <header>
             <div class="header__menu">
                <Link<Route> to={Route::Index}>{ "Home" }</Link<Route>>
                <Link<Route> to={Route::AMM}>{ "자산관리" }</Link<Route>>
            </div>
            <div>
                //// Dynamic User Signin & Register Feature
                <Link<Route> to={Route::Signin}>{ "signin" }</Link<Route>>
                <Link<Route> to={Route::Register}>{ "register" }</Link<Route>>
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