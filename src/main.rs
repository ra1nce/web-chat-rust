#[macro_use]
extern crate dotenv_codegen;

use askama::Template;
use axum::{
    routing::get,
    response::{IntoResponse, Response, Html},
    http::StatusCode, Router
};
use axum_extra::routing::SpaRouter;
use axum_extra::extract::cookie::Cookie;

use tower_cookies::{CookieManagerLayer, Cookies, cookie::time::OffsetDateTime};
use std::{net::SocketAddr, time::Duration};
use rand::prelude::*;

mod database;

use crate::database::Database;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    messages: Vec<(String, String, bool)>,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(index))
        .layer(CookieManagerLayer::new())
        .merge(SpaRouter::new("/static", "static").index_file("file_not_found.html"));

    let port: u16 = dotenv!("WEB_PORT").parse().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(cookies: Cookies) -> impl IntoResponse {
    let nickname = match cookies.get("nickname") {
        Some(c) => c.value().to_string(),
        None => {
            let mut time = OffsetDateTime::now_utc();
            time += Duration::from_secs(60 * 60 * 24 * 31);
            
            let nickname = create_nickname();

            let mut cookie = Cookie::new("nickname", create_nickname());
            cookie.set_expires(time);

            cookies.add(cookie);

            nickname
        },
    };
    
    let db = Database::new();
    let messages: Vec<(String, String, bool)> = db.get_messages().iter().map(|(nick, msg)| {(nick.to_owned(), msg.to_owned(), nick == &nickname)}).collect();

    HtmlTemplate(
        IndexTemplate { 
            messages,
        }
    )
}

fn create_nickname() -> String {
    let id = rand::thread_rng().gen_range(0..1000000);

    format!("Anon{id}")
}