use poem::{
    get, handler, http::StatusCode, listener::TcpListener, middleware::Tracing, EndpointExt,
    IntoResponse, Response, Route, Server,
};

static JSON: &str = r#"
{
    "code":"0",
    "msg":"ok",
    "data":[]
}
"#;

#[handler]
fn hello() -> Response {
    Response::builder()
        .header("Content-Type", "application/json")
        .status(StatusCode::OK)
        .body(JSON.to_string())
        .into_response()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/hello", get(hello)).with(Tracing);
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("hello-world")
        .run(app)
        .await
}
