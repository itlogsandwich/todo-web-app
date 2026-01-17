use crate::error::TodoError;

mod todo;
mod db;
mod templates;
mod error;
mod routes;

#[tokio::main] 
async fn main() -> Result<(), TodoError> 
{
    dotenvy::dotenv().ok();
    let shared_state = routes::TodoState
    {
        db: db::connect_to_db().await?
    };

    sqlx::migrate!().run(&shared_state.db).await.unwrap();

    let app = routes::api_routes(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await?;

    println!("Server running on http://localhost:3000");
    axum::serve(listener, app)
        .await?;

    Ok(())
}
