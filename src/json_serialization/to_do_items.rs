use futures::future::{ready, Ready};
use actix_web::{Responder, HttpResponse, HttpRequest};
use actix_web::http::Error;
use serde::Serialize;
use serde::Deserialize;
use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_items_count: i8,
    pub done_items_count: i8
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct)
            }
        }
        let done_count: i8 = done_array_buffer.len() as i8;
        let pending_count: i8 = pending_array_buffer.len() as i8;

        return ToDoItems{
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_items_count: pending_count,
            done_items_count: done_count
        }
    }
}

impl Responder for ToDoItems {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok().content_type("application/json").body(body)))
    }
}

#[derive(Deserialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: String
}

