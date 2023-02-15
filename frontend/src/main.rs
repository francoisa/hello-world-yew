use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/counter")]
    Counter,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Counter)]
fn counter() -> Html {
    let data = use_state(|| Option::<u32>::None);
    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::post("/api/counter").send().await.unwrap();

                    if !resp.ok() {
                        tracing::error!(
                            "Error fetching data {} ({})",
                            resp.status(),
                            resp.status_text()
                        );
                        return;
                    }

                    let content = match resp.text().await {
                        Err(err) => {
                            tracing::error!("Error fetching data {err}");
                            return;
                        }
                        Ok(content) => content,
                    };

                    let count = match content.parse() {
                        Err(err) => {
                            tracing::error!("Data is not a number: {err}");
                            return;
                        }
                        Ok(count) => count,
                    };

                    data.set(Some(count));
                });
            }

            || {}
        });
    }

    html! {
        <div class="container">
            <div class="row">
                <div class="col">
                    { "Count: " }
                    {data.map(|d|d.to_string()).unwrap_or_default()}
                </div>
                <div class="col">
                    <button type="button" class="btn btn-danger">{ "v" }</button>
                    { " " }
                    <button type="button" class="btn btn-success">{ "^" }</button>
                </div>
                <div class="col">{ " " }</div>
            </div>
            <div class="row">
                <div class="col">
                <a href="/">{ "Home" }</a>
                </div>
            </div>
        </div>
    }
}

// -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Counter => html! { <Counter /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <div class="container">
        <div class="row">
            <div class="col">
                <br/>
            </div>
        </div>
        <div class="row">
            <div class="col">
                { " " }
            </div>
            <div class="col">
                <div class="card" style="width: 18rem;">
                    <div class="card-body">
                        <h5 class="card-title">{ "Simple apps" }</h5>
                        <h6 class="card-subtitle mb-2 text-muted">{ "Apps in yew" }</h6>
                        <p class="card-text">{ "Here some links to simple apps written with yew." }</p>
                        <a href="/counter" class="card-link">{ "Counter app" }</a>
                        <a href="#" class="card-link">{ "Another link" }</a>
                    </div>
                </div>
            </div>
            <div class="col">
                { " " }
            </div>
        </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    yew::Renderer::<App>::new().render();
}
