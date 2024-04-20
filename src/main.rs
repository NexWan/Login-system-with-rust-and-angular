mod db;

use actix_web::{get, post, web, App, HttpServer, Responder};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
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

#[post("/api/user")]
async fn get_user(user: web::Json<User>) -> impl Responder {
    format!("Welcome! {:?}",user.name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); //This is used to allow CORS from any origin

        App::new()
            .service(hello)//This is the default route to see if the server is running
            .route("/hey", web::get().to(manual_hello))
            .service(get_user)
            .wrap(cors) // Wraps the cors around the app
    })
    .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

//This struct is used to create a USER type which deserialize the JSON data coming from the front end
#[derive(Deserialize, Debug)]
struct User {
    id: Mutex<i32>,
    name: Mutex<String>,
    pwd: Mutex<String>,
}