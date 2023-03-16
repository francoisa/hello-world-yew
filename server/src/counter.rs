use std::sync::{Arc, Mutex};

use axum::{extract::Extension, response::IntoResponse, routing::post, Router};

#[derive(Default)]
struct Counter {
    count: usize,
}

pub(crate) fn setup(router: Router) -> Router {
    let api_routes = Router::new()
        .nest("/counter", Router::new().route("/", post(counter)))
        .nest("/increment", Router::new().route("/", post(increment)))
        .nest("/decrement", Router::new().route("/", post(decrement)));
    router
        .nest("/api", api_routes)
        .layer(Extension(Arc::new(Mutex::new(Counter::default()))))
}

async fn increment(Extension(counter): Extension<Arc<Mutex<Counter>>>) -> impl IntoResponse {
    let mut counter = counter.lock().unwrap();
    counter.count += 1;
    format!("{}", counter.count)
}

async fn decrement(Extension(counter): Extension<Arc<Mutex<Counter>>>) -> impl IntoResponse {
    let mut counter = counter.lock().unwrap();
    counter.count -= 1;
    format!("{}", counter.count)
}


async fn counter(Extension(counter): Extension<Arc<Mutex<Counter>>>) -> impl IntoResponse {
    let counter = counter.lock().unwrap();
    format!("{}", counter.count)
}
