use yew::*;
use yew_router::prelude::*;
use crate::{
    // views::style::{
    // },
    route::Route,
    handlers::{
        index_handler
    },
};

#[function_component(Cost)]
pub fn cost() -> Html {
    html! {
        <>
            <header>
                <div id="header__menu">
                    <Link<Route> to={Route::Index}>{ "Home" }</Link<Route>>
                </div>
            </header>
            <main>
                <section>
                    <div>{ "Cost Management" }</div>
                </section>
                <section>
                    <div>
                        <span>{"Test ACC Name"}</span>
                        <span>{"Test ACC Number"}</span>
                    </div>
                </section>
            </main>
            <footer>
                <h4>{ "© 2024 MyWebsite" }</h4>
            </footer>
        </>
    }
}