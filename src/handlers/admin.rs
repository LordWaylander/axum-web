use axum::response::Html;

pub async fn hello_fn() -> Html<&'static str> {
    Html("<h1>Hello Admin World!</h1>")
}

pub async fn truc_fn() -> Html<&'static str> {
    Html("<h1>Hello Admin truc!</h1>")
}