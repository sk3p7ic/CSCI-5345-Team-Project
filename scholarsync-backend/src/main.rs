use std::sync::RwLock;

use actix_web::{App, HttpServer, web::Data};
use routes::{add_paper, edit_professor_desc, get_all_professors, get_professor, RouteHandlerData};

mod dataset;
mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data: RouteHandlerData = Data::new(RwLock::new(dataset::load_dataset("data.json")));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(get_all_professors)
            .service(get_professor)
            .service(edit_professor_desc)
            .service(add_paper)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
