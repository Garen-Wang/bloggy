use std::fs;

use actix_web::{web, HttpResponse};

use crate::state::AppState;

pub async fn get_static_image(
    _app_state: web::Data<AppState>,
    params: web::Path<(String, String)>
) -> HttpResponse {
    println!("getting static image: {:?}", params);
    let (article_title , image_filename) = params.0;
    let image_content = fs::read(format!("./public/images/{}/{}", article_title, image_filename));
    if let Ok(content) = image_content {
        let bytes = web::Bytes::from(content);
        HttpResponse::Ok().content_type("image/jpeg").body(bytes)
    } else {
        let a = fs::read_to_string("./public/404.html").unwrap();
        HttpResponse::Ok().body(a)
    }
}

fn find_article_by_name(title: String) -> Option<String> {
    fs::read_to_string(format!("./public/articles/{}.html", title)).ok()
}

pub async fn get_blog_article_by_name(
    _app_state: web::Data<AppState>,
    params: web::Path<String>
) -> HttpResponse {
    let title = params.0;
    let html_content = find_article_by_name(title)
        .unwrap_or(fs::read_to_string("./public/404.html").unwrap());
    HttpResponse::Ok().body(html_content)
}

pub async fn get_blog_article_by_id(
    _app_state: web::Data<AppState>,
    _params: web::Path<usize>
) -> HttpResponse {
    HttpResponse::Ok().json("unimplemented")
}
