use lazybe_macros::typed_url;

#[test]
fn static_url() {
    typed_url!(Root, "");
    assert_eq!(Root::axum_url(), "/");

    typed_url!(Ping, "ping");
    assert_eq!(Ping::axum_url(), "/ping");

    typed_url!(Healthcheck, "api" / "health");
    assert_eq!(Healthcheck::axum_url(), "/api/health");

    typed_url!(Book, "api" / "v1" / "books");
    assert_eq!(Book::axum_url(), "/api/v1/books");
}
