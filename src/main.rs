mod db;

use std::ptr::null;
use actix_web::{get, post, web, App, HttpServer, Responder, Error};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use sqlx::{Executor, Row};
use actix_session::{Session, SessionMiddleware, storage::{CookieSessionStore}};
use actix_web::cookie::Key;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello world!"
}

async fn manual_hello() -> impl Responder {
    let pool = db::get_connection().await.unwrap();
    let mut count = sqlx::query("SELECT COUNT(*) FROM test_table")
        .fetch_all(&pool)
        .await
        .unwrap();

    format!("{}", count[0].get::<i64, usize>(0)) //If this returns the amount of data into the table, then the connection is working
}

#[post("/api/login")]
async fn get_user(user: web::Json<User>, session: Session) -> Result<impl Responder, actix_web::Error> {
    let username = user.username.lock().unwrap().clone();
    let password = user.password.lock().unwrap().clone();
    let conn = db::get_connection().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let mut rows = sqlx::query("SELECT * FROM test_table WHERE username = $1 AND password = $2")
        .bind(&username)
        .bind(&password)
        .fetch_all(&conn)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?; //This is used to catch the error if the query fails
    if rows.len() == 0 {
        let confirm = Response {
            status: "error".to_string(),
            message: "User not found".to_string(),
        };
        session.insert("user", "")?;
        return Ok(web::Json(confirm))
    }
    let confirm = Response {
        status: "success".to_string(),
        message: "User found".to_string(),
    };
    session.insert("user", &username)?;
    Ok(web::Json(confirm))
}

#[post("/api/add_user")]
async fn add_user(user: web::Json<User>, session:Session) -> Result<impl Responder, actix_web::Error> {
    let username = user.username.lock().unwrap().clone();
    let password = user.password.lock().unwrap().clone();
    let conn = db::get_connection().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let mut rows = sqlx::query("INSERT INTO test_table (username, password) VALUES ($1, $2)")
        .bind(&username)
        .bind(&password)
        .execute(&conn)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?; //This is used to catch the error if the query fails
    let confirm = Response {
        status: "success".to_string(),
        message: "User added".to_string(),
    };
    Ok(web::Json(confirm))
}

#[get("/api/verify_session")]
async fn verify_session(session: Session) -> Result<impl Responder, actix_web::Error> {
    let user = session.get::<String>("user")?;
    match user {  //This is used to check if the user is in the session
        Some(user) => {
            Ok(web::Json(Response {
                status: "success".to_string(),
                message: user,
            }))
        }
        None => {
            Ok(web::Json(Response {
                status: "error".to_string(),
                message: "No user found".to_string(),
            }))
        }
    }
}

//This is used to logout the user from the session
#[get("/api/logout")]
async fn logout(session: Session) -> Result<impl Responder, actix_web::Error> {
    session.remove("user");
    Ok(web::Json(Response {
        status: "success".to_string(),
        message: "User logged out".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); //This is used to allow CORS from any origin

        App::new()
            .service(hello)//This is the default route to see if the server is running
            .route("/hey", web::get().to(manual_hello))
            .service(get_user)
            .service(add_user)
            .service(verify_session)
            .service(logout)
            .wrap(cors) // Wraps the cors around the app
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build()
            ) //This is used to create a session for the user
    })
    .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

//This struct is used to create a USER type which deserialize the JSON data coming from the front end
#[derive(Deserialize, Debug)]
struct User {
    username: Mutex<String>,
    password: Mutex<String>,
}

#[derive(Serialize, Debug)]
struct Response {
    status: String,
    message: String,
}