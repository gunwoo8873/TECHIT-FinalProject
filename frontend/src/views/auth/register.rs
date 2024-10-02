use yew::{function_component, html, Html};
use crate::{
    router::Route,
    api::types::{UserRegister}
};

#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <>
        <main>
        <section>
        <div>
            <form>
                <label for="user_id">{"Userid"}</label>
                <input type="text" id="user_id" class="register_input" name="userid"/>

                <label for="user_email">{"Email"}</label>
                <input type="email" id="user_email" class="register_input" name="useremail"/>

                <label for="user_ps">{"Password"}</label>
                <input type="password" id="user_ps" class="register_input" name="password"/>

                <label for="user_ps_check">{"Password Check"}</label>
                <input type="password" id="user_ps_check" class="register_input" name="password_check"/>

                <label for="user_phone">{"Phone Number"}</label>
                <input type="text" id="user_phone" class="register_input" name="phone"/>

                <label for="user_address">{"Address"}</label>
                <input type="text" id="user_address" class="register_input" name="address"/>

                <button type="submit" id="register_btn">{"Register"}</button>
            </form>
        </div>
        </section>
        </main>
        </>
    }
}