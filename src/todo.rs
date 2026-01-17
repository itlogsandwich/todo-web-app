use serde::{ Serialize, Deserialize };

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo
{
    pub id: i32,
    pub description: String,
    pub is_complete: bool,
}
