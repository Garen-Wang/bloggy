#![allow(dead_code)]
use actix_web::web;

use crate::handler::{health_check_handler, get_blog_article_by_name, get_blog_article_by_id};

pub fn general_routes(config: &mut web::ServiceConfig) {
    config.route("/health", web::get().to(health_check_handler));
}

pub fn blog_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/blog")
            .route("/{article_title}", web::get().to(get_blog_article_by_name))
            .route("/id/{article_id}", web::get().to(get_blog_article_by_id))
    );
}
