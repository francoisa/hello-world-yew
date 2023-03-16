use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

fn call_api(path: String, data: UseStateHandle<Option<u32>>) {
    spawn_local(async move {
        let resp = Request::post(format!("/api/{}/", path).as_str()).send().await.unwrap();
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

        data.set(Some(count))
    });
}

#[function_component(Counter)]
pub fn counter() -> Html {
    let data = use_state(|| Option::<u32>::None);
    let inc_data = data.clone();
    let oninc = Callback::from(move |_| {
        let inc_data = inc_data.clone();
        call_api("increment".to_string(), inc_data);
    });
    let dec_data = data.clone();
    let ondec = Callback::from(move |_| {
        let dec_data = dec_data.clone();
        call_api("decrement".to_string(), dec_data);
    });

    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::post("/api/counter/").send().await.unwrap();

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
                    { "Counter: " }
                    {data.map(|d|d.to_string()).unwrap_or_default()}
                </div>
                <div class="col">
                    <button onclick={oninc} type="button" class="btn btn-danger">{ "v" }</button>
                    { " " }
                    <button onclick={ondec} type="button" class="btn btn-success">{ "^" }</button>
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
