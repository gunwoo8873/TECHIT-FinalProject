use yew::{function_component, html, Html};
// use yew_router::prelude::*;
// use crate::router::Route;

#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <>
        <main>
        <section>
        <div>
            <form>
                <label for="user_email">{"Email"}</label>
                <input type="email" id="user_email" name="useremail"/>
                <label for="user_ps">{"Password"}</label>
                <input type="password" id="user_ps" name="password"/>
                <label for="user_address">{"Address"}</label>
                <input type="text" id="user_address" name="address"/>
                <button type="submit">{"Register"}</button>
            </form>
        </div>
        </section>
        </main>
        </>
    }
}