use yew::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::views::css::{
    header_style::*, footer_style::*,
};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
            <header style={header()}>
                <div id="header__menu">
                    <Link<Route> to={Route::Index}>{ "Home" }</Link<Route>>
                    <Link<Route> to={Route::AMM}>{ "AMM" }</Link<Route>>
                    <Link<Route> to={Route::Directions}>{ "Directions" }</Link<Route>>
                </div>
                <div id="header__account">
                    <Link<Route> to={Route::Signin}>{ "Sign In" }</Link<Route>>
                    <Link<Route> to={Route::Register}>{ "Register" }</Link<Route>>
                </div>
            </header>
            <main>
                <section>
                    <h1>{ "Welcome to the Index Page" }</h1>
                    <p>{ "This is the main content of the page." }</p>
                </section>
            </main>
            <footer>
                <h4>{ "© 2024 MyWebsite" }</h4>
            </footer>
        </>
    }
}