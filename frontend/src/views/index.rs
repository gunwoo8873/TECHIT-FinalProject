use {
    yew::prelude::*,
    yew_router::prelude::*,
    crate::{
        route::Route,
    },
};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
            <header>
                <div id="header__menu">
                    <Link<Route> to={Route::Index}>{ "Home" }</Link<Route>>
                </div>
                <div id="header__account">
                    // <Link<Route> to={Route::Signin}>{ "Sign In" }</Link<Route>>
                    // <Link<Route> to={Route::Register}>{ "Register" }</Link<Route>>
                </div>
            </header>
            <main>
                <section>
                    <p>{ "Welcome to Bank" }</p>
                </section>
            </main>
            <footer>
                <h4>{ "© 2024 MyWebsite" }</h4>
            </footer>
        </>
    }
}