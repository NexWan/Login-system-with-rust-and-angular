use std::path::PathBuf;
use actix_web::{get, post, web, Responder, Error};
use sqlx::{Row};
use actix_session::{Session};
use crate::{db, types::{User, Response}};
use actix_files::NamedFile;
use actix_web::error::ParseError::Status;
use scrypt::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Scrypt
};
use log::error;


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
    let error =  Response {
        status: "failed".to_string(),
        message: "User or password incorrect".to_string(),
    };
    let username = user.username.lock().unwrap().clone();
    let password = user.password.lock().unwrap().clone();
    let conn = db::get_connection().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let rows = sqlx::query("SELECT username, password FROM test_table WHERE LOWER(username) = LOWER($1)")
        .bind(&username)
        .fetch_all(&conn)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?; //This is used to catch the error if the query fails
    if rows.len() == 0 {
        return Ok(web::Json(error))
    }
    let sql_pass = {rows[0].get::<String,usize>(1)}; // The password is obtained through the sql query, so we can compare it to it's hash
    let parse_hash = PasswordHash::new(&sql_pass).map_err(|e| actix_web::error::ErrorInternalServerError(e));
    if parse_hash.is_err(){
        session.insert("user", "")?;
        return Ok(web::Json(error))
    }
    let confirmation = Scrypt.verify_password(password.as_ref(), &parse_hash.unwrap()); // We compare both passwords, the original with the hashed one in the sql server to see if it matches
    if !confirmation.is_ok() { // If it doesn't match it will show an error
        return Ok(web::Json(error))
    }
    // If the previous flag wasn't checked that means it matches and we can assign it
    let username = { rows[0].get::<String,usize>(0) };
    let confirm = Response {
        status: "success".to_string(),
        message: format!("{}",username).to_string(),
    };
    Ok(web::Json(confirm))
}

#[post("/api/add_user")]
pub async fn add_user(user: web::Json<User>, session:Session) -> Result<impl Responder, Error> {
    let username = user.username.lock().unwrap().clone();
    let password = user.password.lock().unwrap().clone();
    let salt = SaltString::generate(&mut OsRng);
    let pass_hash = Scrypt.hash_password(password.as_ref(), &salt).map_err(|e| actix_web::error::ErrorInternalServerError(e)).unwrap().to_string();
    let conn = db::get_connection().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    sqlx::query("INSERT INTO test_table (username, password) VALUES ($1, $2)")
        .bind(&username)
        .bind(&pass_hash)
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

pub async fn not_found() -> Result<NamedFile, Error> {
    let path:PathBuf = "static/404.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}