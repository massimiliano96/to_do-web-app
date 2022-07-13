use actix_web::web;
mod path;
mod auth;
use std::env;

pub fn views_factory(app: &mut web::ServiceConfig) {
    let args: Vec<String> = env::args().collect();
    let params: &String = &args[args.len() -1];
    if params.as_str() == "cancel_logout" {
        println!("logout view isn't being configured");
        auth::auth_factory(app, false);
    }else{
        auth::auth_factory(app, true);
    }
}