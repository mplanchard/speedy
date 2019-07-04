use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::iter::FromIterator;
use std::process::Command;

use chrono::{Local, NaiveDate};
use clap::{App, ArgMatches, SubCommand};
use liquid;
use pulldown_cmark::{html, Options as MDOptions, Parser as MDParser};
use warp;


const IDX_NUM_RECENT_POSTS: u8 = 10;


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


struct TemplateBlockStrings {
    about: &'static str,
    header: &'static str,
    notfound: &'static str,
}

struct TemplatePageStrings {
    about: &'static str,
    generic: &'static str,
    index: &'static str,
    post: &'static str,
}

struct TemplateSnippetStrings {
    footer_license: &'static str,
    footer_nav_content: &'static str,
    footer_nav: &'static str,
    head: &'static str,
    index_content: &'static str,
    posts_content: &'static str,
    posts_post: &'static str,
    tag_posts: &'static str,
}

/// Raw template HTML
struct TemplateStrings {
    blocks: TemplateBlockStrings,
    pages: TemplatePageStrings,
    snippets: TemplateSnippetStrings,
}

const TEMPLATE_STRINGS: TemplateStrings = TemplateStrings {
    blocks: TemplateBlockStrings {
        about: include_str!("../templates/blocks/about.html"),
        header: include_str!("../templates/blocks/header.html"),
        notfound: include_str!("../templates/blocks/notfound.html"),
    },
    pages: TemplatePageStrings {
        about: include_str!("../templates/pages/about.html"),
        generic: include_str!("../templates/pages/generic.html"),
        index: include_str!("../templates/pages/index.html"),
        post: include_str!("../templates/pages/post.html"),
    },
    snippets: TemplateSnippetStrings {
        footer_license: include_str!("../templates/snippets/footer-license.html"),
        footer_nav_content: include_str!("../templates/snippets/footer-nav-content.html"),
        footer_nav: include_str!("../templates/snippets/footer-nav.html"),
        head: include_str!("../templates/snippets/head.html"),
        index_content: include_str!("../templates/snippets/index-content.html"),
        posts_content: include_str!("../templates/snippets/posts-content.html"),
        posts_post: include_str!("../templates/snippets/posts-post.html"),
        tag_posts: include_str!("../templates/snippets/tag-posts.html"),
    },
};


struct PageTemplates {
    about: liquid::Template,
    generic: liquid::Template,
    index: liquid::Template,
    post: liquid::Template,
}
impl PageTemplates {
    fn new(parser: &liquid::Parser) -> Self {
        let parse = |template_str| parse_template_str(parser, template_str);
        Self {
            about: parse(TEMPLATE_STRINGS.pages.about),
            generic: parse(TEMPLATE_STRINGS.pages.generic),
            index: parse(TEMPLATE_STRINGS.pages.index),
            post: parse(TEMPLATE_STRINGS.pages.post),
        }
    }
}


struct SnippetTemplates {
    footer_license: liquid::Template,
    footer_nav_content: liquid::Template,
    footer_nav: liquid::Template,
    head: liquid::Template,
    index_content: liquid::Template,
    posts_content: liquid::Template,
    posts_post: liquid::Template,
    tag_posts: liquid::Template,
}
impl SnippetTemplates {
    fn new(parser: &liquid::Parser) -> Self {
        let parse = |template_str| parse_template_str(parser, template_str);
        Self {
            footer_license: parse(TEMPLATE_STRINGS.snippets.footer_license),
            footer_nav_content: parse(TEMPLATE_STRINGS.snippets.footer_nav_content),
            footer_nav: parse(TEMPLATE_STRINGS.snippets.footer_nav),
            head: parse(TEMPLATE_STRINGS.snippets.head),
            index_content: parse(TEMPLATE_STRINGS.snippets.index_content),
            posts_content: parse(TEMPLATE_STRINGS.snippets.posts_content),
            posts_post: parse(TEMPLATE_STRINGS.snippets.posts_post),
            tag_posts: parse(TEMPLATE_STRINGS.snippets.tag_posts),
        }
    }
}

