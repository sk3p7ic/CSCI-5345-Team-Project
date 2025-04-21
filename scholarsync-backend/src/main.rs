use std::sync::RwLock;

use actix_web::{App, HttpServer, web::Data};
use routes::{
    RouteHandlerData, add_paper, add_professor, delete_paper, delete_professor, edit_professor,
    get_all_professors, get_papers, get_professor,
};

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
            .service(add_professor)
            .service(edit_professor)
            .service(delete_professor)
            .service(get_papers)
            .service(add_paper)
            .service(delete_paper)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
