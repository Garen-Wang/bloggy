#![allow(dead_code)]
use std::{fs, path::Path, io::{self, BufRead}, borrow::Cow};
use regex::Regex;
use walkdir::{WalkDir, DirEntry};
use lazy_static::lazy_static;

use crate::template::{render_article_from_markdown, render_archives};

mod template;

/// for compatibility of the previous posts
/// {% qnimg path/to %} => <img src="/blog/image/path/to" />
fn replace_qiniu_to_link(text: &str) -> Cow<str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\{%\s*qnimg\s*(?P<path>.*?)\s*%\}"#).unwrap();
    }
    RE.replace_all(text, r#"<img src="/blog/image/$path" />"#)
}

/// copy the directory recursively
fn copy_dir_all(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dest).unwrap();
    for entry in fs::read_dir(&src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            // println!("copy_dir_all: {:#?}, {:#?}", entry.path(), dest.as_ref().join(entry.file_name()));
            copy_dir_all(entry.path(), dest.as_ref().join(entry.file_name()))?;
        } else {
            // println!("copy: {:#?}, {:#?}", entry.path(), dest.as_ref().join(entry.file_name()));
            fs::copy(entry.path(), dest.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// remove all files, but not removes all directories
fn remove_dir_contents(path: impl AsRef<Path>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            fs::remove_file(entry.path())?;
        } else if file_type.is_dir() {
            remove_dir_contents(entry.path())?;
            // fs::remove_dir_all(entry.path())?;
        }
    }
    Ok(())
}

/// copy static resources to `public`
/// TODO: still buggy
fn generate_static_resources() -> io::Result<()> {
    // fs::copy("./static/index_base.hbs", "./public/index.html").unwrap(); // TODO: bug
    fs::copy("./static/404.html", "./public/404.html")?; // TODO: bug
    fs::copy("./static/index.html", "./public/index.html")?; // TODO: bug

    // copy images, js, css respectively
    copy_dir_all("./static/images", "./public/images")?;

    fs::create_dir_all("./public/css")?;
    copy_dir_all("./static/css", "./public/css")?;

    copy_dir_all("./static/js", "./public/js")?;
    Ok(())
}

/// must be used initially
fn clear_public_folder() -> io::Result<()> {
    remove_dir_contents("./public")
}

fn read_article_from_file(dir_entry: &DirEntry) -> io::Result<(ArticleConfig, String)> {
    let mut article_config = ArticleConfig::new();
    let read_content = fs::read(dir_entry.path()).unwrap();
    let real_content = match read_content.lines().filter_map(|s| s.ok()).take(1).next().unwrap().trim() {
        "---" => {
            article_config = ArticleConfig::from(
                read_content.lines().filter_map(|s| s.ok()).take(6)
            );
            read_content.lines().
                filter_map(|s| s.ok())
                .skip(6).collect::<Vec<String>>()
                .join("\n")
        },
        _ => String::from_utf8(read_content).unwrap()
    };
    // modify markdown file here
    let real_content = replace_qiniu_to_link(&real_content);
    Ok((article_config, real_content.to_string()))
}

fn generate_articles() -> io::Result<()> {
    let markdown_files = WalkDir::new("./posts").into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .collect::<Vec<DirEntry>>();

    fs::create_dir_all("./public/articles")?;
    for dir_entry in markdown_files { 
        let (article_config, real_content) = read_article_from_file(&dir_entry)?;
        let final_html_content = render_article_from_markdown(article_config, &real_content);
        // let html_content = render_html_from_markdown(&real_content);
        let html_path = format!("./public/articles/{}.html", dir_entry.file_name().to_str().unwrap().trim_end_matches(".md"));
        // let final_html_content = render_final_html_content(&article_config.title, &html_content, true);
        // let final_html_content = format!(
            // "{}\n{}", 
            // template::render_html_head(&article_config.title),
            // template::render_html_body(&html_content)
        // );
        fs::write(&html_path, final_html_content)?;
    }
    Ok(())
}

fn get_config_from_article(filename: &str) -> ArticleConfig {
    let path = format!("./posts/{}", filename);
    let read_content = fs::read(path).unwrap();
    if read_content.lines().filter_map(|s| s.ok()).take(1).next().unwrap().trim() == "---" {
        ArticleConfig::from(
            read_content.lines().filter_map(|s| s.ok()).take(6)
        )
    } else {
        ArticleConfig::new()
    }
}

fn generate_archives_info() -> Vec<(String, ArticleConfig)> {
    let markdown_files: Vec<String> = WalkDir::new("./posts").into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().display().to_string().ends_with(".md"))
        .filter_map(|entry| entry.file_name().to_os_string().into_string().ok())
        .collect();
    markdown_files.into_iter()
        .map(|filename| (
            filename.trim_end_matches(".md").to_owned(),
            get_config_from_article(&filename)
        ))
        .collect()
    // let articles = markdown_files.into_iter()
    //     .map(|name| format!("/blog/{}", name.trim_end_matches(".md")));
    // let archive_links: Vec<String> = articles.into_iter()
    //     .map(|link| {
    //         let title = link.trim_start_matches("/blog/");
    //         format!(r#"<a href={}>{}</a>"#, link, title)
    //     }).collect();
    // archive_links.join("<br /> \n")
}

