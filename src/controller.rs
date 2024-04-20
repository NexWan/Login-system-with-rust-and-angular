use actix_web::{get, post, web, Responder, Error};
use sqlx::{Row};
use actix_session::{Session};
use crate::{db, types::{User, Response}};


#[get("/")]
async fn hello() -> impl Responder {
    "Hello world!"
}

pub async fn manual_hello() -> impl Responder {
    let pool = db::get_connection().await.unwrap();
    let count = sqlx::query("SELECT COUNT(*) FROM test_table")
        .fetch_all(&pool)
        .await
        .unwrap();

    format!("{}", count[0].get::<i64, usize>(0)) //If this returns the amount of data into the table, then the connection is working
}

#[post("/api/login")]
pub async fn get_user(user: web::Json<User>, session: Session) -> Result<impl Responder, Error> {
    let username = user.username.lock().unwrap().clone();
    let password = user.password.lock().unwrap().clone();
    let conn = db::get_connection().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let rows = sqlx::query("SELECT * FROM test_table WHERE username = $1 AND password = $2")
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
pub async fn add_user(user: web::Json<User>, session:Session) -> Result<impl Responder, Error> {
    let username = user.username.lock().unwrap().clone();
    let password = user.password.lock().unwrap().clone();
    let conn = db::get_connection().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    sqlx::query("INSERT INTO test_table (username, password) VALUES ($1, $2)")
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
pub async fn verify_session(session: Session) -> Result<impl Responder, Error> {
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

//This is used to log out the user from the session
#[get("/api/logout")]
pub async fn logout(session: Session) -> Result<impl Responder, Error> {
    session.remove("user");
    Ok(web::Json(Response {
        status: "success".to_string(),
        message: "User logged out".to_string(),
    }))
}