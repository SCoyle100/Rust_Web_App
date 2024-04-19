
//this imports the mod.rs file in the /to_do directory
//which in turn gives us access to all of the structs and the enums.rs
//which gives us access to Taskstatus and its implementation block
//You can see, since we have access to mod structs, we have access to the mod.rs
//file in the /to_do/structs directory, which gives us access to base, done, and pending
//Thus, we are able to use Done and Pending

/* 
mod state;
mod to_do;
mod processes;
use std::env;
use serde_json::value::Value;
use serde_json::Map;
use state::read_file;
use to_do::to_do_factory;
use to_do::enums::TaskStatus;
use processes::process_input;


fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];
    let state: Map<String, Value> = read_file("./state.json");
    let status: String;
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None=> {
            status = "pending".to_owned();
        }
    }
    let item = to_do_factory(title, TaskStatus::from_string(status.to_uppercase()));
    process_input(item, command.to_string(), &state);
}
*/


use actix_web::{web, App, HttpServer, Responder, HttpRequest};



async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
HttpServer::new(|| {
    App::new()
    .route("/", web::get().to(greet))
    .route("/{name}", web::get().to(greet))
    .route("/say/hello", web::get().to(|| 
        async { "Hello Again!" }))
})
.bind("127.0.0.1:8080")?
.run()
.await
}

