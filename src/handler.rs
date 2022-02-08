use std::fs;

use actix_web::{web, HttpResponse};

use crate::state::AppState;

// GET handler
pub async fn get_static_image(
    _app_state: web::Data<AppState>,
    params: web::Path<(String, String)>
) -> HttpResponse {
    println!("[GET] get_static_image: {:?}", params);
    let article_title = &params.0;
    let image_filename = &params.1;
    let image_content = fs::read(format!("./public/images/{}/{}", article_title, image_filename));
    if let Ok(content) = image_content {
        let bytes = web::Bytes::from(content);
        HttpResponse::Ok().content_type("image/jpeg").body(bytes)
    } else {
        HttpResponse::Ok().body(get_404_error_html_content())
    }
}

fn find_article_by_name(title: &str) -> Option<String> {
    fs::read_to_string(format!("./public/articles/{}.html", title)).ok()
}

fn get_404_error_html_content() -> String {
    fs::read_to_string("./public/404.html").unwrap()
}

// GET handler
pub async fn get_blog_article_by_name(
    _app_state: web::Data<AppState>,
    params: web::Path<(String,)>
) -> HttpResponse {
    println!("[GET] get_blog_article_by_name: {:?}", params);
    let title = &params.0;
    let html_content = find_article_by_name(title)
        .unwrap_or(get_404_error_html_content());
    HttpResponse::Ok().body(html_content)
}

// TODO: GET handler
pub async fn get_blog_article_by_id(
    _app_state: web::Data<AppState>,
    params: web::Path<(usize,)>
) -> HttpResponse {
    println!("[GET] get_blog_article_by_id: {:?}", params);
    HttpResponse::Ok().json("unimplemented")
}

// GET handler
pub async fn get_homepage(
    _app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("[GET] get_homepage");
    HttpResponse::Ok().body(
        fs::read_to_string("./public/index.html").unwrap()
    )
}
