use crate::todo_list::TodoList;
use crate::templates::{ HtmlTemplate, IndexTemplate, AddTemplate};
use crate::error::TodoError;
use axum::{ Router, Form };
use axum::routing::{ get, post };
use axum::response::{IntoResponse , Response, Redirect };
use axum::extract::{ State };
use axum::http::HeaderMap;
use serde::{ Serialize, Deserialize};
use std::sync::{ Arc, Mutex };

type ApiResult<T> = Result<T, TodoError>;

#[derive(Clone)]
pub struct TodoState
{
   pub todos: Arc<Mutex<TodoList>>,
}

#[derive(Serialize, Deserialize)]
struct CreateRequest
{
    description: String,
}

pub fn api_routes(state: TodoState) -> Router
{
    Router::new()
        .route("/", get(index))
        .route("/todo", post(add_todo_handler))
        .fallback_service(tower_http::services::ServeDir::new("assets"))
        .with_state(state)
}

async fn index(
    State(state): State<TodoState>
) -> ApiResult<impl IntoResponse>
{
    println!("---> {:<12} - index - ", "HANDLER");
    
    let todos = state.todos.lock().unwrap();

    let template = IndexTemplate 
    {
        todo_list: todos.clone(),
    };

    Ok(HtmlTemplate(template))
}

#[axum::debug_handler]
async fn add_todo_handler(
    State(state): State<TodoState>,
    headers: HeaderMap,
    Form(payload): Form<CreateRequest>,
) -> ApiResult<impl IntoResponse>
{
    println!("---> {:<12} - add_todo - ", "HANDLER");

    let mut todos = state.todos.lock().unwrap();

    let is_htmx = headers.contains_key("hx-request");
    
    todos.add_todo(payload.description);
    if is_htmx
    {
        let template = AddTemplate 
        {
            todo_list: todos.clone(),
        };

        Ok(HtmlTemplate(template).into_response())
    }
    else
    {
        let template = IndexTemplate
        {
            todo_list: todos.clone(),
        };

        Ok(HtmlTemplate(template).into_response())
    }
}

async fn delete_todo_handler(
    State(state): State<TodoState>,
    headers: HeaderMap,
) -> ApiResult<impl IntoResponse>
{
    println!("---> {:<12} - remove_todo -", "HANDLER");

    let mut todos = state.todos.lock().unwrap();

    let is_htmx = headers.contains_key("hx-request");

    //hmmm how can I pass its index hmmm
    // todos.remove_todo()
    //
    Ok(())
}

