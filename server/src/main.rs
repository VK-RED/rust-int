use std::sync::{Arc, Mutex};

use actix_web::{get,HttpServer, Responder};

use store::{todo::Todo, user::User};

pub mod handlers;
pub mod utils;
pub mod middleware;
pub mod errors;

#[get("/")]
async fn hello_world() -> impl Responder {
    "hello"
}

pub struct CombinedState{
    pub users: Vec<User>,
    pub todos: Vec<Todo>
}

#[derive(Clone)]
pub struct GlobalState{
    pub overall_state : Arc<Mutex<CombinedState>>,
}

const PORT :u16 = 8080;

#[macro_export]
macro_rules! init_app {
    ($overall_state:expr) => {
        actix_web::App::new()
        .app_data(actix_web::web::Data::new($overall_state.clone()))
        .service(crate::hello_world)
        .service(
            actix_web::web::scope("/user")
            .service(crate::handlers::user::signin)
            .service(crate::handlers::user::signup)
        )
        .service(
            actix_web::web::scope("/authed")
            .wrap(actix_web::middleware::from_fn(crate::middleware::middleware))
            .service(crate::handlers::todo::create_todo)
            .service(crate::handlers::todo::update_todo)
            .service(crate::handlers::todo::get_todos)
        )

    };
}

pub fn prepare_global_state() -> GlobalState{
    let combined_state = CombinedState{todos:vec![], users:vec![]};
    GlobalState{overall_state: Arc::new(Mutex::new(combined_state))}
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    let address = format!("127.0.0.1:{}", PORT);

    println!("Running on the port : {}", PORT);

    let state = prepare_global_state();

    HttpServer::new(move||{
        init_app!(state)
    })
    .bind(address)?
    .run()
    .await

}