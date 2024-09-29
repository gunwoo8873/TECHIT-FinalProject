use yew::*;
use yew_router::prelude::*;
use crate::router::Route;
use crate::views::css::{
    header_style::{*}, footer_style::{*}
};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
        <header style={header()}>
             <div class="header__menu">
                <Link<Route> to={Route::Index}>{ "Home" }</Link<Route>>
                <Link<Route> to={Route::AMM}>{ "자산관리" }</Link<Route>>
            </div>
            <div class="header__menu">
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
        <footer style={footer()}>
            <h4>{"© 2024 MyWebsite"}</h4>
        </footer>
        </>
    }
}