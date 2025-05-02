use lazybe_macros::typed_uri;

#[test]
fn static_url() {
    typed_uri!(Root, "");
    assert_eq!(Root::AXUM_PATH, "/");
    assert_eq!(Root::AXUM_PATH, Root::new_url());

    typed_uri!(Ping, "ping");
    assert_eq!(Ping::AXUM_PATH, "/ping");
    assert_eq!(Ping::AXUM_PATH, Ping::new_url());

    typed_uri!(Healthcheck, "api" / "health");
    assert_eq!(Healthcheck::AXUM_PATH, "/api/health");
    assert_eq!(Healthcheck::AXUM_PATH, Healthcheck::new_url());

    typed_uri!(Book, "api" / "v1" / "books");
    assert_eq!(Book::AXUM_PATH, "/api/v1/books");
    assert_eq!(Book::AXUM_PATH, Book::new_url());
}

#[test]
fn dynamic_url() {
    typed_uri!(RootBook, (book_id: String));
    assert_eq!(RootBook::AXUM_PATH, "/{book_id}");
    assert_eq!(RootBook::new_url("my-book".to_string()), "/my-book");

    typed_uri!(Book, "api" / "books" / (book_id: u32));
    assert_eq!(Book::AXUM_PATH, "/api/books/{book_id}");
    assert_eq!(Book::new_url(123), "/api/books/123");

    typed_uri!(BookAuthor, "api" / "books" / (book_id: u32) / "authors");
    assert_eq!(BookAuthor::AXUM_PATH, "/api/books/{book_id}/authors");
    assert_eq!(BookAuthor::new_url(123), "/api/books/123/authors");

    typed_uri!(BookEditiion, "api" / "books" / (book_id: u32) / "editions" / (edition: u8));
    assert_eq!(BookEditiion::AXUM_PATH, "/api/books/{book_id}/editions/{edition}");
    assert_eq!(BookEditiion::new_url(123, 1), "/api/books/123/editions/1");

    typed_uri!(MovieEpisode, "api" / "movies" / (movie_id: u32) / (episode: u8));
    assert_eq!(MovieEpisode::AXUM_PATH, "/api/movies/{movie_id}/{episode}");
    assert_eq!(MovieEpisode::new_url(123, 2), "/api/movies/123/2");
}
