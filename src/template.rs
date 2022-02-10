#![allow(dead_code)]
use std::fs;
use serde_json::json;

use crate::generator::ArticleConfig;

fn render_utterances_comments(pathname: &str) -> String {
    format!(r#"<script src="https://utteranc.es/client.js"
    repo="garen-wang/garen-wang.github.io"
    issue-term="{}"
    theme="github-light"
    crossorigin="anonymous"
    async>
</script>"#, pathname)
}

fn render_html_head(title: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="https://cdnjs.cloudflare.com/ajax/libs/bulma/0.3.1/css/bulma.min.css" rel="stylesheet">
    <title>{title}</title>
</head>"#,
        title = title
    )
}

fn render_html_body(
    title: &str, body: &str, comments: bool,
) -> String {
    format!(
        r#"<body>
    <section class="hero is-primary is-fullheight">
        <div class="hero-head">
            <nav class="navbar">
                <div class="container">
                    <div id="navbarMenuHeroA" class="navbar-menu">
                        <div class="navbar-end">
                        </div>
                    </div>
                </div>
            </nav>
        </div>

        <div class="hero-body">
            <div class="container has-text-centered">
            </div>
        </div>
        <div class="container">
            <p>test</p>
        </div>
        <div class="container">
            <p>test</p>
        </div>
    </section>
    {}
    {}
</body>
</html>"#,
        body,
        if comments { render_utterances_comments(title) } else { "".into() },
    )
}

pub fn render_html_from_markdown(real_content: &str) -> String {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    let parser = pulldown_cmark::Parser::new_ext(real_content, options);
    let mut html_content = String::new();
    pulldown_cmark::html::push_html(&mut html_content, parser);
    html_content
}

pub fn render_article_from_markdown(article_config: ArticleConfig, real_content: &str) -> String {
    render_final_html_content(
        &article_config.title,
        &render_html_from_markdown(real_content),
        true,
    )
}

pub fn render_archives(title: &str, filenames: Vec<(String, ArticleConfig)>) -> String {
    let archive_links: Vec<String> = filenames.into_iter()
        .map(|(filename, config)| {
            format!(r#"<a href=/blog/{}>{}</a>"#, filename, config.title)
        }).collect();
    render_final_html_content(
        title,
        &archive_links.join("<br> \n"),
        false
    )
}

pub fn render_final_html_content(
    title: &str,
    main_html_content: &str,
    comments: bool,
    // _date: &str,
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
    });
    reg.render_template(&fs::read_to_string("./static/article_base.hbs").unwrap(), &params).unwrap()
}

pub fn render_homepage_html_content() -> String {
    // let reg = handlebars::Handlebars::new();
    fs::read_to_string("./static/index_base.hbs").unwrap()
}

#[cfg(test)]
mod tests {
    // use super::*;

}