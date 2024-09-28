use yew::{function_component, html, Html};

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
        <header>
            <form action="/api/{id}" method="get">
                <div>
                    <a id="1" class="header__menu">{"Test"}</a>
                    <a id="2" class="header__menu">{"Test"}</a>
                    <a id="3" class="header__menu">{"Test"}</a>
                    <a id="4" class="header__menu">{"Test"}</a>
                    <a id="5" class="header__menu">{"Test"}</a>
                </div>
            </form>
        </header>
        <main>
            <section>
            </section>
        </main>
        <footer>
            <h1>{"TEST"}</h1>
        </footer>
        </>
    }
}