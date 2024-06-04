use axum::response::Html;

pub async fn hello_fn() -> &'static str {
    "<h1>Hello Admin world!</h1>"
}

pub async fn truc_fn() -> Html<&'static str> {
    Html("<h1>Hello Admin truc!</h1>")
}