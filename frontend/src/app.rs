use yew::prelude::*;
use yew_router::prelude::*;
use crate::router::{Route, switch};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            // <Switch<Route> render={Switch::render(switch)} />
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}