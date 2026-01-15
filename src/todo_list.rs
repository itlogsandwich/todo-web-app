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

    pub fn remove_todo(&mut self, id: usize) -> TodoListResult<()>
    {
        if id > self.todos.len()
        {
            return Err(TodoError::IndexOutOfBounds);
        }
        
        self.todos.remove(id);

        Ok(())
    }

    pub fn update_todo(&mut self, id: usize, new_description: String) -> TodoListResult<()>
    {

        if id > self.todos.len()
        {
            return Err(TodoError::IndexOutOfBounds);
        }

        self.todos.get_mut(id).ok_or(TodoError::IndexOutOfBounds)?.edit_description(new_description);

        Ok(())
    }  

}
