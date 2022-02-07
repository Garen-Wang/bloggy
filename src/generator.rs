use std::fs;
use walkdir::WalkDir;

mod template;

pub const POSTS_DIR_NAME: &str = "posts";
pub const PUBLIC_DIR_NAME: &str = "public";

pub fn generate_articles() {
    let public_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), PUBLIC_DIR_NAME);
    fs::remove_dir_all(&public_path)
        .and_then(|_| fs::create_dir(&public_path))
        .expect("error when removing folder 'public'");
    let posts_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), POSTS_DIR_NAME);
    let markdown_files = WalkDir::new(&posts_path).into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().display().to_string().ends_with(".md"))
        .map(|entry| entry.path().display().to_string())
        .collect::<Vec<String>>();

    let options = {
        let mut options = pulldown_cmark::Options::empty();
        options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
        options
    };
    let mut articles = vec![];
    for file in markdown_files { 
        let content = fs::read_to_string(&file)
            .expect(format!("error when reading {}", file).as_str());
        let parser = pulldown_cmark::Parser::new_ext(&content, options);
        let mut html_content = String::new();
        pulldown_cmark::html::push_html(&mut html_content, parser);
        let title: String = file.split("/").last().unwrap()
            .split(".").filter(|&s| s != "md").collect();
        let html_path = format!("{}/{}.html", public_path, title);
        articles.push(html_path.clone());
        let final_html_content = format!("{}\n{}", template::render_html_head(&title), template::render_html_body(format!("{}{}", html_content, create_archive()).as_str()));
        fs::write(&html_path, final_html_content)
            .expect(format!("error when writing to {}", html_path).as_str());
    }
    fs::copy(
        format!("{}/static/404.html", env!("CARGO_MANIFEST_DIR")),
        format!("{}/404.html", public_path)
    ).unwrap();
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
