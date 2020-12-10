use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize)]
pub struct Output(pub bool, pub String, pub String);
/*
pub struct Row{
    pub success: bool,
    pub code: String,
    pub user: String
}
*/