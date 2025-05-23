use std::sync::RwLock;

use crate::dataset::{Dataset, Paper, Professor};
use actix_web::{
    HttpResponse,
    web::{Data, Json, Path as WebPath},
};
use serde::{Deserialize, Serialize};

pub type RouteHandlerData<'data> = Data<RwLock<Dataset<'data>>>;

#[derive(Serialize)]
struct RouteResponseMessage {
    message: &'static str,
}

impl std::convert::From<&'static str> for RouteResponseMessage {
    fn from(value: &'static str) -> Self {
        Self { message: value }
    }
}

/// Returns a JSON list of all stored `Professor`s.
#[actix_web::get("/api/professors")]
pub async fn get_all_professors(data: RouteHandlerData<'static>) -> HttpResponse {
    match data.read() {
        Ok(data) => {
            let mut professors = data.0.values().collect::<Vec<_>>();
            professors.sort_by_key(|p| p.id);
            HttpResponse::Ok().json(professors)
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError().json(RouteResponseMessage::from(
                "Could not get list of professors.",
            ))
        }
    }
}

#[derive(Deserialize)]
struct AddProfessorProps {
    name: String,
    dept: String,
    desc: String,
}

#[actix_web::post("/api/professors")]
pub async fn add_professor(
    form_body: Json<AddProfessorProps>,
    data: RouteHandlerData<'static>,
) -> HttpResponse {
    match data.write() {
        Ok(mut data) => {
            let props = form_body.into_inner();
            let next_id: u32 = {
                let mut professors = data.0.values().collect::<Vec<_>>();
                professors.sort_by_key(|p| p.id);
                professors.into_iter().map(|p| p.id).last().unwrap_or(0) + 1
            };
            let professor = Professor {
                id: next_id,
                name: props.name,
                dept: props.dept,
                desc: props.desc,
                papers: Vec::new(),
            };
            data.0.insert(next_id, professor.clone());
            if let Err(err) = data.save_state() {
                eprintln!("Could not save data state: {err}");
            }
            HttpResponse::Ok().json(professor)
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError()
                .json(RouteResponseMessage::from("Could not add professor."))
        }
    }
}

/// Returns a `Professor` with a given `prof_id`, if existent.
#[actix_web::get("/api/professors/{prof_id}")]
pub async fn get_professor(
    params: WebPath<(u32,)>,
    data: RouteHandlerData<'static>,
) -> HttpResponse {
    let (prof_id,) = params.into_inner();
    match data.read() {
        Ok(data) => {
            if let Some(professor) = data.0.get(&prof_id) {
                HttpResponse::Ok().json(professor)
            } else {
                HttpResponse::NotFound().json(RouteResponseMessage::from("Professor not found."))
            }
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError().json(RouteResponseMessage::from(
                "Could not get list of professors.",
            ))
        }
    }
}

#[derive(Deserialize)]
struct EditProfessorProps {
    name: String,
    dept: String,
}

#[actix_web::patch("/api/professors/{prof_id}")]
pub async fn edit_professor(
    params: WebPath<(u32,)>,
    form_body: Json<EditProfessorProps>,
    data: RouteHandlerData<'static>,
) -> HttpResponse {
    let (prof_id,) = params.into_inner();
    match data.write() {
        Ok(mut data) => {
            if let Some(professor) = data.0.get_mut(&prof_id) {
                professor.name = form_body.name.to_owned();
                professor.dept = form_body.dept.to_owned();
            } else {
                return HttpResponse::NotFound()
                    .json(RouteResponseMessage::from("Professor not found."));
            }
            match data.save_state() {
                Ok(()) => match data.0.get(&prof_id) {
                    Some(professor) => HttpResponse::Ok().json(professor),
                    None => HttpResponse::NotFound()
                        .json(RouteResponseMessage::from("Professor not found.")),
                },
                Err(err) => {
                    eprintln!("Could not save data state: {err}");
                    HttpResponse::Accepted().json(RouteResponseMessage::from(
                        "Professor was edited in memory, but the edit has not been committed.",
                    ))
                }
            }
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError()
                .json(RouteResponseMessage::from("Could not edit professor."))
        }
    }
}

#[actix_web::delete("/api/professors/{prof_id}")]
pub async fn delete_professor(
    params: WebPath<(u32,)>,
    data: RouteHandlerData<'static>,
) -> HttpResponse {
    let (prof_id,) = params.into_inner();
    match data.write() {
        Ok(mut data) => {
            if data.0.remove(&prof_id).is_none() {
                return HttpResponse::NotFound()
                    .json(RouteResponseMessage::from("Professor not found."));
            }
            match data.save_state() {
                Ok(_) => HttpResponse::NoContent().finish(),
                Err(_) => HttpResponse::Accepted().json(RouteResponseMessage::from(
                    "Professor was deleted in memory, but the deletion has not been committed.",
                )),
            }
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError()
                .json(RouteResponseMessage::from("Could not delete professor."))
        }
    }
}

