#![allow(dead_code)]
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::json;

use crate::generator::{ArticleConfig, generate_index_item_info};

fn render_html_from_markdown(real_content: &str) -> String {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    let parser = pulldown_cmark::Parser::new_ext(real_content, options);
    let mut html_content = String::new();
    pulldown_cmark::html::push_html(&mut html_content, parser);
    html_content
}

fn render_toc_html_from_markdown(real_content: &str) -> String {
    let toc = pulldown_cmark_toc::TableOfContents::new(&real_content);
    // let levels = 1..=6;
    // for heading in toc.headings().filter(|h| levels.contains(h.level())) {
    //     let anchor = heading.anchor();
    // }
    let res = toc.to_cmark();
    let html_content = render_html_from_markdown(&res);
    html_content.replace("ul", "ol")
    // println!("{}", res);
}

pub fn render_article_from_markdown(article_config: ArticleConfig, real_content: &str) -> String {
    render_final_article_html(
        &article_config.title,
        &render_html_from_markdown(real_content),
        true,
        &render_toc_html_from_markdown(real_content),
        &article_config.date,
    )
}

pub fn render_archives(title: &str, filenames: Vec<(String, ArticleConfig)>) -> String {
    let archive_links: Vec<String> = filenames.into_iter()
        .map(|(filename, config)| {
            format!(r#"<a href=/blog/{}>{}</a>"#, filename, config.title)
        }).collect();
    render_final_article_html(
        title,
        &archive_links.join("<br> \n"),
        false,
        "",
        "",
    )
}

fn render_final_article_html(
    title: &str,
    main_html_content: &str,
    comments: bool,
    toc: &str,
    datetime: &str,
    // _pathname: &str,
) -> String {
    // format!(
        // "{}\n{}",
        // render_html_head(title),
        // render_html_body(title, article_content, comments)
    // )
    let reg = handlebars::Handlebars::new();
    let params = json!({
        "title": title,
        "main_html_content": main_html_content,
        "comments": comments,
        "toc": toc,
        "datetime": datetime,
    });
    reg.render_template(&fs::read_to_string("./static/article_base.hbs").unwrap(), &params).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndexItem {
    filename: String,
    title: String,
    heading: String,
}

impl IndexItem {
    pub fn new(filename: String, title: String, heading: String) -> Self { IndexItem { filename, title, heading }
    }
}

pub fn render_homepage_html_content() -> String {
    let reg = handlebars::Handlebars::new();
    
    // let articles = vec![
        // IndexItem {
            // filename: "filename".into(),
            // title: "title".into(),
            // heading: "headingheadingheadingheadingheadingheadingheadingheadingheadingheadingheading".into()
        // },
    // ];
    let articles = generate_index_item_info().unwrap();
    let params = json!({
        "articles": articles
    });
    reg.render_template(&fs::read_to_string("./static/index_base.hbs").unwrap(), &params).unwrap()
    // fs::read_to_string("./static/index_base.hbs").unwrap()
}

