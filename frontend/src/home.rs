use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
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