#[actix_web::get("/api/professors/{prof_id}/papers")]
pub async fn get_papers(params: WebPath<(u32,)>, data: RouteHandlerData<'static>) -> HttpResponse {
    let (prof_id,) = params.into_inner();
    match data.read() {
        Ok(data) => {
            if let Some(professor) = data.0.get(&prof_id) {
                HttpResponse::Ok().json(professor.papers.clone())
            } else {
                HttpResponse::NotFound().json(RouteResponseMessage::from("Professor not found."))
            }
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError()
                .json(RouteResponseMessage::from("Could not get papers."))
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
    data: RouteHandlerData<'static>,
) -> HttpResponse {
    let (prof_id,) = params.into_inner();
    match data.write() {
        Ok(mut data) => {
            if let Some(professor) = data.0.get_mut(&prof_id) {
                let next_id = professor.papers.iter().map(|p| p.id).max().unwrap_or(0) + 1;
                professor.papers.push(Paper {
                    id: next_id,
                    title: form_body.title.to_owned(),
                });
                let res = match professor.papers.iter().last() {
                    Some(paper) => HttpResponse::Ok().json(paper),
                    None => HttpResponse::InternalServerError()
                        .json(RouteResponseMessage::from("Could not get added paper.")),
                };
                if let Err(err) = data.save_state() {
                    eprintln!("Could not save data state: {err}");
                }
                res
            } else {
                HttpResponse::BadRequest().json(RouteResponseMessage::from("Professor not found."))
            }
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError()
                .json(RouteResponseMessage::from("Could not add paper."))
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
    data: RouteHandlerData<'static>,
) -> HttpResponse {
    let (prof_id, paper_id) = params.into_inner();
    match data.write() {
        Ok(mut data) => {
            if let Some(professor) = data.0.get_mut(&prof_id) {
                if let Some(paper) = professor
                    .papers
                    .iter_mut()
                    .find(|p| p.id.clone() == paper_id)
                {
                    paper.title = form_body.title.to_owned();
                    let res = HttpResponse::Ok().json(paper);
                    if let Err(err) = data.save_state() {
                        eprintln!("Could not save data state: {err}");
                    }
                    res
                } else {
                    HttpResponse::NotFound().json(RouteResponseMessage::from("Paper not found."))
                }
            } else {
                HttpResponse::NotFound().json(RouteResponseMessage::from("Professor not found."))
            }
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError()
                .json(RouteResponseMessage::from("Could not edit paper."))
        }
    }
}

#[actix_web::delete("/api/professors/{prof_id}/papers/{paper_id}")]
pub async fn delete_paper(
    params: WebPath<(u32, u32)>,
    data: RouteHandlerData<'static>,
) -> HttpResponse {
    let (prof_id, paper_id) = params.into_inner();
    match data.write() {
        Ok(mut data) => {
            if let Some(professor) = data.0.get_mut(&prof_id) {
                let idx = {
                    let idx = professor
                        .papers
                        .iter()
                        .enumerate()
                        .find(|t| t.1.id == paper_id);
                    if idx.is_none() {
                        return HttpResponse::NotFound()
                            .json(RouteResponseMessage::from("Paper not found."));
                    }
                    idx.expect("Paper must exist").0
                };
                professor.papers.remove(idx);
                match data.save_state() {
                    Ok(_) => HttpResponse::NoContent().finish(),
                    Err(_) => HttpResponse::Accepted().json(RouteResponseMessage::from(
                        "Paper was deleted in memory, but the deletion has not been committed.",
                    )),
                }
            } else {
                HttpResponse::NotFound().json(RouteResponseMessage::from("Professor not found."))
            }
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError()
                .json(RouteResponseMessage::from("Could not delete paper."))
        }
    }
}

#[derive(Serialize)]
struct ProfessorDescription {
    description: String,
}

#[actix_web::get("/api/professors/{prof_id}/description")]
pub async fn get_description(
    params: WebPath<(u32,)>,
    data: RouteHandlerData<'static>,
) -> HttpResponse {
    let (prof_id,) = params.into_inner();
    match data.write() {
        Ok(mut data) => {
            if let Some(professor) = data.0.get_mut(&prof_id) {
                let description = match professor.generate_description().await {
                    Ok(desc) => desc,
                    Err(err) => {
                        eprintln!("Error generating description: {err}");
                        return HttpResponse::InternalServerError()
                            .json(RouteResponseMessage::from("An error occurred."));
                    }
                };
                match data.save_state() {
                    Ok(_) => HttpResponse::Ok().json(ProfessorDescription { description }),
                    Err(_) => HttpResponse::Accepted().json(RouteResponseMessage::from(
                        "Description was saved in memory but has not yet been committed.",
                    )),
                }
            } else {
                HttpResponse::NotFound().json(RouteResponseMessage::from("Professor not found."))
            }
        }
        Err(err) => {
            eprintln!("Error with RwLock (poisoned?): {err}");
            HttpResponse::InternalServerError().json(RouteResponseMessage::from(
                "Could not get professor description.",
            ))
        }
    }
}
