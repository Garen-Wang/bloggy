#![allow(dead_code)]
pub fn render_html_head(title: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
</head>"#,
        title = title
    )
}

pub fn render_html_body(body: &str) -> String {
    format!(
        r#"<body>
    {}
</body>
</html>"#,
        body
    )
}
