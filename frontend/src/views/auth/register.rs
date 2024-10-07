use {
    yew::prelude::*,
    yew_router::prelude::*,
    serde::{Deserialize, Serialize},
    crate::{
        route::Route,
    },
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct RegisterForm {
    user_name: String,
    user_id: String,
    user_email: String,
    user_passwrod: String,
    check_passwrod: String,
    user_phone: String,
    remember: bool,
}
fn register_user(user: &RegisterForm) {

}

#[function_component(Register)]
pub fn register() -> Html {
    html! {
        <>
            <main>
                <section>
                    <form>
                        <div>
                            <label for="user_name">{"Name : "}</label>
                            <input type="text" name="user_name" placeholder="User Name" />
                        </div>
                        <div>
                            <label for="user_id">{"ID : "}</label>
                            <input type="text" name="user_id" placeholder="User Name" />
                        </div>
                        <div>
                            <label for="user_email">{"Email : "}</label>
                            <input type="text" name="user_email" placeholder="abc@abc.com" />
                        </div>
                        <div>
                            <label for="user_passwrod">{"Password : "}</label>
                            <input type="text" name="user_passwrod" placeholder="*********" />
                        </div>
                        <div>
                            <label for="check_passwrod">{"Confirm Password : "}</label>
                            <input type="text" name="check_passwrod" placeholder="*********" />
                        </div>
                        <div>
                            <label for="user_phone">{"Phone : "}</label>
                            <input type="text" name="user_phone" placeholder="010-1234-5678" />
                        </div>
                        <div>
                            <button>{"Forgot Password"}</button>
                            <input type="checkbox" name="remember" />{"Remember me"}
                        </div>
                        <div>
                            <button>{"Register"}</button>
                            <Link<Route> to={Route::Index}>{ "Cancel" }</Link<Route>>
                        </div>
                    </form>
                </section>
            </main>
        </>
    }
}