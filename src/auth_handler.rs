use actix_web::{web, Result, HttpResponse, HttpRequest, Error, FromRequest};
use actix_web::http::StatusCode;
use crate::db::hash_passwd::hash;
use crate::db::models;
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy, RequestIdentity};
use chrono;
use diesel::PgConnection;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;
use serde::{Deserialize, Serialize};
use std::fs;
use actix_web::error::ErrorBadRequest;
use chrono::{Duration, Utc};
use uuid::Uuid;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::Token;


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthUser {
    pub user_id: String,
    pub user_name: String,
    exp: i64,
}

impl AuthUser {
    pub fn new(user_id: String, user_name: String) -> Self {
        Self {
            user_id,
            user_name,
            exp: (Utc::now() + Duration::hours(10)).timestamp(),
        }
    }
}

pub async fn reg(username: Option<&str>, password: &str, conn: PooledConnection<ConnectionManager<PgConnection>>, tera: web::Data<tera::Tera>) -> HttpResponse {
    let passwd = hash(password);
    let hashed_passwd = Some(&*passwd);
    let contex = tera::Context::new();
    match models::User::create(username,  hashed_passwd, &conn) {
        Some(user) => HttpResponse::Ok().json("successful"),
        _ => HttpResponse::InternalServerError().json("Could not create user")
    }
}

pub async fn login(id: Identity, username: &str, password: &str, conn: PooledConnection<ConnectionManager<PgConnection>>) -> HttpResponse {
    // Validate that the email + hashed password matches
    let hashed = hash(password);
    match models::User::by_username(username, &conn) {
        Some(user) =>
            if hashed == user.password.unwrap() {
                let auth_user = AuthUser::new(user.id, user.username.unwrap());
                let token = create_jwt(auth_user);
                let tok = Token { token: token.as_ref().unwrap().to_string() };
                id.remember(token.unwrap());
                HttpResponse::Ok().json(tok)
            }
            else {
                HttpResponse::InternalServerError().json("Password doesn't match")
            }
        _ => HttpResponse::InternalServerError().json("User doesn't exists")
    }
}

// Encode a json web token (JWT)
pub fn create_jwt(private_claim: AuthUser) -> Result<String, Error> {
    let encoding_key = EncodingKey::from_secret("secret_key".as_ref());
    encode(
        &Header::default(),
        &private_claim,
        &encoding_key,
    )
    .map_err(|e| ErrorBadRequest(e.to_string()))
}

// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<AuthUser, Error> {
    let decoding_key = DecodingKey::from_secret("secret_key".as_ref());
    decode::<AuthUser>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ErrorBadRequest(e.to_string()))
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::InternalServerError().json("Vishel")

