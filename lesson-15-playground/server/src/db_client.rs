use std::env;
use std::error::Error;

use library::{serialize_message_as_bin, MessageType};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
/*
struct User {
    uid: Uuid,
}

struct Message {
    id: Uuid,
    uid: Uuid,
    message: String, // Deserializes into MessageType
}
*/
pub async fn setup_database_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .connect(&env::var("DATABASE_URL").unwrap())
        .await?;

    // Creating tables if they don't exist
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS users (
            uid TEXT NOT NULL UNIQUE PRIMARY KEY
         )",
    )
    .execute(&pool)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            uid TEXT NOT NULL,
            message BLOB,
            FOREIGN KEY(uid) REFERENCES users(uid)
         )",
    )
    .execute(&pool)
    .await?;

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS message_views (
            id TEXT PRIMARY KEY,
            uid TEXT NOT NULL,
            FOREIGN KEY(id) REFERENCES messages(id),
            FOREIGN KEY(uid) REFERENCES users(uid)
         )",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

// Authenticate user - save it's UID into DB
pub async fn auth_client(pool: &Pool<Sqlite>, uid: Uuid) -> Result<String, Box<dyn Error>> {
    // Insert user if not exists
    let uid = uid.to_string();
    sqlx::query!(
        "INSERT INTO users (uid) VALUES (?)
         ON CONFLICT(uid) DO NOTHING",
        uid
    )
    .execute(pool)
    .await?;

    Ok(uid)
}

// Save a message user sent
pub async fn save_message(
    pool: &Pool<Sqlite>,
    uid: String,
    message: &MessageType,
) -> Result<(), Box<dyn Error>> {
    // Insert user if not exists

    /*let uid = sqlx::query!(
           "INSERT INTO users (uid) VALUES (?)
            ON CONFLICT(uid) DO UPDATE SET uid=excluded.uid
            RETURNING uid",
           uid
       )
       .fetch_one(&pool)
       .await?
       .uid;
    */
    // Insert message
    let ser_message = serialize_message_as_bin(message).unwrap();
    let message_id: String = Uuid::new_v4().to_string();
    sqlx::query!(
        "INSERT INTO messages (id, uid, message) VALUES (?, ?, ?)",
        message_id,
        uid,
        ser_message
    )
    .execute(pool)
    .await?;

    Ok(())
}
/*
#[derive(sqlx::FromRow, Debug)]
struct AllMessages{
    message: String,
}

// This funciton will load all unseen messages since last time client was connected to server
pub async fn load_all_messages(
    pool: Pool<Sqlite>,
    uid: String,
) -> Result<Vec<String>, Box<dyn Error>> {
    // Insert user if not exists
    let mut missed_msgs: Vec<String> = vec![];

    let res = sqlx::query_as::<_, AllMessages>(
        "SELECT message from messages where id not in (SELECT id from message_views where uid = ?)"
    )
    .bind(uid)
    .fetch_all(&pool)
    .await?;

    for msg in res{
        let msg = msg.message;
        missed_msgs.push(msg);
    }

    Ok(missed_msgs)
}

// This funciton will return given message from DB
pub async fn load_single_message(
    pool: Pool<Sqlite>,
    id: String,
) -> Result<String, Box<dyn Error>> {
    let res = sqlx::query_as::<_, AllMessages>(
        "SELECT message from messages where id = ?"
    )
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(res.message)
}
*/
