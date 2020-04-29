extern crate diesel;
extern crate jsonwebtoken as jwt;
extern crate rustc_serialize;
extern crate chrono;

use diesel::prelude::*;
//use rocket::request::FromForm;
use chrono::NaiveDateTime; // This type is used for date field in Diesel.
use crate::schema::users;
//use crate::schema::posts::dsl::{posts as all_posts};
use bcrypt::{hash, DEFAULT_COST};
use chrono::Local;
use serde_derive::{Serialize, Deserialize};
//use jwt::{encode, decode, Header, Algorithm};
use crate::errors::MyStoreError;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    #[serde(skip)] // we're removing id from being shown in the response
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip)] // we're removing password from being shown in the response
    pub password: String, 
    pub created_at: NaiveDateTime
}

impl User {
    pub fn create(register_user: RegisterUser, conn: &SqliteConnection) -> Result<User, MyStoreError> {
        //use diesel::RunQueryDsl;

        let new = NewUser{ email: register_user.email, password: Self::hash_password(register_user.password)?,
            created_at: Local::now().naive_local() };
        
        diesel::insert_into(users::table).values(&new).execute(conn)?;
        todo!()
    }

    pub fn hash_password(plain: String) -> Result<String, MyStoreError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub password: String, 
    pub created_at: NaiveDateTime
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
    pub password_confirmation: String
}

impl RegisterUser {
    pub fn validates (self) -> Result<RegisterUser, MyStoreError> {
        if self.password == self.password_confirmation {
            Ok(self)
        } else {
            Err(MyStoreError::PasswordNotMatch(
                "Passwords do not match".to_string()
            ))
        }
    }
}

#[derive(Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String
}

impl AuthUser {
     
    pub fn login(&self, connection: &SqliteConnection) -> Result<User, MyStoreError> {
        use bcrypt::verify;
        //use diesel::QueryDsl;
        //use diesel::RunQueryDsl;
        //use diesel::ExpressionMethods;
        use crate::schema::users::dsl::email;

        let mut records =
            users::table
                .filter(email.eq(&self.email))
                .load::<User>(connection)?;

        let user =
            records
                .pop()
                .ok_or(MyStoreError::DBError(diesel::result::Error::NotFound))?;

        let verify_password =
            verify(&self.password, &user.password)
                .map_err( |_error| {
                    MyStoreError::WrongPassword(
                        "Wrong password, check again please".to_string()
                    )
                })?;

        if verify_password {
            Ok(user)
        } else {
            Err(MyStoreError::WrongPassword(
                "Wrong password, check again please".to_string()
            ))
        }

    }
}


