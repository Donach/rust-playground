//! Webapp of chat application, used to visualize and control data.
//! 
//! # Usage
//! 
//! ```
//! cargo run --bin webapp
//! ```
//! 
//! The webapp runs on port 8000. It has very simple interface allowing to:
//! - view and delete data of users from the db
//! - delete specific messages
//! - load testing data into DB
use handlebars::{handlebars_helper, Handlebars};

use library::{Message, User, MessageType, db_client::save_message};
use rocket::{self, get, launch, post, routes, State, serde::{json::Json, Serialize}, http::Status, form::FromForm, uri};
use sqlx::{Sqlite, Pool, types::Uuid};
use rocket::{response::Redirect, form::Form};

use library::db_client::{setup_database_pool, get_messages_all as db_get_messages, delete_user as db_delete_user, get_users as db_get_users, delete_message as db_delete_message, get_messages_user as db_get_messages_user, auth_client};

use rocket_dyn_templates::{Template, handlebars};
use rocket::response::content::RawHtml;

fn generate_random_message() -> String {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let random_number: u32 = rng.gen();
    let message = format!("Message with random number {}", random_number);
    message
}

/// Dummy function to create test users and messages
#[post("/generate_test_data")]
async fn generate_test_data(db: &State<Pool<Sqlite>>) -> Result<Redirect, Status> {
    for _i in 0..3 {
        let _res = auth_client(db, Uuid::new_v4()).await;
    };


    let users = db_get_users(db).await.unwrap();
    for user in users {
        let uid = user.uid;
        let _res = auth_client(db, Uuid::new_v4());
        let msg = MessageType::Text(generate_random_message());
        let res = save_message(db, uid.to_string(), &msg).await;
        if res.is_err() {
            println!("Error saving message: {:?}", res);
        }
    }
    Ok(Redirect::to(uri!(index)))
}

/// Returns all messages of all users
#[get("/messages")]
async fn get_messages(db: &State<Pool<Sqlite>>) -> Json<Vec<Message>> {
    let db_get_messages = db_get_messages(db).await.unwrap();

    Json(db_get_messages)
    // Retrieve messages from the database
}

/// Returns all users
#[get("/users")]
async fn get_users(db: &State<Pool<Sqlite>>) -> Json<Vec<User>> {
    let db_get_users = db_get_users(db).await.unwrap();
    Json(db_get_users)
    // Retrieve messages from the database
}

#[derive(FromForm)]
struct UserForm {
    uid: String,
}

/// Deletes a user and all it's messages
#[post("/delete_user", data = "<user_form>")]
async fn delete_user(user_form: Form<UserForm>, db: &State<Pool<Sqlite>>) -> Result<Redirect, Status> {
    let res = db_delete_user(user_form.uid.clone(), db).await;
    match res {
        Ok(_) => Ok(Redirect::to(uri!(index))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[derive(FromForm)]
struct DeleteMessageForm {
    id: String,
}

/// Deletes a specific message
#[post("/delete_message", data = "<user_form>")]
async fn delete_message(user_form: Form<DeleteMessageForm>, db: &State<Pool<Sqlite>>) -> Result<Redirect, Status> {
    let res = db_delete_message(user_form.id.clone(), db).await;
    match res {
        Ok(_) => Ok(Redirect::to(uri!(index))),
        Err(_) => Err(Status::InternalServerError),
    }
}

/// Show only messages from given user by uid
#[get("/filter_messages?<uid>")]
async fn filter_messages (uid: String, db: &State<Pool<Sqlite>>) -> RawHtml<Template> {//-> Json<Vec<Message>> {
    let users = db_get_users(db).await.unwrap();
    let messages = db_get_messages_user(db, uid).await.unwrap();
    let context = Context { users, messages };
    RawHtml(Template::render("index", context))
}

#[derive(Serialize)]
struct Context {
    users: Vec<User>,
    messages: Vec<Message>,
}

#[get("/")]
async fn index(db: &State<Pool<Sqlite>>) -> RawHtml<Template> {
    let users = db_get_users(db).await.unwrap();
    let messages = db_get_messages(db).await.unwrap();
    let context = Context { users, messages };
    RawHtml(Template::render("index", context))
}

handlebars_helper!(message_as_str: |msg: MessageType| msg.to_string());


#[launch]
async fn webapp() -> _ {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let _dotenv = dotenvy::dotenv();

    rocket::build()
        //.attach(Template::fairing())
        .attach(Template::custom(move |engines| {
            let mut handlebars = Handlebars::new();
            let _ = handlebars.register_templates_directory(".hbs", "./webapp/templates");
            handlebars.register_helper("message_as_str", Box::new(message_as_str));
            engines.handlebars = handlebars;
        }))
        .mount("/", routes![index, get_messages, get_users, delete_user, delete_message, filter_messages, generate_test_data])
        .manage(setup_database_pool().await.unwrap())

    
}