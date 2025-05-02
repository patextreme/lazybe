use lazybe::axum::Router;
use lazybe::axum::extract::Path;
use lazybe::axum::routing::get;
use lazybe::macros::typed_uri;

typed_uri!(Healthcheck, "health");
typed_uri!(Echo, "echo" / (word: String) / (repetition: usize));

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route(Healthcheck::AXUM_PATH, get(healthcheck))
        .route(Echo::AXUM_PATH, get(echo));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server is listening on 0.0.0.0:8080");
    lazybe::axum::serve(listener, app).await?;
    Ok(())
}

async fn healthcheck() -> &'static str {
    "OK"
}

async fn echo(Path(path): Path<Echo>) -> String {
    std::iter::repeat_n(path.word.as_str(), path.repetition)
        .collect::<Vec<_>>()
        .join("\n")
}
