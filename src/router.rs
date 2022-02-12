use actix_web::web;

use crate::handler::{general_handlers::{get_main_homepage, get_favicon}, blog_handlers::{get_blog_homepage, get_blog_archives, get_blog_about_page, get_blog_article_by_name}, static_handlers::{get_static_image, get_static_css, get_static_js}};

pub fn general_routes(config: &mut web::ServiceConfig) {
    config
    .route("/", web::get().to(get_main_homepage))
    .route("/favicon.ico", web::get().to(get_favicon));
}

pub fn blog_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/blog")
        .route("/", web::get().to(get_blog_homepage))
        .route("/archives", web::get().to(get_blog_archives))
        .route("/about", web::get().to(get_blog_about_page))
        .route("/{article_title}", web::get().to(get_blog_article_by_name))
    );
}

pub fn static_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/static")
        .route("/img/{article_title}/{image_filename}", web::get().to(get_static_image))
        .route("/css/{css_filename}", web::get().to(get_static_css))
        .route("/js/{js_filename}", web::get().to(get_static_js))
    );
}