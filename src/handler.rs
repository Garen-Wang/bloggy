#![allow(dead_code)]
use std::fs;

use actix_web::{web, HttpResponse};

use crate::{state::AppState, generator::PUBLIC_DIR_NAME};

// GET
pub async fn health_check_handler(
    _app_state: web::Data<AppState>
) -> HttpResponse {
    HttpResponse::Ok().json("1")
}

// POST
pub async fn insert_new_course(
    _app_state: web::Data<AppState>
) -> HttpResponse {
    println!("Received new course");
    HttpResponse::Ok().json("Course added")
}

// GET
pub async fn get_course_for_teacher(
    _app_state: web::Data<AppState>,
    _params: web::Path<usize>
) -> HttpResponse {
    HttpResponse::Ok().json("Course not found for teacher".to_string())
}

// GET
pub async fn get_course_detail(
    _app_state: web::Data<AppState>,
    _params: web::Path<(usize, usize)>
) -> HttpResponse {
    HttpResponse::Ok().json("Course not found".to_string())
}

fn find_article_by_name(title: String) -> Option<String> {
    let public_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), PUBLIC_DIR_NAME);
    fs::read_to_string(format!("{}/{}.html", public_path, title)).ok()
}

pub async fn get_blog_article_by_name(
    _app_state: web::Data<AppState>,
    params: web::Path<String>
) -> HttpResponse {
    let title = params.0;
    let html_content = find_article_by_name(title)
        .unwrap_or(find_article_by_name("404".into()).unwrap());
    HttpResponse::Ok().body(html_content)
}

pub async fn get_blog_article_by_id(
    _app_state: web::Data<AppState>,
    _params: web::Path<usize>
) -> HttpResponse {
    HttpResponse::Ok().json("unimplemented")
}
