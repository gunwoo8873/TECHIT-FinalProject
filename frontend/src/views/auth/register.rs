use yew::{function_component, html, Html};
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Register)]
pub fn register() -> Html {
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
        <div>
            <h1>{"Register Page"}</h1>
            <form>
                <label for="username">{"Username:"}</label>
                <input type="text" id="username" name="username" />
                <label for="password">{"Password:"}</label>
                <input type="password" id="password" name="password" />
                <button type="submit">{"Register"}</button>
            </form>
        </div>
        </section>
        </main>
        </>
    }
}