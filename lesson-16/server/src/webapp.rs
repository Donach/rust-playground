use library::{Message, serialize_message, User};
use rocket::{self, get, launch, post, routes, delete, State, serde::json::Json, http::Status, fs::{FileServer, relative}, Config, Shutdown, Rocket, Ignite};
use sqlx::{Sqlite, Pool, Error};

use crate::db_client::{setup_database_pool, get_messages as db_get_messages, delete_user as db_delete_user, get_users as db_get_users};


#[get("/messages")]
async fn get_messages(db: &State<Pool<Sqlite>>) -> Json<Vec<Message>> {
    let db_get_messages = db_get_messages(db).await.unwrap();
    Json(db_get_messages)
    // Retrieve messages from the database
}

#[get("/users")]
async fn get_users(db: &State<Pool<Sqlite>>) -> Json<Vec<User>> {
    let db_get_users = db_get_users(db).await.unwrap();
    Json(db_get_users)
    // Retrieve messages from the database
}

#[post("/user/<uid>")]
async fn delete_user(uid: String, db: &State<Pool<Sqlite>>) -> Status {
    // Delete user and associated data from the database
    let res = db_delete_user(uid, db).await;
    match res {
        Ok(_) => Status::Ok,
        Err(_) => Status::NotFound,
    }
}


#[get("/shutdown")]
fn shutdown(shutdown: Shutdown) -> &'static str {
    shutdown.notify();
    "Shutting down..."
}



pub async fn main() {
    let res = rocket::build()
        .mount("/", routes![get_messages, get_users, delete_user])
        .manage(setup_database_pool().await.unwrap()) // Handle errors appropriately
        .mount("/", FileServer::from(relative!("static")))
        .launch()
        .await
        .expect("server failed unexpectedly");

    // If the server shut down (by visiting `/shutdown`), `result` is `Ok`.
    //res
}