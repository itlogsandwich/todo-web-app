use crate::error::TodoError;
use crate::todo_list::TodoList;
use axum::{ Router };
use std::sync::{ Arc, Mutex };

mod templates;
mod error;
mod todo;
mod todo_list;
mod routes;

#[tokio::main] 
async fn main() -> Result<(), TodoError> 
{
    let shared_state = routes::TodoState
    {
        todos: Arc::new(Mutex::new(TodoList::new())),
    };

    let app = routes::api_routes(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await?;

    println!("Server running on http://localhost:3000");
    axum::serve(listener, app)
        .await?;

    Ok(())
}
