use std::{fs, path::Path, io, borrow::Cow};
use chrono::prelude::*;
use regex::Regex;
use walkdir::{WalkDir, DirEntry};
use lazy_static::lazy_static;

use crate::template::{render_article_from_markdown, render_archives, render_homepage_html_content, IndexItem};

mod template;

/// for compatibility of the previous posts
/// {% qnimg path/to %} => <img src="/static/img/path/to" />
fn replace_qiniu_to_link(text: &str) -> Cow<str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\{%\s*qnimg\s*(?P<path>.*?)\s*%\}"#).unwrap();
    }
    RE.replace_all(text, r#"<img src="/static/img/$path" />"#)
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
    fs::copy("./static/favicon.ico", "./public/favicon.ico")?;
    
    // generate 404 webpage from markdown file
    let entry = WalkDir::new("./posts/special/404.md").into_iter().take(1).next().unwrap().unwrap();
    let (article_config, real_content) = read_article_from_file(&entry)?;
    let a = render_article_from_markdown(article_config, &real_content);
    fs::write("./public/404.html", a)?;

    // render index webpage dynamically
    fs::write("./public/index.html", render_homepage_html_content())?;

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

/// return the index that splits the config at first few lines
/// if the content is invalid, return Err (but error content does not matter)
fn split_config_from_markdown(read_content: &str) -> io::Result<usize> {
    let lines = read_content.lines();
    let mut cnt = 0;
    for (idx, line) in lines.enumerate() {
        if line.trim() == "---" {
            cnt += 1;
            if cnt == 2 {
                return Ok(idx + 1);
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::Other, "invalid markdown"))
}

/// read the article by passing a *walkdir::DirEntry*
/// return the parsed result of article config and the main body
fn read_article_from_file(dir_entry: &DirEntry) -> io::Result<(ArticleConfig, String)> {
    let read_content = fs::read_to_string(dir_entry.path())?;
    let n = split_config_from_markdown(&read_content)?;
    let real_content = read_content.lines().skip(n).collect::<Vec<&str>>().join("\n");
    let article_config = ArticleConfig::from(
        read_content.lines().take(n)
    );
    // some replacement in order to make previous posts compatible 
    let real_content = replace_qiniu_to_link(&real_content);
    Ok((article_config, real_content.to_string()))
}

/// generate all articles in blog and save to `public` folder
fn generate_articles() -> io::Result<()> {
    let markdown_files = WalkDir::new("./posts").into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .collect::<Vec<DirEntry>>();

    fs::create_dir_all("./public/articles")?;
    for dir_entry in markdown_files { 
        let (article_config, real_content) = read_article_from_file(&dir_entry)?;
        if !article_config.visible {
            continue;
        }
        let final_html_content = render_article_from_markdown(article_config, &real_content);
        // let html_content = render_html_from_markdown(&real_content);
        let html_path = format!("./public/articles/{}.html", dir_entry.file_name().to_str().unwrap().trim_end_matches(".md"));
        fs::write(&html_path, final_html_content)?;
    }
    Ok(())
}

// TODO: remove all punctuations, generate readable headings
fn generate_heading_of_index_item(real_content: &str) -> String {
    real_content.chars().take(100).collect()
}

pub fn generate_index_item_info() -> io::Result<Vec<IndexItem>> {
    // filename, title, heading
    let dir_entries : Vec<DirEntry> = WalkDir::new("./posts").into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().display().to_string().ends_with(".md"))
        .collect();
    let mut ans = vec![];
    for dir_entry in dir_entries {
        let (article_config, real_content) = read_article_from_file(&dir_entry)?;
        if !article_config.visible {
            continue;
        }
        let filename = dir_entry.file_name().to_str().unwrap().trim_end_matches(".md");
        let heading = generate_heading_of_index_item(&real_content);
        ans.push(IndexItem::new(filename.to_string(), article_config.title, heading));
    }
    Ok(ans)
}

fn generate_archives_info() -> io::Result<Vec<(String, ArticleConfig)>> {
    let dir_entries : Vec<DirEntry> = WalkDir::new("./posts").into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().display().to_string().ends_with(".md"))
        .collect();
    let mut ans = vec![];
    for dir_entry in dir_entries {
        let (article_config, _) = read_article_from_file(&dir_entry)?;
        if !article_config.visible {
            continue;
        }
        let filename = dir_entry.file_name().to_str().unwrap().trim_end_matches(".md");
        ans.push((filename.to_string(), article_config));
    }
    Ok(ans)
}

fn generate_archives() -> io::Result<()> {
    let html_path = "./public/archives.html";
    let final_html_content = render_archives("Garen Wang's Archives", generate_archives_info()?);
    fs::write(html_path, final_html_content)?;
    Ok(())
}

/// main function of generator
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
    pub date: DateTime<Utc>,
    pub tags: Vec<String>,
    pub visible: bool,
}

impl ArticleConfig {
    pub fn new() -> Self {
        ArticleConfig {
            title: "".into(),
            mathjax: false,
            date: chrono::Utc::now(),
            tags: vec![],
            visible: true,
        }
    }
}

impl Default for ArticleConfig {
    fn default() -> Self {
        ArticleConfig::new()
    }
}

impl<'a, T> From<T> for ArticleConfig where T: Iterator<Item = &'a str> {
    fn from(iterator: T) -> Self {
        let mut title: String = "".into();
        let mut mathjax: bool = false;
        let mut date: DateTime<Utc> = chrono::Utc::now();
        let mut tags: Vec<String> = vec![];
        let mut visible: bool = true;

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
                date = Utc.datetime_from_str(a, "%Y-%m-%d %H:%M:%S").unwrap();
            } else if line.starts_with("tags:") {
                let a = line.trim_start_matches("tags:").trim();
                let v: Vec<String> = a.split(",").map(|x| x.trim().to_string()).collect();
                tags = v;
            } else if line.starts_with("visible:") {
                let a = line.trim_start_matches("visible:").trim();
                visible = match a {
                    "true" => true,
                    "false" => false,
                    _ => false
                };
            }
        }
        ArticleConfig { title, mathjax, date, tags, visible }
    }
}


#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn test_article_config_from_trait() {
        // title: test
        // mathjax: true
        // date: 2022-01-17 11:45:32
        // tags: CSAPP
        let read_content = fs::read_to_string("./posts/test.md").unwrap();
        let header_content = read_content.lines().take(6);
        let article_config = ArticleConfig::from(header_content);
        assert_eq!(article_config, ArticleConfig {
            title: "test".into(),
            mathjax: true,
            date: Utc.datetime_from_str("2022-01-17 11:45:32", "%Y-%m-%d %H:%M:%S").unwrap(),
            tags: vec!["CSAPP".to_string()],
            visible: true,
        });
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

    #[test]
    fn test_read_config_from_markdown() {
        let read_content = fs::read_to_string("./posts/test.md").unwrap();
        let lines = read_content.lines();
        let mut cnt = 0;
        for (idx, line) in lines.enumerate() {
            if line.trim() == "---" {
                cnt += 1;
                if cnt == 2 {
                    assert_eq!(idx, 5);
                } else if cnt == 1 {
                    assert_eq!(idx, 0);
                }
            }
        }
    }
}
