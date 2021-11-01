#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::os::windows::process::ExitStatusExt;
use std::process::ExitStatus;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::tera::Context;
use serde::Serialize;
use rust_webserver::process_manager::process::ProcessState;
use rust_webserver::process_manager::process::ProcessState::{NotStarted, Running, Stopped};

#[derive(Serialize)]
struct ProcessListElement {
    name: String,
    state: String
}

impl ProcessListElement {
    fn new(name: String, state: ProcessState) -> Self {
        let state_str = match state {
            NotStarted => String::from("Not started"),
            Running => String::from("Running"),
            Stopped(code) => format!("Stopped with code {}", code)
        };

        ProcessListElement {
            name,
            state: state_str.to_string()
        }
    }
}

#[derive(Serialize)]
struct ProcessListContext {
    processes: Vec<ProcessListElement>
}

#[get("/")]
fn index() -> Template {
    let mut context = ProcessListContext {processes: vec![
        ProcessListElement::new(String::from("App1"), Running),
        ProcessListElement::new(String::from("App2"), NotStarted),
        ProcessListElement::new(String::from("App3"), Stopped(1)),
    ]};
    Template::render("index", context)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .ignite().await?
        .launch().await
}
