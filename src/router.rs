use actix_web::web;
use crate::handler::{get_blog_article_by_name, get_blog_article_by_id, get_static_image, get_homepage};

pub fn general_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(get_homepage));
}

pub fn blog_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/blog")
        .route("/static/{article_title}/{image_filename}", web::get().to(get_static_image))
        .route("/{article_title}", web::get().to(get_blog_article_by_name))
        .route("/id/{article_id}", web::get().to(get_blog_article_by_id))
    );
}
