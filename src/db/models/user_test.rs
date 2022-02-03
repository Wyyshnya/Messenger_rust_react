use serde::de::Unexpected::Option;
use crate::db::{establish_connection, models::{User, Chats}};

#[test]
fn create_user_with_phone_and_email() {
    let conn = establish_connection().get().unwrap();
    let username = Some("Pukich");
    let password = Some(&*"1337");
    let user = User::create(username, password, &conn).unwrap();

    assert_eq!(user.username.unwrap().as_str(), username.unwrap());
}

#[test]
fn get_user_by_id() {
    let conn = establish_connection().get().unwrap();
    let username = Some(&*"Puckich");
    let password = Some(&*"1337");
    let user = User::create(username, password, &conn).unwrap();
    let existing_user = User::by_id(&user.id , &conn).unwrap();
    assert_eq!(existing_user.username, user.username);
}

#[test]
fn cre() {
    let conn = establish_connection().get().unwrap();
    let title = Some("Pukich");
    let id_creator = Some("43d3671b-569f-4d7d-8597-c937ca1f05d5".to_string());
    let is_dialog = Some(true);
    Chats::create(title, id_creator, is_dialog, &conn);

    assert_eq!(title.unwrap(), title.unwrap());
}

#[test]
fn get() {
    let conn = establish_connection().get().unwrap();
    let chat_id = 1;
    let title = Some("Pukich".to_string());
    let existing_user = Chats::by_id(chat_id , &conn).unwrap();
    assert_eq!(existing_user.title, title);
}