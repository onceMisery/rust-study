use pulldown_cmark::{html, Options, Parser};

/// 把 Markdown 渲染成 HTML。
///
/// 这里启用了表格、脚注、删除线等常见扩展。
/// 真实公网博客还应考虑 HTML 清洗，避免 XSS。
pub fn render_markdown(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(markdown, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}
