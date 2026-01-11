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
    pub fn update(&mut self)
    {
       
        if self.is_done
        {
            self.is_done = false;
        }

        self.is_done = true;
    }
}
