use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;
use std::process::Command;

use chrono::{Local, NaiveDate};
use clap::{App, ArgMatches, SubCommand};
use liquid;
use pulldown_cmark::{html, Options as MDOptions, Parser as MDParser};
use warp;


#[derive(Debug)]
struct Metadata {
    title: String,
    slug: String,
    created: NaiveDate,
    updated: NaiveDate,
    tags: Vec<String>,
    summary: String,
}
impl Metadata {
    const NUM_HEADER_LNS: u8 = 6;
    const TAG_DELIMITER: &'static str = ",";
    const DATE_FMT: &'static str = "%Y-%m-%d";

    fn new<S: AsRef<str>>(header_text: S) -> Self {
        let headers = Self::header_map(&header_text);
        let get_value = |v: &str| Self::header_value(&headers, v);

        Metadata {
            title: get_value(&"title").into(),
            slug: get_value(&"slug").into(),
            created: Self::date(get_value(&"created")),
            updated: Self::date(get_value(&"updated")),
            tags: Self::tags(get_value(&"tags")),
            summary: get_value(&"summary").into(),
        }
    }

    fn header_map<'a, S: AsRef<str>>(header_text: &'a S) -> HashMap<&'a str, &'a str> {
        let lines = header_text
            .as_ref()
            .lines()
            .take(Self::NUM_HEADER_LNS.into());
        let ln_tuples = lines.map(|ln| {
            let mut parts = ln.splitn(2, ":").map(|i| i.trim());
            (
                parts.next().expect(&format!("bad header: {:?}", ln)),
                parts.next().expect(&format!("bad header: {:?}", ln)),
            )
        });
        ln_tuples.collect()
    }

    fn header_value<'a>(headers: &'a HashMap<&str, &str>, key: &str) -> &'a str {
        headers.get(key).expect(&format!("No {:?} header", key))
    }

    fn tags<S: AsRef<str>>(tags: S) -> Vec<String> {
        tags.as_ref()
            .split(Self::TAG_DELIMITER)
            .map(|s| s.trim())
            .map(|s| s.to_owned())
            .collect()
    }

    fn date<S: AsRef<str>>(date: S) -> NaiveDate {
        NaiveDate::parse_from_str(date.as_ref(), Self::DATE_FMT)
            .expect(&format!("invalid date: {:?}", date.as_ref()))
    }
}

#[derive(Debug)]
struct Post {
    metadata: Metadata,
    content: String,
}


fn md_to_html(md: &str, opts: MDOptions) -> String {
    let mut html = String::new();
    html::push_html(&mut html, MDParser::new_ext(&md, opts));
    html
}

fn get_md_opts() -> MDOptions {
    let mut options = MDOptions::empty();
    options.insert(MDOptions::ENABLE_FOOTNOTES);
    options.insert(MDOptions::ENABLE_TABLES);
    options.insert(MDOptions::ENABLE_STRIKETHROUGH);
    options
}

fn to_liquid_val<S: AsRef<str>>(string: S) -> liquid::value::Value {
    liquid::value::Value::scalar(string.as_ref().to_owned())
}

fn generate_footer_nav_block(
    parser: &liquid::Parser,
    prev_slug: Option<String>,
    next_slug: Option<String>,
) -> String {
    let footer_nav_tpl = include_str!("../templates/snippets/footer-nav.html");
    let footer_nav_first_tpl = include_str!("../templates/snippets/footer-nav-first.html");
    let footer_nav_last_tpl = include_str!("../templates/snippets/footer-nav-last.html");

    let mut globals = liquid::value::Object::new();
    let template: &'static str;

    if let Some(prev) = prev_slug {
        globals.insert("previous".into(), to_liquid_val(prev));

        if let Some(next) = next_slug {
            globals.insert("next".into(), to_liquid_val(next));
            template = footer_nav_tpl;
        } else {
            template = footer_nav_last_tpl;
        }
    } else {
        if let Some(next) = next_slug {
            globals.insert("next".into(), to_liquid_val(next));
            template = footer_nav_first_tpl
        } else {
            template = ""
        }
    };

    parser
        .parse(&template)
        .expect("failed to parse footer nav template")
        .render(&globals)
        .expect("failed to render footer nav template")
}

fn generate_footer_license_block(parser: &liquid::Parser) -> String {
    let footer_license_tpl = include_str!("../templates/snippets/footer-license.html");
    let today = format!("{}", Local::today().format("%Y-%m-%d"));
    let globals = liquid::value::Object::from_iter(vec![("year".into(), to_liquid_val(today))]);

    parser
        .parse(&footer_license_tpl)
        .expect("failed to parse footer license template")
        .render(&globals)
        .expect("failed to render footer license template")
}

