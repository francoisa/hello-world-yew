use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(Counter)]
pub fn counter() -> Html {
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
