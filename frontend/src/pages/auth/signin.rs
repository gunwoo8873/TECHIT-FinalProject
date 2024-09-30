use yew::{function_component, html, Html};
use yew_router::prelude::*;
use crate::router::Route;

#[function_component(Signin)]
pub fn signin() -> Html {
    html! {
        <>
        <main>
            <section>
                <form>
                    <label for="username">{"Username"}</label>
                    <input type="text" id="username" name="username" />
                    <label for="password">{"Password"}</label>
                    <input type="password" id="password" name="password" />
                    <div>
                        <button type="submit">{"Sign In"}</button>
                        <Link<Route> to={Route::Index}>
                            <button type="submit">{"Cancel"}</button>
                        </Link<Route>>
                    </div>
                    <div>
                        <Link<Route> to={Route::Directions}>{"Forgot Password?"}</Link<Route>>
                        <input type="checkbox" name="rememberid" id="rememberid" />
                        <label for="rememberid">{"Remember me"}</label>
                    </div>
                </form>
            </section>
        </main>
        <footer>
            <h1>{"TEST"}</h1>
        </footer>
        </>
    }
}