struct Templates {
    pages: PageTemplates,
    snippets: SnippetTemplates,
}
impl Templates {
    fn new(parser: &liquid::Parser) -> Self {
        Self {
            pages: PageTemplates::new(parser),
            snippets: SnippetTemplates::new(parser),
        }
    }
}

struct PreRenderedTemplates {
    footer_license: String,
    // Post list items, with title, summary & link
    post_summaries: Vec<String>,
}
impl PreRenderedTemplates {
    fn new(templates: &Templates, posts: &Vec<Post>) -> Self {
        Self {
            footer_license: Self::render_footer_license(&templates.snippets.footer_license),
            post_summaries: Self::render_post_summaries(&templates.snippets.posts_post, posts),
        }
    }

    fn render_footer_license(template: &liquid::Template) -> String {
        let today = format!("{}", Local::today().format("%Y-%m-%d"));
        let globals = liquid::value::Object::from_iter(vec![("year".into(), to_liquid_val(today))]);

        template
            .render(&globals)
            .expect("failed to render footer license template")
    }

    fn render_post_summaries<'a, P: IntoIterator<Item = &'a Post>>(
        template: &liquid::Template,
        posts: P,
    ) -> Vec<String> {
        posts
            .into_iter()
            .map(|p| {
                let globals = liquid::value::Object::from_iter(vec![
                    ("slug".into(), to_liquid_val(&p.metadata.slug)),
                    ("title".into(), to_liquid_val(&p.metadata.title)),
                    ("summary".into(), to_liquid_val(&p.metadata.summary)),
                ]);
                template
                    .render(&globals)
                    .expect(&format!("couldn't render post: {:?}", p))
            })
            .collect()
    }
}

/// Maintain structs and data to be shared among rendering functions
struct Context {
    blocks: TemplateBlockStrings,
    // All posts, sorted by date of creation descending
    posts: Vec<Post>,
    pre_rendered: PreRenderedTemplates,
    templates: Templates,
}
impl Context {
    fn new() -> Self {
        let parser = liquid::ParserBuilder::with_liquid()
            .build()
            .expect("failed to build parser");
        let templates = Templates::new(&parser);
        let posts = Self::collect_posts();
        Self {
            blocks: TEMPLATE_STRINGS.blocks,
            pre_rendered: PreRenderedTemplates::new(&templates, &posts),
            posts: posts,
            templates: templates,
        }
    }

    fn collect_posts() -> Vec<Post> {
        let md_opts = Self::get_md_opts();
        let mut posts = files_from_dir("posts")
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
        posts.sort_by(|a, b| a.metadata.created.cmp(&b.metadata.created).reverse());
        posts
    }

    fn generate_all(&self) {
        self.generate_about_page();
        self.generate_index_page();
        self.generate_notfound_page();
        self.generate_post_pages();
        self.generate_posts_page();
    }

    fn generate_about_page(&self) {
        fs::write("static/about.html", &self.render_about_page())
            .expect("couldn't write about.html");
    }

    fn generate_index_page(&self) {
        fs::write("static/index.html", &self.render_index_page())
            .expect("failed to write index file");
    }

    fn generate_notfound_page(&self) {
        fs::write("static/notfound.html", &self.render_notfound_page())
            .expect("failed to write 404 file");
    }

    fn generate_post_page<S: AsRef<str>, T: AsRef<str>>(&self, slug: S, content: T) {
        fs::write(
            format!("static/posts/{}.html", slug.as_ref()),
            content.as_ref(),
        )
        .expect(&format!("couldn't write post: {}", slug.as_ref()));
    }

    fn generate_post_pages(&self) {
        for (i, post) in self.posts.iter().enumerate() {
            self.generate_post_page(&post.metadata.slug, &self.render_post_page(i, &post))
        }
    }

    fn generate_posts_page(&self) {
        fs::write("static/posts.html", &self.render_posts_page())
            .expect("failed to write index file");
    }

    fn generic_globals_vec<S: AsRef<str>, T: AsRef<str>>(
        &self,
        title: S,
        content: T,
    ) -> Vec<(Cow<'static, str>, liquid::value::Value)> {
        let head = self.render_head_block(String::from(title.as_ref()));
        vec![
            ("head".into(), to_liquid_val(head)),
            ("header".into(), to_liquid_val(&self.blocks.header)),
            ("content".into(), to_liquid_val(content)),
            (
                "footer-license".into(),
                to_liquid_val(&self.pre_rendered.footer_license),
            ),
        ]
    }

