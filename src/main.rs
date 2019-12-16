#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;

//use self::my_blog::*;
use rocket_contrib::templates::Template; 
use rocket::request::Form;
use rocket::response::{Flash, Redirect};
//use rocket::fairing::AdHoc;
use tera::Context;
use diesel::prelude::*;
use models::{Post, NewPost};
use schema::posts::dsl::*;
use serde_derive::Serialize;
use rocket_contrib::serve::StaticFiles;

fn main() {
  rocket::ignite()
    //.manage(establish_connection())
    .mount("/", routes![index, new, get_new, publish])
    .mount("/", StaticFiles::from("/Users/cherry/Documents/rust/blogPost/static"))
    .attach(Template::fairing())
    .launch();
}

#[derive(Clone, Debug, Serialize)]
pub struct TitleContent {
    pub title: String,
    pub content: String,
}

#[get("/")]
fn index() -> Template {
    let conn = establish_connection();
    let mut context = Context::new();
    let post_list = posts.load::<Post>(&conn).expect("Error loading posts");
    let mut v = Vec::new();
    for post in post_list {
        v.push(TitleContent { title: post.title.clone(), content: post.content.clone() });
    }
    context.insert("posts", &v);
    Template::render("post_summary", &context)
}

#[post("/submitpost", data = "<newpost_form>")]
fn get_new(newpost_form: Form<NewPost>) -> Result<Template, Flash<Redirect>> {
    let conn = establish_connection();
    let mut context = Context::new();
    let inner = newpost_form.into_inner();
    let new_title = &inner.title;
    context.insert("title", &inner.title);
    context.insert("content", &inner.content);
    if new_title.is_empty() {
        Err(Flash::error(Redirect::to("/newpost"), "Title must not be empty."))
    } else if Post::insert(inner, &conn) {
        Ok(Template::render("publish", &context))
    } else {
        Err(Flash::error(Redirect::to("/newpost"), "Whoops, server failure!"))
    }
}


#[get("/newpost")]
fn new() -> Template {
    Template::render("write", &Context::new())
}

#[get("/publish")]
fn publish() -> Result<Redirect, Template> {
    let conn = establish_connection();
    //let draft = posts.filter(published.eq(false)).first::<Post>(&conn);
    if Post::publish(&conn) {
        Ok(Redirect::to("/"))
    } else {
        Err(Template::render("publish", &Context::new()))
    }
}
