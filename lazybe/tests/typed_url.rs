use lazybe_macros::typed_url;

#[test]
fn static_url() {
    typed_url!(Root, "");
    assert_eq!(Root::AXUM_URL, "/");
    assert_eq!(Root::AXUM_URL, Root::new_url());

    typed_url!(Ping, "ping");
    assert_eq!(Ping::AXUM_URL, "/ping");
    assert_eq!(Ping::AXUM_URL, Ping::new_url());

    typed_url!(Healthcheck, "api" / "health");
    assert_eq!(Healthcheck::AXUM_URL, "/api/health");
    assert_eq!(Healthcheck::AXUM_URL, Healthcheck::new_url());

    typed_url!(Book, "api" / "v1" / "books");
    assert_eq!(Book::AXUM_URL, "/api/v1/books");
    assert_eq!(Book::AXUM_URL, Book::new_url());
}

#[test]
fn dynamic_url() {
    typed_url!(RootBook, (book_id: String));
    assert_eq!(RootBook::AXUM_URL, "/{book_id}");
    assert_eq!(RootBook::new_url("my-book".to_string()), "/my-book");

    typed_url!(Book, "api" / "books" / (book_id: u32));
    assert_eq!(Book::AXUM_URL, "/api/books/{book_id}");
    assert_eq!(Book::new_url(123), "/api/books/123");

    typed_url!(BookAuthor, "api" / "books" / (book_id: u32) / "authors");
    assert_eq!(BookAuthor::AXUM_URL, "/api/books/{book_id}/authors");
    assert_eq!(BookAuthor::new_url(123), "/api/books/123/authors");

    typed_url!(BookEditiion, "api" / "books" / (book_id: u32) / "editions" / (edition: u8));
    assert_eq!(BookEditiion::AXUM_URL, "/api/books/{book_id}/editions/{edition}");
    assert_eq!(BookEditiion::new_url(123, 1), "/api/books/123/editions/1");

    typed_url!(MovieEpisode, "api" / "movies" / (movie_id: u32) / (episode: u8));
    assert_eq!(MovieEpisode::AXUM_URL, "/api/movies/{movie_id}/{episode}");
    assert_eq!(MovieEpisode::new_url(123, 2), "/api/movies/123/2");
}
