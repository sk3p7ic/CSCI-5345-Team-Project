use std::sync::RwLock;

use crate::dataset::{Dataset, Paper};
use actix_web::{
    HttpResponse,
    web::{Data, Json, Path as WebPath},
};
use serde::Deserialize;

pub type RouteHandlerData = Data<RwLock<Dataset>>;

/// Returns a JSON list of all stored `Professor`s.
#[actix_web::get("/api/professors")]
pub async fn get_all_professors(data: RouteHandlerData) -> HttpResponse {
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
pub async fn get_professor(params: WebPath<(u32,)>, data: RouteHandlerData) -> HttpResponse {
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

#[derive(Deserialize)]
struct EditProfessorProps {
    name: String,
    dept: String,
    desc: String,
}

#[actix_web::put("/api/professors/{prof_id}")]
pub async fn edit_professor(
    params: WebPath<(u32,)>,
    form_body: Json<EditProfessorProps>,
    data: RouteHandlerData,
) -> HttpResponse {
    let (prof_id,) = params.into_inner();
    match data.write() {
        Ok(mut data) => {
            if let Some(professor) = data.get_mut(&prof_id) {
                professor.name = form_body.name.to_owned();
                professor.dept = form_body.dept.to_owned();
                professor.desc = form_body.desc.to_owned();
                HttpResponse::Ok().json(professor)
            } else {
                HttpResponse::NotFound().body("Professor not found.")
            }
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError().body("Could not edit professor.")
        }
    }
}

#[derive(Deserialize)]
struct AddPaperProps {
    title: String,
}

#[actix_web::post("/api/professors/{prof_id}/papers")]
pub async fn add_paper(
    params: WebPath<(u32,)>,
    form_body: Json<AddPaperProps>,
    data: RouteHandlerData,
) -> HttpResponse {
    let (prof_id,) = params.into_inner();
    match data.write() {
        Ok(mut data) => {
            if let Some(professor) = data.get_mut(&prof_id) {
                let next_id = professor.papers.iter().map(|p| p.id).max().unwrap_or(0) + 1;
                professor.papers.push(Paper {
                    id: next_id,
                    title: form_body.title.to_owned(),
                });
                HttpResponse::Ok().json(professor)
            } else {
                HttpResponse::BadRequest().body("Professor not found.")
            }
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError().body("Could not add paper.")
        }
    }
}

#[derive(Deserialize)]
struct EditPaperProps {
    title: String,
}

#[actix_web::put("/api/professors/{prof_id}/papers/{paper_id}")]
pub async fn edit_paper(
    params: WebPath<(u32, u32)>,
    form_body: Json<EditPaperProps>,
    data: RouteHandlerData,
) -> HttpResponse {
    let (prof_id, paper_id) = params.into_inner();
    match data.write() {
        Ok(mut data) => {
            if let Some(professor) = data.get_mut(&prof_id) {
                if let Some(paper) = professor
                    .papers
                    .iter_mut()
                    .find(|p| p.id.clone() == paper_id)
                {
                    paper.title = form_body.title.to_owned();
                    HttpResponse::Ok().json(professor)
                } else {
                    HttpResponse::NotFound().body("Paper not found.")
                }
            } else {
                HttpResponse::NotFound().body("Professor not found.")
            }
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError().body("Could not edit paper.")
        }
    }
}
