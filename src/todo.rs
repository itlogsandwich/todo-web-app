use serde::{ Serialize, Deserialize };

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo
{
    pub id: i32,
    pub description: String,
    pub is_complete: bool,
}

impl Todo 
{
    pub fn new(id: i32, description: String) -> Self
    {
        Self
        {
            id,
            description,
            is_complete: false,
        }
    }

    pub fn edit_description(&mut self, new_description: String)
    {
        self.description = new_description;
    }

    // pub fn toggle_status(&mut self)
    // {
    //     self.is_done = !self.is_done;
    // }
}
