use lazybe_macros::typed_uri;
use serde::{Deserialize, Serialize};

#[test]
fn static_uri() {
    typed_uri!(Root, "");
    assert_eq!(Root::AXUM_PATH, "/");
    assert_eq!(Root::AXUM_PATH, Root::new_uri());

    typed_uri!(Ping, "ping");
    assert_eq!(Ping::AXUM_PATH, "/ping");
    assert_eq!(Ping::AXUM_PATH, Ping::new_uri());

    typed_uri!(Healthcheck, "api" / "health");
    assert_eq!(Healthcheck::AXUM_PATH, "/api/health");
    assert_eq!(Healthcheck::AXUM_PATH, Healthcheck::new_uri());

    typed_uri!(Book, "api" / "v1" / "books");
    assert_eq!(Book::AXUM_PATH, "/api/v1/books");
    assert_eq!(Book::AXUM_PATH, Book::new_uri());
}

#[test]
fn dynamic_uri() {
    typed_uri!(RootBook, (book_id: String));
    assert_eq!(RootBook::AXUM_PATH, "/{book_id}");
    assert_eq!(RootBook::new_uri("my-book".to_string()), "/my-book");

    typed_uri!(Book, "api" / "books" / (book_id: u32));
    assert_eq!(Book::AXUM_PATH, "/api/books/{book_id}");
    assert_eq!(Book::new_uri(123), "/api/books/123");

    typed_uri!(BookAuthor, "api" / "books" / (book_id: u32) / "authors");
    assert_eq!(BookAuthor::AXUM_PATH, "/api/books/{book_id}/authors");
    assert_eq!(BookAuthor::new_uri(123), "/api/books/123/authors");

    typed_uri!(BookEditiion, "api" / "books" / (book_id: u32) / "editions" / (edition: u8));
    assert_eq!(BookEditiion::AXUM_PATH, "/api/books/{book_id}/editions/{edition}");
    assert_eq!(BookEditiion::new_uri(123, 1), "/api/books/123/editions/1");

    typed_uri!(MovieEpisode, "api" / "movies" / (movie_id: u32) / (episode: u8));
    assert_eq!(MovieEpisode::AXUM_PATH, "/api/movies/{movie_id}/{episode}");
    assert_eq!(MovieEpisode::new_uri(123, 2), "/api/movies/123/2");
}

#[test]
fn query_param_uri() {
    #[derive(Deserialize, Serialize)]
    struct BookQuery {
        author: Option<String>,
        year_published: Option<u16>,
    }

    typed_uri!(Book, "api" / "books" ? Option<BookQuery>);
    assert_eq!(Book::AXUM_PATH, "/api/books");
    assert_eq!(Book::new_uri(None), "/api/books");
    assert_eq!(
        Book::new_uri(Some(BookQuery {
            author: None,
            year_published: None
        })),
        "/api/books"
    );
    assert_eq!(
        Book::new_uri(Some(BookQuery {
            author: Some("writer&friends".to_string()),
            year_published: None
        })),
        "/api/books?author=writer%26friends"
    );
    assert_eq!(
        Book::new_uri(Some(BookQuery {
            author: None,
            year_published: Some(2000)
        })),
        "/api/books?year_published=2000"
    );
    assert_eq!(
        Book::new_uri(Some(BookQuery {
            author: Some("writer&friends".to_string()),
            year_published: Some(2000)
        })),
        "api/books?author=writer%26friends&year_published=2000"
    );
}
