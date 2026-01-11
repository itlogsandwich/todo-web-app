use crate::todo::Todo;
use crate::error::TodoError;
use serde::{ Serialize, Deserialize };

type TodoListResult<T> = Result<T, TodoError>;

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

    pub fn remove_todo(&mut self, id: u64) -> TodoListResult<()>
    {
        if (id as usize) > self.todos.len() || (id as usize) < self.todos.len()
        {
            return Err(TodoError::IndexOutOfBounds);
        }
        
        self.todos.remove(id as usize);

        Ok(())
    }

    pub fn update_todo(&mut self, description: String)
    {
        let todo = Todo::new(description);    
        self.todos.push(todo);
    }  

}
