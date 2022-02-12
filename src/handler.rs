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

// GET handler
pub async fn get_main_homepage(
    _app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("[GET] get_main_homepage");
    HttpResponse::Ok().body(
        fs::read_to_string("./public/index.html").unwrap()
    )
}


pub async fn get_blog_homepage(
    _app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("[GET] get_blog_homepage");
    HttpResponse::Ok().body(
        fs::read_to_string("./public/index.html").unwrap()
    )
}

pub async fn get_blog_archives(
    _app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("[GET] get_blog_archive");
    HttpResponse::Ok().body(
        fs::read_to_string("./public/archives.html").unwrap()
    )
}

pub async fn get_static_css(
    _app_state: web::Data<AppState>,
    params: web::Path<(String, )>
) -> HttpResponse {
    println!("[GET] get_static_css");
    HttpResponse::Ok()
        .content_type("text/css")
        .body(
        fs::read_to_string(format!("./public/css/{}", params.0)).unwrap()
    )
}

pub async fn get_static_js(
    _app_state: web::Data<AppState>,
    params: web::Path<(String, )>
) -> HttpResponse {
    println!("[GET] get_static_js");
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(
        fs::read_to_string(format!("./public/js/{}", params.0)).unwrap()
    )
}

pub async fn get_favicon(
    _app_state: web::Data<AppState>
) -> HttpResponse {
    println!("[GET] get_favicon");
    let image_content = fs::read("./public/favicon.ico");
    if let Ok(content) = image_content {
        let bytes = web::Bytes::from(content);
        HttpResponse::Ok().content_type("image/x-icon").body(bytes)
    } else {
        HttpResponse::Ok().body(get_404_error_html_content())
    }
}