    fn generic_globals<S: AsRef<str>, T: AsRef<str>>(
        &self,
        title: S,
        content: T,
    ) -> liquid::value::Object {
        liquid::value::Object::from_iter(self.generic_globals_vec(title, content))
    }

    fn get_md_opts() -> MDOptions {
        let mut options = MDOptions::empty();
        options.insert(MDOptions::ENABLE_FOOTNOTES);
        options.insert(MDOptions::ENABLE_TABLES);
        options.insert(MDOptions::ENABLE_STRIKETHROUGH);
        options
    }

    fn render_about_page(&self) -> String {
        let head = self.render_head_block(String::from("About"));
        let globals = self.generic_globals("About", &self.blocks.about);
        self.templates
            .pages
            .about
            .render(&globals)
            .expect("failed to render head template")
    }

    fn render_footer_inner_content_block<S: AsRef<str>, T: AsRef<str>>(
        &self,
        slug: Option<S>,
        description: T,
    ) -> String {
        if let Some(s) = slug {
            let globals = liquid::value::Object::from_iter(vec![
                ("slug".into(), to_liquid_val(s)),
                ("description".into(), to_liquid_val(description)),
            ]);
            self.templates
                .snippets
                .footer_nav_content
                .render(&globals)
                .expect("failed to render footer nav content")
        } else {
            "".into()
        }
    }

    fn render_footer_nav_block<S: AsRef<str>, T: AsRef<str>>(
        &self,
        prev_slug: Option<S>,
        next_slug: Option<T>,
    ) -> String {
        let left_content = self.render_footer_inner_content_block(prev_slug, "&lt previous");
        let right_content = self.render_footer_inner_content_block(next_slug, "next &gt");

        let footer_nav_globals = liquid::value::Object::from_iter(vec![
            ("left_content".into(), to_liquid_val(left_content)),
            ("right_content".into(), to_liquid_val(right_content)),
        ]);

        self.templates
            .snippets
            .footer_nav
            .render(&footer_nav_globals)
            .expect("failed to render footer nav template")
    }

    fn render_generic_page(&self, title: &str, content: &str) -> String {
        let globals = self.generic_globals(&title, &content);
        self.templates
            .pages
            .generic
            .render(&globals)
            .expect("failed to render generic template")
    }

    fn render_head_block(&self, title: String) -> String {
        let globals = liquid::value::Object::from_iter(vec![(
            "title".into(),
            liquid::value::Value::scalar(title),
        )]);
        self.templates
            .snippets
            .head
            .render(&globals)
            .expect("failed to render head template")
    }

    fn render_index_page(&self) -> String {
        let index_content_globals = liquid::value::Object::from_iter(vec![(
            "posts".into(),
            to_liquid_val(
                self.pre_rendered
                    .post_summaries
                    .iter()
                    .take(IDX_NUM_RECENT_POSTS.into())
                    .map(|p| p.to_owned())
                    .collect::<Vec<String>>()
                    .join("\n"),
            ),
        )]);
        let index_content = self
            .templates
            .snippets
            .index_content
            .render(&index_content_globals)
            .expect("couldn't render index content");
        let index_globals = self.generic_globals("Home", &index_content);

        self.templates
            .pages
            .index
            .render(&index_globals)
            .expect("couldn't render index")
    }

    fn render_notfound_page(&self) -> String {
        self.render_generic_page("Not Found", &self.blocks.notfound)
    }

    fn render_posts_page(&self) -> String {
        let posts_content_globals = liquid::value::Object::from_iter(vec![(
            "posts".into(),
            to_liquid_val(&self.pre_rendered.post_summaries.join("\n")),
        )]);
        let posts_content = self
            .templates
            .snippets
            .posts_content
            .render(&posts_content_globals)
            .expect("couldn't render posts content");

        self.render_generic_page("Posts", &posts_content)
    }

