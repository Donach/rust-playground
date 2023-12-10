use std::env;
use std::error::Error;

use crate::{serialize_message_as_bin, MessageType, get_timestamp, Message, deserialize_message, User, deserialize_message_as_bin};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;


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

/*     sqlx::query!(
        "DROP TABLE MESSAGES",
    )
    .execute(&pool)
    .await?; */

    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            uid TEXT NOT NULL,
            timestamp TEXT NOT NULL,
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
    let time = get_timestamp();
    sqlx::query!(
        "INSERT INTO messages (id, uid, timestamp, message) VALUES (?, ?, ?, ?)",
        message_id,
        uid,
        time,
        ser_message
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_users(db: &Pool<Sqlite>) -> Result<Vec<User>, sqlx::Error> {
    let raw_users = sqlx::query!("SELECT uid FROM users")
        .fetch_all(db)
        .await?;

    let mut users = Vec::new();
    for raw_user in raw_users {
        users.push(User {
            uid: raw_user.uid,
        })
    };
    Ok(users)
}
pub async fn get_messages(db: &Pool<Sqlite>) -> Result<Vec<Message>, sqlx::Error> {
    let raw_messages = sqlx::query!("SELECT id, uid, timestamp, message FROM messages")
        .fetch_all(db)
        .await?;

    let mut messages = Vec::new();
    for raw_msg in raw_messages {
        let msg = &raw_msg.message;
        let message_type = deserialize_message_as_bin(msg.as_ref().unwrap());
        if message_type.is_ok() {
            let message_type = message_type.unwrap();

            let id = &raw_msg.id.unwrap_or_else(|| "default_id".to_string());
            let uid = &raw_msg.uid;
            let timestamp = &raw_msg.timestamp;

            messages.push(Message {
                id: id.to_string(),
                uid: uid.to_string(),
                timestamp: timestamp.to_string(),
                message: message_type,
            });
        } else {
            log::error!("Error deserializing message: {}", message_type.err().unwrap());
        }
    }

    Ok(messages)
}

pub async fn delete_message(id: String, db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // Delete also any messages sent by this user
    sqlx::query("DELETE FROM messages WHERE id = $1")
        .bind(&id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn delete_user(uid: String, db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // Delete also any messages sent by this user
    sqlx::query("DELETE FROM messages WHERE uid = $1")
        .bind(&uid)
        .execute(db)
        .await?;

    sqlx::query("DELETE FROM users WHERE uid = $1")
        .bind(uid)
        .execute(db)
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
