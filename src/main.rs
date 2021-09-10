mod handlers;
mod dto;

use std::{error::Error,time::Duration};
use axum::{handler::{get, post}, Router};
use std::net::SocketAddr;
use sqlx::mysql::MySqlPoolOptions;
use tower::{BoxError, ServiceBuilder};
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};
use tower_http::trace::Trace;
use std::convert::Infallible;
use axum::http::StatusCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:123456@localhost/demo").await?;

    let app = Router::new()
        .route("/", get(handlers::hello))
        .route("/user/create", post(handlers::create_user))
        .layer(
            ServiceBuilder::new()
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(AddExtensionLayer::new(pool))
                .into_inner()
        )
        .handle_error(|error: BoxError| {
            let result = if error.is::<tower::timeout::error::Elapsed>() {
                Ok(StatusCode::REQUEST_TIMEOUT)
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                ))
            };

            Ok::<_, Infallible>(result)
        })
        // Make sure all errors have been handled
        .check_infallible();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