fn generate_head_block(head_template: &liquid::Template, title: String) -> String {
    let globals = liquid::value::Object::from_iter(vec![(
        "title".into(),
        liquid::value::Value::scalar(title),
    )]);
    head_template
        .render(&globals)
        .expect("failed to render head template")
}

fn generate_generic(
    parser: &liquid::Parser,
    head_template: &liquid::Template,
    header: &str,
    footer_license: &str,
    title: &str,
    content: &str,
) -> String {
    let generic_tpl = include_str!("../templates/generic.html");
    let head = generate_head_block(&head_template, String::from(title));

    let globals = liquid::value::Object::from_iter(vec![
        ("head".into(), to_liquid_val(head)),
        ("header".into(), to_liquid_val(header)),
        ("content".into(), to_liquid_val(content)),
        ("footer-license".into(), to_liquid_val(footer_license)),
    ]);

    parser
        .parse(&generic_tpl)
        .expect("failed to parse about")
        .render(&globals)
        .expect("failed to render head template")

}

fn generate_about(
    parser: &liquid::Parser,
    head_template: &liquid::Template,
    header: &str,
    footer_license: &str,
) {
    let about_tpl = include_str!("../templates/about.html");
    let about_content = include_str!("../templates/static_blocks/about-content.html");
    let head = generate_head_block(&head_template, String::from("About"));

    let globals = liquid::value::Object::from_iter(vec![
        ("head".into(), to_liquid_val(head)),
        ("header".into(), to_liquid_val(header)),
        ("content".into(), to_liquid_val(about_content)),
        ("footer-license".into(), to_liquid_val(footer_license)),
    ]);

    let about = parser
        .parse(&about_tpl)
        .expect("failed to parse about")
        .render(&globals)
        .expect("failed to render head template");

    fs::write("static/about.html", about).expect("couldn't write about.html");
}

fn generate_index_and_posts<'a, P: IntoIterator<Item = &'a Post>>(
    parser: &liquid::Parser,
    head_template: &liquid::Template,
    header: &str,
    footer_license: &str,
    posts: P,
) {
    let posts_tpl = include_str!("../templates/generic.html");
    let index_tpl = include_str!("../templates/index.html");
    let posts_post_tpl = include_str!("../templates/snippets/posts-post.html");
    let posts_content_tpl = include_str!("../templates/snippets/posts-content.html");
    let head = generate_head_block(&head_template, String::from("Home"));

    let index_template = parser
        .parse(&index_tpl)
        .expect("Couldn't parse index template");
    let posts_template = parser
        .parse(&posts_tpl)
        .expect("Couldn't parse posts template");
    let posts_post_tpl = parser
        .parse(&posts_post_tpl)
        .expect("couldn't parse posts-post template");
    let posts_content_tpl = parser
        .parse(&posts_content_tpl)
        .expect("couldn't parse posts-content template");

    let posts_items: String = posts
        .into_iter()
        .map(|p| {
            let globals = liquid::value::Object::from_iter(vec![
                ("slug".into(), to_liquid_val(&p.metadata.slug)),
                ("title".into(), to_liquid_val(&p.metadata.title)),
                ("summary".into(), to_liquid_val(&p.metadata.summary)),
            ]);
            posts_post_tpl
                .render(&globals)
                .expect(&format!("couldn't render post: {:?}", p))
        })
        .collect::<Vec<String>>()
        .join("\n");

    let posts_content_globals =
        liquid::value::Object::from_iter(vec![("posts".into(), to_liquid_val(posts_items))]);
    let posts_content = posts_content_tpl
        .render(&posts_content_globals)
        .expect("couldn't render posts content");

    let posts_globals = liquid::value::Object::from_iter(vec![
        ("head".into(), to_liquid_val(head)),
        ("header".into(), to_liquid_val(header)),
        ("content".into(), to_liquid_val(posts_content)),
        ("footer-license".into(), to_liquid_val(footer_license)),
    ]);
    let posts_html = posts_template
        .render(&posts_globals)
        .expect("couldn't render posts");
    let index_html = index_template
        .render(&posts_globals)
        .expect("couldn't render index");

    fs::write("static/index.html", &index_html).expect("failed to write index file");
    fs::write("static/posts.html", &posts_html).expect("failed to write index file");
}

