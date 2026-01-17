use crate::todo::Todo;
use crate::templates::{ HtmlTemplate, IndexTemplate, TodoTemplate, TodoItemTemplate, UpdateTodoTemplate};
use crate::error::TodoError;
use axum::{ Router, Form };
use axum::routing::{ get, post, delete, patch};
use axum::response::{IntoResponse};
use axum::extract::{ State, Path };
use serde::{ Serialize, Deserialize};

type ApiResult<T> = Result<T, TodoError>;

#[derive(Clone)]
pub struct TodoState
{
    pub db: sqlx::PgPool
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
        .route("/todo/delete/{id}", delete(delete_todo_handler))
        .route("/todo/update/{id}", get(show_update_todo_form_handler).patch(update_todo_handler))
        .route("/todo/change/{id}", patch(change_status_handler))
        .fallback_service(tower_http::services::ServeDir::new("assets"))
        .with_state(state)
}

async fn index(
    State(state): State<TodoState>
) -> ApiResult<impl IntoResponse>
{
    println!("---> {:<12} - index - ", "HANDLER");
    
    let todos = sqlx::query_as!(
        Todo,
        "SELECT id, description, is_complete FROM todos ORDER BY id ASC"
    )
    .fetch_all(&state.db)
    .await?;

    let template = IndexTemplate 
    {
        todo_list: todos,
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

    sqlx::query!(
        "INSERT INTO todos (description) VALUES ($1)",
        payload.description
    )
    .execute(&state.db)
    .await?;

    let todos = sqlx::query_as!(
        Todo,
        "SELECT id, description, is_complete FROM todos ORDER BY id ASC"
    )
    .fetch_all(&state.db)
    .await?;

    let template = TodoTemplate
    {
        todo_list: todos
    };

    Ok(HtmlTemplate(template))
    
}

async fn show_update_todo_form_handler(
    Path(id): Path<i32>,
) -> impl IntoResponse
{
    let template = UpdateTodoTemplate
    {
        id
    };

    HtmlTemplate(template)
}

#[axum::debug_handler]
async fn update_todo_handler(
    State(state): State<TodoState>,
    Path(id): Path<i32>,
    Form(payload): Form<CreateRequest>,
) -> ApiResult<impl IntoResponse>
{
    println!("---> {:<12} - update_todo -", "HANDLER");

    sqlx::query!(
        "UPDATE todos SET description = $2 WHERE id = $1",
        id,
        payload.description,
    )
    .execute(&state.db)
    .await?;

    let updated_todo = sqlx::query_as!(
        Todo,
        "SELECT id, description, is_complete FROM todos WHERE id = $1",
        id
    )
    .fetch_one(&state.db)
    .await?;

    let template = TodoItemTemplate
    {
        todo: updated_todo,
    };

    Ok(HtmlTemplate(template))
}

#[axum::debug_handler]
async fn delete_todo_handler(
    State(state): State<TodoState>,
    Path(id): Path<i32>,
) -> ApiResult<()>
{
    println!("---> {:<12} - delete_todo -", "HANDLER");

    sqlx::query!(
        "DELETE FROM todos WHERE id = $1",
        id
    )
    .execute(&state.db)
    .await?;

    Ok(())
}

//marks task as done or not
#[axum::debug_handler]
async fn change_status_handler(
    State(state): State<TodoState>, 
    Path(id): Path<i32>,
) -> ApiResult<impl IntoResponse>
{
    println!("---> {:<12} - change_status -", "HANDLER");

    sqlx::query!(
        "UPDATE todos SET is_complete = NOT is_complete WHERE id = $1",
        id
    )
    .execute(&state.db)
    .await?;

    let updated_todo = sqlx::query_as!(
        Todo,
        "SELECT id, description, is_complete FROM todos ORDER BY id ASC"
    )
    .fetch_one(&state.db)
    .await?;

    let template = TodoItemTemplate
    {
        todo: updated_todo
    };

    Ok(HtmlTemplate(template))
}
