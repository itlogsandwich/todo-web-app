use crate::todo::Todo;
use crate::error::TodoError;
use serde::{ Serialize, Deserialize };


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoList
{
    pub todos: Vec<Todo>,
}

impl TodoList
{
    pub fn new() -> Self
    {
        Self { todos: Vec::new() }
    }
    pub fn add_todo(&mut self, description: String)
    {
        let todo = Todo::new(description);    
        self.todos.push(todo);
    }

    pub fn update_todo(&mut self, description: String)
    {
        let todo = Todo::new(description);    
        self.todos.push(todo);
    }  

}
