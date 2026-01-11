use crate::error::TodoError;
use axum::{ Router };

mod templates;
mod error;
mod todo;
mod todo_list;
mod routes;

#[tokio::main] async fn main() -> Result<(), TodoError> 
{
    let app = routes::api_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await?;

    println!("Server running on http://localhost:3000");
    axum::serve(listener, app)
        .await?;

    Ok(())
}
