use serde::{ Serialize, Deserialize };

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo
{
    pub description: String,
    is_done: bool,
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

    pub fn edit_description(&mut self, new_description: String)
    {
        self.description = new_description;
    }

    pub fn toggle_status(&mut self)
    {
        self.is_done = !self.is_done;
    }
}
