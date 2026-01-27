// Web Server with Rocket/Actix: Set up a basic HTTP server that
// serves static pages or a simple API (e.g., JSON endpoints for math calculations).
// Learn: Web frameworks, routing, async/await, and HTTP handling.
use std::io;

use rocket::fs::FileServer;
use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{Duration, sleep};
use rocket::form::FromForm;
use rocket::form::Form;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "
    Desde raiz
    "
}

#[get("/")]
fn world() -> &'static str {
    "
    Desde Hello World
    "
}

#[get("/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[get("/")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[derive(FromForm)]
struct Operation {
    operand: String,
    value_1: f32,
    value_2: f32,
}

#[post("/", data = "<operation>")]
async fn operation(operation: Form<Operation>) -> String {
    let operand = & operation.operand;
    let operand = operand.as_str();

    let result: String = match operand {
        "sum" => { (operation.value_1+operation.value_2).to_string() },
        "minus" => { (operation.value_1-operation.value_2).to_string() },
        "multiply" => {(operation.value_1*operation.value_2).to_string()},
        "division" => { (operation.value_1/operation.value_2).to_string() },
        _ => String::from("Not a valid input"),
    };

    format!("{}: {} con {} = {}", operand, operation.value_1, operation.value_2, result)
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/world", routes![world])
        .mount("/delay", routes![delay])
        .mount("/blocking_task", routes![blocking_task])
        .mount("/public", FileServer::from("www/static"))
        .mount("/operation", routes![operation])
}
