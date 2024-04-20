mod db;

use actix_web::{get, post, web, App, HttpServer, Responder};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use serde::de::Error;
use sqlx::{Executor, Row};

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
async fn get_user(user: web::Json<User>) -> Result<impl Responder, actix_web::Error> {
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
        return Ok(web::Json(confirm))
    }
    let confirm = Response {
        status: "success".to_string(),
        message: "User found".to_string(),
    };
    Ok(web::Json(confirm))
}

#[post("/api/add_user")]
async fn add_user(user: web::Json<User>) -> Result<impl Responder, actix_web::Error> {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); //This is used to allow CORS from any origin

        App::new()
            .service(hello)//This is the default route to see if the server is running
            .route("/hey", web::get().to(manual_hello))
            .service(get_user)
            .service(add_user)
            .wrap(cors) // Wraps the cors around the app
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