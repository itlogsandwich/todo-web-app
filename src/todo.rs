use serde::{ Serialize, Deserialize };

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo
{
    pub description: String,
    is_done: bool,
}

struct IdGenerator
{
    
}
impl Todo 
{
    pub fn new(description: String) -> Self
    {
        Self
        {
            description,
            is_done: false,
        }
    }
    pub fn toggle_status(&mut self)
    {
        self.is_done = !self.is_done;
    }
}