fn generate_not_found(
    parser: &liquid::Parser,
    head_template: &liquid::Template,
    header: &str,
    footer_license: &str,
) {
    let not_found_content = include_str!("../templates/static_blocks/notfound.html");
    let not_found = generate_generic(
        parser,
        head_template,
        header,
        footer_license,
        &"Not Found",
        &not_found_content,
    );
    fs::write("static/notfound.html", &not_found).expect("failed to write 404 file");
}

fn generate() -> Result<(), String> {
    let head_tpl = include_str!("../templates/snippets/head.html");
    let post_tpl = include_str!("../templates/post.html");

    let header = include_str!("../templates/static_blocks/header.html");

    let parser = liquid::ParserBuilder::with_liquid()
        .build()
        .expect("failed to build parser");

    let footer_license_block = generate_footer_license_block(&parser);

    let head_template = &parser
        .parse(&head_tpl)
        .expect("couldn't parse head template");
    let post_template = &parser.parse(&post_tpl).expect("couldn't parse post");

    generate_about(&parser, &head_template, &header, &footer_license_block);
    generate_not_found(&parser, &head_template, &header, &footer_license_block);

    let md_opts = get_md_opts();

    let mut posts = fs::read_dir("posts")
        .expect("Couldn't read posts dir")
        .map(|r| r.expect(&"couldn't read dir entry"))
        .filter(|de| de.file_type().unwrap().is_file())
        .filter(|de| &de.file_name().len() > &0)
        .filter(|de| &de.file_name().to_string_lossy()[0..1] != ".")
        .map(|md| {
            let md_txt =
                fs::read_to_string(md.path()).expect(&format!("couldn't read md: {:?}", md));
            let metadata = Metadata::new(&md_txt);
            let md_content = &md_txt
                .lines()
                .skip(Metadata::NUM_HEADER_LNS.into())
                .collect::<Vec<&str>>()
                .join("\n");
            let content = md_to_html(&md_content, md_opts);
            Post { metadata, content }
        })
        .collect::<Vec<Post>>();

    // sort posts by date descending

    posts.sort_by(|a, b| a.metadata.updated.cmp(&b.metadata.updated).reverse());

    generate_index_and_posts(
        &parser,
        &head_template,
        &header,
        &footer_license_block,
        &posts,
    );

    for (i, post) in posts.iter().enumerate() {
        let head = generate_head_block(&head_template, post.metadata.title.to_owned());
        let footer_nav = generate_footer_nav_block(
            &parser,
            // prev is next in vec
            posts.get(i + 1).map(|p| p.metadata.slug.to_owned()),
            // next is prev in vec
            if i > 0 {
                Some(posts[i - 1].metadata.slug.to_owned())
            } else {
                None
            },
        );
        let globals = liquid::value::Object::from_iter(vec![
            ("head".into(), to_liquid_val(head)),
            ("header".into(), to_liquid_val(header)),
            ("content".into(), to_liquid_val(&post.content)),
            (
                "date".into(),
                to_liquid_val(format!("{}", post.metadata.updated.format("%Y-%m-%d"))),
            ),
            ("tags".into(), to_liquid_val(post.metadata.tags.join(", "))),
            ("footer-nav".into(), to_liquid_val(footer_nav)),
            (
                "footer-license".into(),
                to_liquid_val(&footer_license_block),
            ),
        ]);
        let html = post_template
            .render(&globals)
            .expect(&format!("failed to render post: {:?}", post));
        fs::write(format!("static/posts/{}.html", &post.metadata.slug), &html)
            .expect(&format!("couldn't write post: {:?}", post));
    }
    Ok(())
}

fn run() {
    let index = warp::fs::dir("static");
    warp::serve(index).run(([127, 0, 0, 1], 5000));
}

fn cli<'a>() -> ArgMatches<'a> {
    App::new("speedy")
        .subcommand(SubCommand::with_name("generate"))
        .subcommand(SubCommand::with_name("run"))
        .get_matches()
}

fn publish() {
    Command::new("git")
        .args(&["-C", "static", "add", "*"])
        .output()
        .unwrap();
    Command::new("git")
        .args(&[
            "-C",
            "static",
            "commit",
            "-m",
            &format!("{:?}", Local::now()),
        ])
        .output()
        .unwrap();
    Command::new("git")
        .args(&["-C", "static", "push", "target", "master"])
        .output()
        .unwrap();
}

fn main() {
    let opts = cli();
    match opts.subcommand_name() {
        Some("run") => run(),
        Some("generate") => generate().expect("generation failed"),
        Some("publish") => publish(),
        Some(_) => println!("??"),
        None => run(),
    }
}
