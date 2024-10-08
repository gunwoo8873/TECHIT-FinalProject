use {
    yew::prelude::*,
    yew_router::prelude::*,
    crate::views::{
        index::Index,
        notfound::NotFound,
        // auth::{signin::Signin, register::Register},
    },
};

#[derive(Debug, PartialEq, Clone, Routable)]
pub enum Route {
    #[at("/")]
    Index,

    #[not_found]
    #[at("/notfound")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! {<Index />},
        Route::NotFound => html! {<NotFound />},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            // <Switch<Route> render={Switch::render(switch)} />
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}

pub fn _run() {
    yew::Renderer::<App>::new().render();
}