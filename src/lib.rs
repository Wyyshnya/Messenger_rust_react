pub mod db;
pub mod auth_handler;

#[macro_use]
extern crate actix_web;
use actix_web::{error::Error, web, http, Responder, App, HttpServer, Result, HttpResponse, HttpRequest};
use actix_web::http::StatusCode;
use actix_web::web::block;
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::auth_handler::{AuthUser, reg};
use crate::db::db_pool;
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy, RequestIdentity};
use diesel::types::Json;
use crate::db::models::{Chats, MessageContent, Messages};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

#[derive(Deserialize, Serialize)]
pub struct SendChats {
    pub item: Vec<Chats>,
    last_messages: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct SendMessages {
    pub messages: Vec<Mess>,
}

#[derive(Deserialize, Serialize)]
pub struct Mess {
    id: i32,
    sender_id: String,
    date_send: chrono::NaiveDateTime,
    content: MessageContent,
}

#[derive(Deserialize)]
struct FormData {
    username: String,
    password: String,
}

pub struct MessageApp {
    port: u16,
}

 #[derive(Serialize, Deserialize,Debug)]
 struct PostData {
   message: String,
   id_user: String
 }

 #[derive(Serialize, Deserialize,Debug)]
 struct AuthData {
   username: String,
   passwd: String
 }

 #[derive(Serialize, Deserialize,Debug)]
 pub struct Token {
   pub token: String
 }


impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let conn_pool = db::establish_connection();
        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(move || {
            let cors = Cors::default()
              .allow_any_origin()
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
            App::new().data(conn_pool.clone()).wrap(IdentityService::new(CookieIdentityPolicy::new(&[0; 32])
                      .name("auth-cookie")
                      .secure(false))).wrap(cors)
                .service(conversations)
                .service(auth_front)
                .service(messages)
                .service(message_post)
                .service(decoding)

        })
        .bind(("127.0.0.1", self.port))?
        .workers(8).run().await
    }
}

#[post("api/conversations/{id}")]
async fn conversations(req: HttpRequest, pool: web::Data<db_pool>) -> HttpResponse {
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let conn = pool.get().unwrap();
    let user = auth_handler::decode_jwt(&id).unwrap();
    let ids = db::models::UsersChats::by_id(user.user_id, &conn);
    let mut chats = vec![];
    let mut contents = vec![];
    for i in ids.unwrap() {
        chats.push(db::models::Chats::by_id(i.chat_id, &conn).unwrap());
        let mut id = db::models::Messages::by_id(i.chat_id, &conn).unwrap();
        match id.pop() {
            Some(i) => {
                let content = db::models::MessageContent::by_id(i.content_id.unwrap(), &conn);
                contents.push(content.unwrap().content.unwrap());
            },
            None => contents.push("None".to_string())
        }
    };
    let send = SendChats {item: chats, last_messages: contents};
    HttpResponse::Ok().json(send)
}

#[post("api/messages/{id}")]
async fn messages(req: HttpRequest, pool: web::Data<db_pool>) -> HttpResponse {
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let conn = pool.get().unwrap();
    let mut messages = db::models::Messages::by_id(id, &conn).unwrap();
    let mut send = vec![];
    for message in &messages {
        let content = db::models::MessageContent::by_id(message.content_id.unwrap(), &conn).unwrap();
        let mess = Mess {
            id: message.id,
            sender_id: message.sender_id.as_ref().unwrap().to_string(),
            date_send: message.date_send.unwrap(),
            content,
        };
        send.push(mess);
    }
    // let send = SendChats {item: chats};
    HttpResponse::Ok().json(send)
}

#[post("api/message_post/{id}")]
async fn message_post(req: HttpRequest, data: web::Json<PostData>, pool: web::Data<db_pool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let user = auth_handler::decode_jwt(&data.id_user);
    db::models::MessageContent::push(Some(&data.message), Some("text".to_string()), &conn);
    let mut last_id = db::models::MessageContent::list(&conn);
    db::models::Messages::push(Some(id), Some(user.unwrap().user_id),
                               Some(chrono::Utc::now().naive_utc()), Some(last_id.pop().unwrap().id), &conn);
    HttpResponse::Ok().json("successful")
}

#[post("api/sign_in")]
async fn auth_front(id: Identity, req: HttpRequest, data: web::Json<AuthData>, pool: web::Data<db_pool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let username = &*data.username;
    let password = &*data.passwd;
    auth_handler::login(id, username, password, conn).await
}

#[post("api/decode_jwt")]
async fn decoding(req: HttpRequest, data: web::Json<Token>) -> HttpResponse {
    let user = auth_handler::decode_jwt(&data.token);
    HttpResponse::Ok().json(user.unwrap())
}
