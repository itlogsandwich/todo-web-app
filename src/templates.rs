use crate::error::TodoError;
use crate::todo::Todo;
use askama::Template;
use axum::response::{ IntoResponse, Response, Html } ;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate
{
    pub todo_list: Vec<Todo>
}

#[derive(Template)]
#[template(path = "components/todo.html")]
pub struct TodoTemplate
{
    pub todo_list: Vec<Todo>
}


#[derive(Template)]
#[template(path = "components/todo_item.html")]
pub struct TodoItemTemplate
{
    pub todo: Todo,
}

#[derive(Template)]
#[template(path = "components/update.html")]
pub struct UpdateTodoTemplate
{
    pub id: i32
}

pub struct HtmlTemplate<T>(pub T);

impl <T> IntoResponse for HtmlTemplate<T>
    where T: Template,
{
    fn into_response(self) -> Response
    {
        match self.0.render()
        {
            Ok(html) => Html(html).into_response(),
            Err(err) => 
            {
                println!("Error: {err}");
                (TodoError::InternalServer(err.to_string())).into_response()
            }
        }
    }

}