    fn render_post_page(&self, index: usize, post: &Post) -> String {
        // let head = self.render_head_block(post.metadata.title.to_owned());
        let footer_nav = self.render_footer_nav_block(
            // prev is next in vec
            self.posts
                .get(index + 1)
                .map(|p| p.metadata.slug.to_owned()),
            // next is prev in vec
            if index > 0 {
                Some(self.posts[index - 1].metadata.slug.to_owned())
            } else {
                None
            },
        );
        let mut globals_vec = self.generic_globals_vec(&post.metadata.title, &post.content);
        globals_vec.extend(vec![
            (
                "date".into(),
                to_liquid_val(format!("{}", post.metadata.updated.format("%Y-%m-%d"))),
            ),
            ("footer-nav".into(), to_liquid_val(footer_nav)),
            ("tags".into(), to_liquid_val(post.metadata.tags.join(", "))),
        ]);
        let globals = liquid::value::Object::from_iter(globals_vec);
        self.templates
            .pages
            .post
            .render(&globals)
            .expect(&format!("failed to render post: {:?}", post))
    }
}


fn files_from_dir<S: AsRef<str>>(path: S) -> impl Iterator<Item = fs::DirEntry> {
    fs::read_dir(path.as_ref())
        .expect(&format!("Couldn't read dir: {}", path.as_ref()))
        .map(|r| r.expect(&"couldn't read dir entry"))
        // exclude directories
        .filter(|de| (&de.file_type().unwrap()).is_file())
        // exclude zero-length file names
        .filter(|de| &de.file_name().len() > &0)
        // exclude any files whose names start with a dot (I'm looking at you,
        // .DS_Store!)
        .filter(|de| &de.file_name().to_string_lossy()[0..1] != ".")
}


fn md_to_html(md: &str, opts: MDOptions) -> String {
    let mut html = String::new();
    html::push_html(&mut html, MDParser::new_ext(&md, opts));
    html
}

fn parse_template_str<S: AsRef<str>>(parser: &liquid::Parser, template: S) -> liquid::Template {
    parser
        .parse(&template.as_ref())
        .expect(&format!("Couldn't parse template: {}", template.as_ref()))
}

fn to_liquid_val<S: AsRef<str>>(string: S) -> liquid::value::Value {
    liquid::value::Value::scalar(string.as_ref().to_owned())
}

fn tag_map<'a, T>(posts: T) -> HashMap<&'a String, Vec<&'a Post>>
where
    T: IntoIterator<Item = &'a Post>,
{
    let mut tags_to_posts = HashMap::new();
    posts.into_iter().for_each(|post| {
        post.metadata.tags.iter().for_each(|tag| {
            tags_to_posts
                .entry(tag)
                .and_modify(|post_vec: &mut Vec<&Post>| post_vec.push(post))
                .or_insert(vec![post]);
        });
    });
    tags_to_posts
}

fn generate_tags_content(context: &Context, tags_to_posts: HashMap<&String, Vec<&Post>>) -> String {
    let mut tags = tags_to_posts.keys().map(|k| *k).collect::<Vec<&String>>();
    tags.sort();

    tags.into_iter()
        .map(|tag| {
            let posts = tags_to_posts.get(tag).expect("Tag disappeared?");
            let post_content = posts
                .into_iter()
                .map(|post| {
                    let globals = liquid::value::Object::from_iter(vec![
                        ("slug".into(), to_liquid_val(&post.metadata.slug)),
                        ("title".into(), to_liquid_val(&post.metadata.title)),
                        ("summary".into(), to_liquid_val(&post.metadata.summary)),
                    ]);
                    context
                        .templates
                        .snippets
                        .posts_post
                        .render(&globals)
                        .expect(&format!("Could not render post: {:?}", post))
                })
                .collect::<Vec<String>>()
                .join("\n");
            let tag_globals = liquid::value::Object::from_iter(vec![
                ("tag".into(), to_liquid_val(tag)),
                ("posts".into(), to_liquid_val(post_content)),
            ]);
            context
                .templates
                .snippets
                .tag_posts
                .render(&tag_globals)
                .expect(&format!("Couldn't render tag: {}", tag))
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn generate() {
    let context = Context::new();
    context.generate_all();
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
        Some("generate") => generate(),
        Some("publish") => publish(),
        Some(_) => println!("??"),
        None => run(),
    }
}