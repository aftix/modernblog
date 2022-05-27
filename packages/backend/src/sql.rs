use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};


#[derive(Debug, Clone, PartialEq, Eq, Hash, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub draft: bool,
    pub time: NaiveDateTime,
    pub header: Option<String>,
}

use crate::schema::posts;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Insertable)]
#[table_name = "posts"]
#[allow(clippy::extra_unused_lifetimes)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub draft: bool,
    pub time: NaiveDateTime,
    pub header: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Queryable)]
pub struct Image {
    pub id: i32,
    pub name: String,
    pub postid: i32,
}

use crate::schema::images;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Insertable)]
#[table_name = "images"]
pub struct NewImage {
    pub name: String,
    pub postid: i32,
}
