use serde_json::value::Value;
use serde_json::Map;
use actix_web::HttpRequest;

use crate::to_do;
use crate::state::read_file;
use crate::processes::process_input;

pub async fn create(req: HttpRequest) -> String {
    let state:Map<String, Value> = read_file(String::from("./state.json"));

    let title: String = req.match_info().get("title").unwrap().to_string();
}