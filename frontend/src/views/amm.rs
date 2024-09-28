use yew::{function_component, html, Html};
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(AMM)]
pub fn amm() -> Html {
    html! {
        <>
        <header>
             <div class="header__menu">
                <Link<Route> to={Route::Index}>{ "Home" }</Link<Route>>
                <Link<Route> to={Route::AMM}>{ "자산관리" }</Link<Route>>
            </div>
        </header>
        <main>
            <section>
                <div>
                    <span></span>
                </div>
                <div>
                </div>
            </section>
        </main>
        </>
    }
}