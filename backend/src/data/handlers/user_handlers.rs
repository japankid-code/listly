use actix_web::web;
use diesel::{result::Error, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::MySqlPool;
use crate::MySqlPooledConnection;
use diesel::dsl::insert_into;

use crate::diesel_schema::users::dsl::*;
use std::vec::Vec;

use crate::user_models::{NewUser, User};

pub fn get_all_users(pool: web::Data<MySqlPool>) -> Vec<User> {
    let conn: &mut MySqlPooledConnection = &mut pool.get().unwrap();
    let items: Vec<User> = users.load::<User>(conn).expect("cant load users");
    items
}

// // Handler for GET /users/{id}
// pub async fn get_user_by_id(
//     db: web::Data<MySqlPool>,
//     user_id: web::Path<String>,
// ) -> Result<HttpResponse, Error> {
//     Ok(
//         web::block(move || db_get_user_by_id(db, user_id.into_inner()))
//             .await
//             .map(|user| HttpResponse::Ok().json(user))
//             .map_err(|_| HttpResponse::InternalServerError())?,
//     )
// }

// // Handler for DELETE /users/{id}
// pub async fn delete_user(
//     db: web::Data<MySqlPool>,
//     user_id: web::Path<i32>,
// ) -> Result<HttpResponse, Error> {
//     Ok(
//         web::block(move || delete_single_user(db, user_id.into_inner()))
//             .await
//             .map(|user| HttpResponse::Ok().json(user))
//             .map_err(|_| HttpResponse::InternalServerError())?,
//     )
// }

pub fn get_user_by_id(pool: web::Data<MySqlPool>, user_id: String) -> User {
    let conn: &mut MySqlPooledConnection = &mut pool.get().unwrap();
    let _user: Result<User, Error> = users.find(user_id).get_result::<User>(conn);

    _user.ok().unwrap()
}

pub fn add_single_user(pool: web::Data<MySqlPool>, item: web::Json<NewUser>) -> User {
    let conn: &mut MySqlPooledConnection = &mut pool.get().unwrap();
    let new_user: NewUser = NewUser {
        id: Uuid::new_v4().to_string(),
        first_name: item.first_name.to_string(),
        last_name: item.last_name.to_string(),
        email: item.email.to_string(),
    };

    let new_user_id = new_user.id.clone();

    let _insert = insert_into(users).values(new_user).execute(conn);

    let _user: Result<User, Error> = users.find(new_user_id).get_result::<User>(conn);

    _user.ok().unwrap()
}

// fn delete_single_user(
//     db: web::Data<MySqlPool>,
//     user_id: String,
// ) -> Result<usize, diesel::result::Error> {
//     let conn: MySqlPooledConnection = db.get().unwrap();
//     let count = delete(users.find(user_id)).execute(&conn)?;
//     Ok(count)
// }
