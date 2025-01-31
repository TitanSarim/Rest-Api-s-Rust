use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::*;

#[derive(Queryable, Selectable)]
pub struct Rustacean{
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Queryable)]
pub struct Crates{
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}


#[derive(Insertable)]
#[diesel(table_name=crates)]
pub struct NewCrates{
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}