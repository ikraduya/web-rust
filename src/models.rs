use super::schema::{one_user, book_review, comment};
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use chrono::naive::NaiveDateTime;

#[derive(Debug, Queryable, Deserialize, Insertable)]
#[table_name="one_user"]
pub struct LoginUser {
    pub username: String,
    pub passcode: String,
}

#[derive(Queryable, Deserialize, Serialize, Insertable)]
#[table_name="book_review"]
pub struct BookReview {
    pub id: i32,
    pub title: String,
    pub review: Option<String>,
    pub rating: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub author: String
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(primary_key(id))]
#[table_name="book_review"]
pub struct NewBookReview {
    #[diesel(deserialize_as = "i32")]
    pub id: Option<i32>,
    pub title: String,
    pub review: Option<String>,
    pub rating: f32,
    pub author: String
}

#[derive(Queryable, Insertable)]
#[table_name="comment"]
pub struct Comment {
    pub id: i32,
    pub username: String,
    pub content: String,
    pub review_id: i32
}
