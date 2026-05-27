use std::env;
use std::fs;

use pulldown_cmark::{html, Parser};
use serde::Deserialize;
use tera::{Tera, Context};

struct ParsedFile {
    front_matter: String,
    body: String,
}

#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: Option<String>,
    date: Option<String>,
    tags: Option<String>,
    categories: Option<String>,
}

fn parse_front_matter(raw: &str) -> FrontMatter {
    serde_yaml::from_str(raw)
        .unwrap_or(FrontMatter {
            title: None, date: None, tags: None, categories: None })
}

fn split_front_matter(raw: &str) -> ParsedFile {
    let delim = "---\n";
    match raw.find(delim) {
        None => ParsedFile {
            front_matter: String::new(),
            body: raw.to_string(),
        },
        Some(first) => {
            let after_first = &raw[first + delim.len()..];
            match after_first.find(delim) {
                None => {
                    eprintln!("missing closing ---");
                    ParsedFile {
                        front_matter: String::new(),
                        body: raw.to_string(),
                    }
                }
                Some(second) => {
                    let fm_start = first + delim.len();
                    let fm_end = fm_start + second;
                    let body_start = fm_end + delim.len();

                    ParsedFile {
                        front_matter: raw[fm_start..fm_end].to_string(),
                        body: raw[body_start..].to_string(),
                    }
                }
            }
        }
    }
}

fn render_markdown(body: &str) -> String {
    let parser = Parser::new(body);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: {} <file.md>", args[0]);
        std::process::exit(1);
    }

    let raw = match fs::read_to_string(&args[1]) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("cannot open file {} - {}", args[1], e );
            std::process::exit(1);
        }
    };

    let parsed = split_front_matter(&raw);
    let fm: FrontMatter = if parsed.front_matter.is_empty() {
        FrontMatter { title: None, date: None, tags: None, categories: None }
    } else {
        parse_front_matter(&parsed.front_matter)
    };

    let content_html = render_markdown(&parsed.body);


    let mut tera = Tera::default();
    tera.add_raw_template("single.html", include_str!("../layouts/single.html"))
        .expect("failed to load template");

    let mut ctx = Context::new();
    ctx.insert("content", &content_html);
    ctx.insert("title", fm.title.as_deref().unwrap_or("untitled"));
    ctx.insert("date", fm.date.as_deref().unwrap_or(""));
    if let Some(ref title) = fm.title {
        ctx.insert("title", title);
    }
    if let Some(ref date) = fm.date {
        ctx.insert("date", date);
    }
    let html = tera.render("single.html", &ctx).expect("template render failed");
    println!("{}", html);
}
