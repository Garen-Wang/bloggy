use actix_web::web;
use crate::handler::{get_blog_article_by_name, get_static_image, get_main_homepage, get_blog_homepage, get_blog_archives, get_static_css, get_static_js};

pub fn general_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(get_main_homepage));
}

pub fn blog_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/blog")
        .route("/", web::get().to(get_blog_homepage))
        .route("/archives", web::get().to(get_blog_archives))
        .route("/css/{css_filename}", web::get().to(get_static_css))
        .route("/js/{js_filename}", web::get().to(get_static_js))
        .route("/image/{article_title}/{image_filename}", web::get().to(get_static_image))
        .route("/{article_title}", web::get().to(get_blog_article_by_name))
    );
}
