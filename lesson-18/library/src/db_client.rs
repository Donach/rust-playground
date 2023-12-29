//! Database client functions used by server (and webapp)
//! 
//! The functions in this file are used to communicate with the database
//! Most functionality is tested as well.
//! 
use std::env;
use std::error::Error;

use crate::{
    deserialize_message_as_bin, get_timestamp, serialize_message_as_bin, Message, MessageType, User,
};
use sqlx::sqlite::{SqlitePoolOptions, SqliteQueryResult};
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// Init function for database, returns a Pool used to connect to the database for further functions
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

/// Authenticate user - save it's UID into DB
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

/// Returns a list of all users
pub async fn get_users(db: &Pool<Sqlite>) -> Result<Vec<User>, sqlx::Error> {
    let raw_users = sqlx::query!("SELECT uid FROM users").fetch_all(db).await?;

    let mut users = Vec::new();
    for raw_user in raw_users {
        users.push(User { uid: raw_user.uid })
    }
    Ok(users)
}

/// Delete a single user and all it's messages
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


/// Save a message user sent to db as binary data
pub async fn save_message(
    pool: &Pool<Sqlite>,
    uid: String,
    message: &MessageType,
) -> Result<SqliteQueryResult, sqlx::Error> {
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
    let res = sqlx::query!(
        "INSERT INTO messages (id, uid, timestamp, message) VALUES (?, ?, ?, ?)",
        message_id,
        uid,
        time,
        ser_message
    )
    .execute(pool)
    .await;
    match res {
        Ok(res) => {
            Ok(res)
        }
        Err(err) => {
            log::error!("Error saving message: {}", err);
            Err(err)
        }
    }
}

/// Returns a list of all messages
pub async fn get_messages_all(db: &Pool<Sqlite>) -> Result<Vec<Message>, sqlx::Error> {
    let raw_messages = sqlx::query!("SELECT id, uid, timestamp, message FROM messages")
        .fetch_all(db)
        .await?;

    let mut messages = Vec::new();
    for raw_msg in raw_messages {
        let msg = &raw_msg.message;
        let message_type = deserialize_message_as_bin(msg.as_ref().unwrap());
        if let Ok(message_type) = message_type {
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
            log::error!(
                "Error deserializing message: {}",
                message_type.err().unwrap()
            );
        }
    }

    Ok(messages)
}

/// Returns a list of all messages of given user
pub async fn get_messages_user(db: &Pool<Sqlite>, uid: String) -> Result<Vec<Message>, sqlx::Error> {
    let raw_messages = sqlx::query!("SELECT id, uid, timestamp, message FROM messages where uid = ?", uid)
        .fetch_all(db)
        .await?;

    let mut messages = Vec::new();
    for raw_msg in raw_messages {
        let msg = &raw_msg.message;
        let message_type = deserialize_message_as_bin(msg.as_ref().unwrap());
        if let Ok(message_type) = message_type {
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
            log::error!(
                "Error deserializing message: {}",
                message_type.err().unwrap()
            );
        }
    }

    Ok(messages)
}

/// Delete a single message using message ID
pub async fn delete_message(id: String, db: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    // Delete also any messages sent by this user
    sqlx::query("DELETE FROM messages WHERE id = $1")
        .bind(&id)
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
