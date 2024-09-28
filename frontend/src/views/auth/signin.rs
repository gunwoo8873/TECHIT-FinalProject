use yew::{function_component, html, Html};
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Signin)]
pub fn signin() -> Html {
    html! {
        <>
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
                <form>
                    <label for="username">{"Username:"}</label>
                    <input type="text" id="username" name="username" />
                    <label for="password">{"Password:"}</label>
                    <input type="password" id="password" name="password" />
                    <button type="submit">{"Sign In"}</button>
                </form>
            </section>
        </main>
        <footer>
            <h1>{"TEST"}</h1>
        </footer>
        </>
    }
}