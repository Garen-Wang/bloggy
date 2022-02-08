#![allow(dead_code)]
use std::{fs, path::Path, io};
use walkdir::WalkDir;

mod template;

pub const POSTS_DIR_NAME: &str = "posts";
pub const PUBLIC_DIR_NAME: &str = "public";
pub const STATIC_DIR_NAME: &str = "static";

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

fn remove_dir_contents(path: impl AsRef<Path>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type().unwrap();
        if file_type.is_file() {
            fs::remove_file(entry.path()).unwrap();
        } else if file_type.is_dir() {
            fs::remove_dir_all(entry.path()).unwrap();
        }
    }
    Ok(())
}

fn generate_static_images() {
    copy_dir_all("./static/images", "./public/images")
        .expect("error when copying static images");
}

fn clear_public_folder() {
    remove_dir_contents("./public")
        .expect("error when removing folder public");
}

fn generate_articles() {
    let markdown_files = WalkDir::new("./posts").into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().display().to_string().ends_with(".md"))
        .map(|entry| entry.path().display().to_string())
        .collect::<Vec<String>>();

    let options = {
        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        options
    };
    fs::create_dir_all("./public/articles")
        .expect("error when creating folder articles");
    let mut articles = vec![];
    for file in markdown_files { 
        let content = fs::read_to_string(&file)
            .expect(format!("error when reading {}", file).as_str());
        let parser = pulldown_cmark::Parser::new_ext(&content, options);
        let mut html_content = String::new();
        pulldown_cmark::html::push_html(&mut html_content, parser);
        let title: String = file.split("/").last().unwrap()
            .split(".").filter(|&s| s != "md").collect();
        let html_path = format!("./public/articles/{}.html", title);
        articles.push(html_path.clone());
        let final_html_content = format!("{}\n{}", template::render_html_head(&title), template::render_html_body(format!("{}{}", html_content, create_archive()).as_str()));
        fs::write(&html_path, final_html_content)
            .expect(format!("error when writing to {}", html_path).as_str());
    }

    fs::copy("./static/404.html", "./public/404.html").unwrap();
}

fn create_archive() -> String {
    let posts_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), POSTS_DIR_NAME);
    let public_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), PUBLIC_DIR_NAME);
    let markdown_files = WalkDir::new(&posts_path).into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().display().to_string().ends_with(".md"))
        .map(|entry| entry.path().display().to_string())
        .collect::<Vec<String>>();
    let articles = markdown_files.into_iter()
        .map(|name| format!("{}{}.html", public_path, name.trim_start_matches(&posts_path).trim_end_matches(".md")));
    let archive_links: Vec<String> = articles.into_iter()
        .map(|filename| {
            let link = filename.trim_start_matches(&public_path);
            let title = link.trim_start_matches("/").trim_end_matches(".html");
            format!(r#"<a href={}>{}</a>"#, link, title)
        }).collect();
    archive_links.join("<br /> \n")
}

pub fn generate() {
    clear_public_folder();
    generate_static_images();
    generate_articles();
}
