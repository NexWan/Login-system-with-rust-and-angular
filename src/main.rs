mod db;
mod controller;
mod types;

use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use actix_session::{SessionMiddleware, storage::{CookieSessionStore}};
use actix_web::cookie::Key;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive(); //This is used to allow CORS from any origin
        App::new()
            .service(controller::hello)//This is the default route to see if the server is running
            .route("/hey", web::get().to(controller::manual_hello))
            .service(controller::get_user)
            .service(controller::add_user)
            .service(controller::verify_session)
            .service(controller::logout)
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