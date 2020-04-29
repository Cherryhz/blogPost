extern crate diesel;
extern crate jsonwebtoken as jwt;
extern crate rustc_serialize;

use diesel::prelude::*;
use rocket::request::FromForm;
use crate::schema::posts;
use crate::schema::posts::dsl::{posts as all_posts};
//use bcrypt::{hash, DEFAULT_COST};
//use serde_derive::{Serialize, Deserialize};
//use jwt::{encode, decode, Header, Algorithm};
//use super::errors::MyStoreError;

#[derive(Debug, Queryable, PartialEq, Insertable)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub published: bool,
}

impl Post{
    /*
    pub fn all(conn: &SqliteConnection) -> Vec<Post> {
        all_posts.load::<Post>(conn).order(posts::id.desc()).expect("Error loading posts");
    }
    */

    pub fn insert(new: NewPost, conn: &SqliteConnection) -> bool {
        let p = Post{ id: None, title: new.title, content: new.content, published: false};
        diesel::insert_into(posts::table).values(&p).execute(conn).is_ok()
    }

    pub fn publish(conn: &SqliteConnection) -> bool {
        let post = all_posts.get_result::<Post>(conn);
        if post.is_err() {
            return false;
        }

        let new_status = !post.unwrap().published;
        
        diesel::update(all_posts)
            .set(posts::published.eq(new_status))
            .execute(conn)
            .is_ok()
    }
}

#[derive(FromForm)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}