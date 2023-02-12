use anyhow::Result;
use axum::Router;

fn router() -> Router {
    Router::new()
}

pub async fn listen_client_requests() -> Result<()> {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router().into_make_service())
        .await?;

    Ok(())
}
