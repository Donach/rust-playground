use handlebars::{handlebars_helper, Handlebars, JsonRender};
use serde_json::{json, Value};

use library::{Message, serialize_message, User, MessageType};
use rocket::{self, get, launch, post, routes, delete, State, serde::{json::Json, Serialize}, http::Status, fs::{FileServer, relative}, Config, Shutdown, Rocket, Ignite, form::FromForm, uri};
use sqlx::{Sqlite, Pool, Error};
use rocket::{response::{Redirect}, form::Form};

use library::db_client::{setup_database_pool, get_messages as db_get_messages, delete_user as db_delete_user, get_users as db_get_users, delete_message as db_delete_message};

use rocket_dyn_templates::{Template, context, handlebars};
use rocket::response::content::RawHtml;
use simple_logger;

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

#[derive(FromForm)]
struct DeleteUserForm {
    uid: String,
}

#[post("/delete_user", data = "<user_form>")]
async fn delete_user(user_form: Form<DeleteUserForm>, db: &State<Pool<Sqlite>>) -> Result<Redirect, Status> {
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

#[post("/delete_message", data = "<user_form>")]
async fn delete_message(user_form: Form<DeleteMessageForm>, db: &State<Pool<Sqlite>>) -> Result<Redirect, Status> {
    let res = db_delete_message(user_form.id.clone(), db).await;
    match res {
        Ok(_) => Ok(Redirect::to(uri!(index))),
        Err(_) => Err(Status::InternalServerError),
    }
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
    RawHtml(Template::render("index", &context))
}

handlebars_helper!(message_as_str: |msg: MessageType| msg.to_string());


#[launch]
async fn webapp() -> _ {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    let _dotenv = dotenvy::dotenv();

    // create the handlebars registry
    // register some custom helpers
  

    rocket::build()
        //.attach(Template::fairing())
        .attach(Template::custom(move |engines| {
            let mut handlebars = Handlebars::new();
            handlebars.register_templates_directory(".hbs", "./webapp/templates");
            handlebars.register_helper("message_as_str", Box::new(message_as_str));
            engines.handlebars = handlebars;
        }))
        .mount("/", routes![index, get_messages, get_users, delete_user, delete_message])
        .manage(setup_database_pool().await.unwrap())

    
}