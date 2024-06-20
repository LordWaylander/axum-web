use axum::response::Html;

pub async fn show_users() -> Html<&'static str> {
    Html("<h1>Hello Admin World!</h1>")
}

pub async fn get_one_user() -> Html<&'static str> {
    Html("<h1>Hello Admin truc!</h1>")
}

pub async fn create_user() -> Html<&'static str> {
    Html("<h1>Hello Admin truc!</h1>")
}

pub async fn update_user() -> Html<&'static str> {
    Html("<h1>Hello Admin truc!</h1>")
}

pub async fn delete_user() -> Html<&'static str> {
    Html("<h1>Hello Admin truc!</h1>")
}
