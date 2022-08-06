use serde_json::value::Value;
use serde_json::Map;
use actix_web::{HttpRequest, Responder};

use crate::to_do;
use crate::state::read_file;
use crate::processes::process_input;
use super::utils::return_state;

pub async fn create(req: HttpRequest) -> impl Responder {
    let mut state: Map<String, Value> = read_file("./state.json");

    let title: String = req.match_info().get("title").unwrap().to_string();

    let item = to_do::to_do_factory(&String::from("pending"), &title).expect("create ");

    // add the to_do item to state.json
    process_input(item, String::from("create"), &mut state);

    return return_state()
}