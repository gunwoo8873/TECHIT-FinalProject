use yew::{function_component, html, Html};

#[function_component(NotFound)]
pub fn notfound() -> Html {
    html! {
        <>
        <main>
            <section>
                <h1>{"NOT FOUND"}</h1>
            </section>
        </main>
        </>
    }
}