fn generate_archives() -> io::Result<()> {
    let html_path = "./public/archives.html";
    let final_html_content = render_archives("Garen Wang's Archives", generate_archives_info());
    fs::write(html_path, final_html_content)?;
    Ok(())
}

pub fn generate() -> io::Result<()> {
    clear_public_folder()?;
    generate_static_resources()?;
    generate_archives()?;
    generate_articles()?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArticleConfig {
    pub title: String,
    pub mathjax: bool,
    pub date: String,
    pub tags: Vec<String>,
}

impl ArticleConfig {
    pub fn new() -> Self {
        ArticleConfig {
            title: "".into(),
            mathjax: false,
            date: chrono::Utc::now().to_string(),
            tags: vec![],
        }
    }
}

impl<T> From<T> for ArticleConfig where T: Iterator<Item = String> {
    fn from(iterator: T) -> Self {
        let mut title: String = "".into();
        let mut mathjax: bool = false;
        let mut date: String = "".into();
        let mut tags: Vec<String> = vec![];

        for line in iterator {
            if line.starts_with("title:") {
                title = line.trim_start_matches("title:").trim().to_string();
            } else if line.starts_with("mathjax:") {
                let a = line.trim_start_matches("mathjax:").trim();
                mathjax = match a {
                    "true" => true,
                    "false" => false,
                    _ => false,
                };
            } else if line.starts_with("date:") {
                let a = line.trim_start_matches("date:").trim();
                date = a.to_string();
            } else if line.starts_with("tags:") {
                let a = line.trim_start_matches("tags:").trim();
                let v: Vec<String> = a.split(",").map(|x| x.trim().to_string()).collect();
                tags = v;
            }
        }
        ArticleConfig { title, mathjax, date, tags }
    }
}


#[cfg(test)]
mod tests {
    use std::io::BufRead;

    use regex::Regex;

    use super::*;

    #[test]
    fn test_article_config_from_trait() {
        // title: test
        // mathjax: true
        // date: 2022-01-17 11:45:32
        // tags: CSAPP
        let read_content = fs::read("./posts/test.md").unwrap();
        let header_content = read_content.lines().filter_map(|x| x.ok()).take(6);
        let article_config = ArticleConfig::from(header_content);
        assert_eq!(article_config, ArticleConfig {
            title: "test".into(),
            mathjax: true,
            date: "2022-01-17 11:45:32".into(),
            tags: vec!["CSAPP".to_string()],
        });
    }

    #[test]
    fn test_read_first_few_lines() {
        let mut article_config = ArticleConfig::new();
        let read_content = fs::read("./posts/test.md").unwrap();
        let real_content = match read_content.lines().filter_map(|s| s.ok()).take(1).next().unwrap().trim() {
            "---" => {
                article_config = ArticleConfig::from(
                    read_content.lines().filter_map(|s| s.ok()).take(6)
                );
                read_content.lines().
                    filter_map(|s| s.ok())
                    .skip(6).collect::<Vec<String>>()
                    .join("\n")
            },
            _ => String::from_utf8(read_content).unwrap()
        };
        // fs::write("./tests/test.md", real_content).unwrap();
        let title = "test";
        let html_path = format!("./public/articles/{}.html", title);
        let final_html_content = render_article_from_markdown(article_config, &real_content);
        fs::write(&html_path, final_html_content)
            .expect(format!("error when writing to {}", html_path).as_str());
    }

    #[test]
    fn test_regex_convert_qnimg_to_link() {
        // {% qnimg CSAPP-Attack-Lab-Writeup/success1.png %}
        let re = Regex::new(r#"\{%\s*qnimg\s*(.*?)\s*%\}"#).unwrap();
        let text = r#"{% qnimg CSAPP-Attack-Lab-Writeup/success1.png %}"#;
        for cap in re.captures_iter(text) {
            assert_eq!("{% qnimg CSAPP-Attack-Lab-Writeup/success1.png %}", &cap[0]);
            assert_eq!("CSAPP-Attack-Lab-Writeup/success1.png", &cap[1]);
        }
    }

    #[test]
    fn test_pulldown_cmark_toc() {
        let entry = walkdir::WalkDir::new("./posts/CSAPP-Bomb-Lab-Writeup.md").into_iter().next().unwrap().unwrap();
        let (_article_config, real_content) = read_article_from_file(&entry).unwrap();
        let toc = pulldown_cmark_toc::TableOfContents::new(&real_content);
        let res = toc.to_cmark();
        println!("{}", res);
    }
}
