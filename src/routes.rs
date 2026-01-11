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
struct TodoState
{
    todos: Arc<Mutex<TodoList>>,
}

#[derive(Serialize, Deserialize)]
struct CreateRequest
{
    description: String,
}

pub fn api_routes() -> Router
{
    let shared_state = TodoState
    {
        todos: Arc::new(Mutex::new(TodoList::new())),
    };

    Router::new()
        .route("/", get(index).post(add_todo_handler))
        .fallback_service(tower_http::services::ServeDir::new("assets"))
        .with_state(shared_state)
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
    Form(payload): Form<CreateRequest>,
) -> ApiResult<impl IntoResponse>
{
    println!("---> {:<12} - add_todo - ", "HANDLER");

    let mut todos = state.todos.lock().unwrap();

    // let is_htmx = headers.contains_key("hx-request");
    //
    // if is_htmx
    // {
    //     Ok(Response::builder()
    //         .header("HX-Trigger", "add_todo")
    //         .body(body_contents)?
    //         .into_response())
    // }

    todos.add_todo(payload.description);
    Ok(Redirect::to("/"))
}

