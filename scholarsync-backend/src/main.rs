use std::sync::RwLock;

use actix_web::{
    App, HttpResponse, HttpServer,
    web::{Data, Path as WebPath},
};
use dataset::Dataset;

mod dataset;

/// Returns a JSON list of all stored `Professor`s.
#[actix_web::get("/api/professors")]
async fn get_all_professors(data: Data<RwLock<Dataset>>) -> HttpResponse {
    match data.read() {
        Ok(data) => {
            let mut professors = data.values().collect::<Vec<_>>();
            professors.sort_by_key(|p| p.id);
            HttpResponse::Ok().json(professors)
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError().body("Could not get list of professors.")
        }
    }
}

/// Returns a `Professor` with a given `prof_id`, if existent.
#[actix_web::get("/api/professors/{prof_id}")]
async fn get_professor(params: WebPath<(u32,)>, data: Data<RwLock<Dataset>>) -> HttpResponse {
    let (prof_id,) = params.into_inner();
    match data.read() {
        Ok(data) => {
            if let Some(professor) = data.get(&prof_id) {
                HttpResponse::Ok().json(professor)
            } else {
                HttpResponse::NotFound().body("Professor not found.")
            }
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError().body("Could not get list of professors.")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Data::new(RwLock::new(dataset::load_dataset("data.json")));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(get_all_professors)
            .service(get_professor)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
