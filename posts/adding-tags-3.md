title: Adding Support for Tags: Part 3
slug: adding-support-for-tags-3
created: 2019-07-04
updated: 2019-07-04
tags: rust, blog, programming
summary: finishing up refactoring

# Adding Tags to my Static Site: Part 3

* Part 1 [here](/posts/adding-support-for-tags-1.html)
* Part 2 [here](/posts/adding-support-for-tags-2.html)

To summarize what we've done so far, in Part 1 we refactored the way we were
parsing the headers from our markdown files, pulling the functionality into
associated functions and methods for the `Metadata` struct, with a convenient
`Metadata::new()` implementation that allows us to pass either headers
alone or an entire markdown file and get back `Metadata`.

In Part 2, we prepared for rendering our tags summary and tag views by
writing a function that collects references to all of our posts into a
[`HashMap`], with our tags as the keys. We also wrote some simple HTML
templates that we can use to render our tags.

My initial intent was to go straight from there into generating the HTML,
but frankly the code for generating HTML pages is awfully messy. Writing
functions to render some HTML requires passing in four to six parameters,
and there are quite a few of these functions, since so far almost every
page has gotten its own bespoke rendering function. I think we've well
passed the [Rule of Three](https://en.wikipedia.org/wiki/Rule_of_three_(computer_programming))
and that it's time to do some more refactoring.

## Deciding How to Start

So, we have a few of what I consider to be smells scattered throughout
the code:

* lots of very similar functions (indicates the potential to abstract logic)
* functions that take lots of parameters (indicates the ability to create
  structured objects representing common parameters)
* similar, if not identical, blocks of code in a variety of functions (indicates
  the potential for pulling this logic out into one shared location)
* Violations of the single responsibility principle (SRP), e.g. functions which
  both generate HTML and write files to the disk (indicates the opportunity
  to create smaller, dedicated functions, that may be easier to reuse)

So, I think I'm going to proceed in this order:

* Create structs to represent common parameters
* Extract shared logic into dedicated functions
* Replace bespoke functions with calls to generic ones that conform to the SRP

## Collecting Parameters into Structures

So, why do our functions have so many parameters? Well, largely because
there are a lot of shared instances of parsers, templates, rendered HTML,
etc. that it would be inefficient to make inside of each function. So, for
example, the signature of our function to generate the `about.html` page looks
like this:

```rust
fn generate_about(
    parser: &liquid::Parser,
    head_template: &liquid::Template,
    header: &str,
    footer_license: &str,
) { ... }
```

We're passing in a reference to a [liquid::Parser][liquid parser] which we use to render
[liquid::Template][liquid template] instances We pass in a reference to one such template,
which is used to generate the `<head>` portion of the resultant HTML. The
`<head>` HTML is unique per page, so we can't pass in a pre-rendered HTML
blob, but we also don't want to incur the cost of creating the template
instance in every function that renders an HTML page, thus the reference.
Finally, we also pass in reference to a pre-rendered block to go in the
`<header>`, as well as the standard license notification to use as
`<footer>`. These are the same on every page that uses them, so they are
just passed as string slices. As you might imagine, the other functions
used to generate HTML are in a similar state.

Especially in a language like Rust, we need to consider the control
flow of our code. Ideally, we make as few clones of data as possible,
using references wherever we can. Of course, this often necessitates the
need for lifetimes, and requires really thinking about when something will
be defined and how long it will exist. In this case, we have a number of
structs and data that can be created at the beginning of our application
flow and reused throughout. Indeed, this is what the existing code is
doing, but in a rather ad-hoc way. The `generate()` function, which is
the entry point to HTML generation, starts out by defining a bunch of
resources that are passed to all of our bespoke functions:

```rust
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

    ...

}
```

There's no reason why we can't formalize this into a few more organized
structs. Let's give it a shot.

### Making constants constant

Something that's peppered all over the code is calls to [`include_str!`]
to pull in our template text as `&'static str` variables. This is great,
but it does necessitate defining a bunch of variables by hand, and passing
them around by hand where needed. If we were okay with taking a runtime
hit, we could just scan our template directory and use the [registry pattern](https://martinfowler.com/eaaCatalog/registry.html)
to get our template text by name when we need it.

I'm not sure I want to take that hit at the moment, so, for now, and unless
it turns out later to be too much hassle, we'll stick with manually defining
things. That being said, we _can_ provide a better, unified interface:

```rust
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
    footer_nav_first: &'static str,
    footer_nav_last: &'static str,
    footer_nav: &'static str,
    head: &'static str,
    index_content: &'static str,
    posts_content: &'static str,
    posts_post: &'static str,
    tag_posts: &'static str,
}

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
        footer_nav_first: include_str!("../templates/snippets/footer-nav-first.html"),
        footer_nav_last: include_str!("../templates/snippets/footer-nav-last.html"),
        footer_nav: include_str!("../templates/snippets/footer-nav.html"),
        head: include_str!("../templates/snippets/head.html"),
        index_content: include_str!("../templates/snippets/index-content.html"),
        posts_content: include_str!("../templates/snippets/posts-content.html"),
        posts_post: include_str!("../templates/snippets/posts-post.html"),
        tag_posts: include_str!("../templates/snippets/tag-posts.html"),
    },
};
```

Now, our template code is available anywhere via the `TEMPLATE_STRINGS`
constant. Yes, that block was not super fun to write (although vim makes
it pretty easy), and yes, it will require manual updating in the future.
I see this is one of the benefits of this system being a bespoke solution
just for my website, though. I can put in the effort for better performance
by ignoring abstractions I would need if this were a more generalized
solution. Again, it may turn out that eventually the hassle of keeping this
up to date is too much, but for now, it's not too bad.

Now, I'll go ahead and remove all of the previously existing [`include_str!`]
calls, and replace references to their variables with references to this
const. This leads to a lot of transformations of sections like:

```rust
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
```

to:

```rust
let head = generate_head_block(&head_template, String::from("Home"));

let index_template = parser
    .parse(&TEMPLATE_STRINGS.pages.index)
    .expect("Couldn't parse index template");
let index_content_template = parser
    .parse(&TEMPLATE_STRINGS.snippets.index_content)
    .expect("Couldn't parse index content template");
let posts_template = parser
    .parse(&TEMPLATE_STRINGS.pages.generic)
    .expect("Couldn't parse posts template");
let posts_post_tpl = parser
    .parse(&TEMPLATE_STRINGS.snippets.posts_post)
    .expect("couldn't parse posts-post template");
let posts_content_tpl = parser
    .parse(&TEMPLATE_STRINGS.snippets.posts_content)
    .expect("couldn't parse posts-content template");
```

which is much nicer.

It also allows us to reduce our parameters somewhat on functions like
`generate_about`, which were previously taking a reference to the
parsed `header` block. And finally we can delete all of the files we
were keeping around after re-organizing the `templates` directory, since
there are now no [`include_str!`] calls referencing them that might
cause compilation to fail.

#### Removing footer duplication

One of the things this process helped me to realize is that I don't need
so much duplication of footer navigation templates. Currently, there are
`footer-nav-first.html`, `footer-nav-last.html`, and `footer-nav.html`.
These provide the footer navigation for the first post, the last post,
and any post in the middle, respectively. However, they all share the
same basic structure of two divs, with the only difference being the
content of the divs (the `previous` section is empty for the first post,
for example).

As such, we can unify the footer nav templates into one that looks like
this:

```html
<nav class="footer-nav">
    <div class="footer-nav-left">
        {{ left_content }}
    </div>
    <div class="footer-nav-right">
        {{ right_content }}
    </div>
</nav>
```

and then create a single snippet to use for the left/right content:

```html
<a href="/posts/{{ slug }}.html">
    {{ description }}
</a>
```

I replaced the content of `footer-nav.html` with the former, and created
`footer-nav-content.html` with the latter.

Adding the new template necessitates updating `TEMPLATE_STRINGS` and its
associated structs.

This necessitates a slight re-working of our function for generating the
footer nav. Currently it looks like this:

```rust
fn generate_footer_nav_block(
    parser: &liquid::Parser,
    prev_slug: Option<String>,
    next_slug: Option<String>,
) -> String {
    let mut globals = liquid::value::Object::new();
    let template: &'static str;

    if let Some(prev) = prev_slug {
        globals.insert("previous".into(), to_liquid_val(prev));

        if let Some(next) = next_slug {
            globals.insert("next".into(), to_liquid_val(next));
            template = TEMPLATE_STRINGS.snippets.footer_nav;
        } else {
            template = TEMPLATE_STRINGS.snippets.footer_nav_last;
        }
    } else {
        if let Some(next) = next_slug {
            globals.insert("next".into(), to_liquid_val(next));
            template = TEMPLATE_STRINGS.snippets.footer_nav_first
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
```

It bases its decision on which template to use and what to put into the
templating globals on whether there is a previous/next slug. This is fine,
but we can simplify it with our new design:

```rust
fn generate_footer_content_block<S: AsRef<str>, T: AsRef<str>>(
    template: &liquid::Template,
    slug: Option<S>,
    description: T,
) -> String {
    if let Some(s) = slug {
        let globals = liquid::value::Object::from_iter(vec![
            ("slug".into(), to_liquid_val(s)),
            ("description".into(), to_liquid_val(description)),
        ]);
        template
            .render(&globals)
            .expect("failed to render footer nav content")
    } else {
        "".into()
    }
}

fn generate_footer_nav_block<S: AsRef<str>, T: AsRef<str>>(
    footer_nav_template: &liquid::Template,
    content_nav_template: &liquid::Template,
    prev_slug: Option<S>,
    next_slug: Option<T>,
) -> String {
    let left_content =
        generate_footer_content_block(content_nav_template, prev_slug, "&lt previous");
    let right_content =
        generate_footer_content_block(content_nav_template, next_slug, "next &gt");

    let footer_nav_globals = liquid::value::Object::from_iter(vec![
        ("left_content".into(), to_liquid_val(left_content)),
        ("right_content".into(), to_liquid_val(right_content)),
    ]);

    footer_nav_template
        .render(&footer_nav_globals)
        .expect("failed to render footer nav template")
}
```

This allows us to delete our `footer-nav-first/last.html` files and remove
them from our template string global.

### Collecting shared runtime references

Now that we handled one source of code bloat by refactoring our constants
to be globally available rather than passed around all over the place,
we can start working on wrangling the data and structs that must be
created at runtime and which are used frequently by a variety of functions.

One of these is obviously the [`liquid::Parser`]. Others are the [`liquid::Template`]
instances for each of our `TEMPLATE_STRINGS`. For templated content that need only
be rendered once and can be shared across different pages (e.g. the footer,
which is dependent on the year but otherwise static), we may also want to
include those pre-rendered strings.

Let's start with structs to create and hold templates for each of our
`TEMPLATE_STRINGS` groups. Here is a representative one for `PageTemplates`:

```rust
fn parse_template_str<S: AsRef<str>>(parser: &liquid::Parser, template: S) -> liquid::Template {
    parser
        .parse(&template.as_ref())
        .expect(&format!("Couldn't parse template: {}", template.as_ref()))
}

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
```

Having done the same for snippets, we can collect the templates into their
own struct:

```rust
struct Templates {
    page: PageTemplates,
    snippets: SnippetTemplates,
}
impl Templates {
    fn new(parser: &liquid::Parser) -> Self {
        Self {
            page: PageTemplates::new(parser),
            snippets: SnippetTemplates::new(parser),
        }
    }
}
```

And then we can include these in what I'm again arbitrarily choosing to
call a `Context` object, which will contain our globally needed values:

```rust
/// Maintain structs and data to be shared among rendering functions
struct Context {
    templates: Templates,
    blocks: TemplateBlockStrings,
}
impl Context {
    fn new() -> Self {
        let parser = liquid::ParserBuilder::with_liquid()
            .build()
            .expect("failed to build parser");
        Self {
            templates: Templates::new(&parser),
            blocks: TEMPLATE_STRINGS.blocks,
        }
    }
}
```

I know I said earlier that we need the [liquid::Parser][liquid parser] instance everywhere,
but I actually think it's pretty much exclusively used to create templates.
Since we decided to go ahead and make all the templates from the get-go,
we can omit it from our `Context`.

We can also go ahead and add in our pre-rendered template(s), for example
the footer license block:

```rust
struct PreRenderedTemplates {
    footer_license: String,
}
impl PreRenderedTemplates {
    fn new(templates: &Templates) -> Self {
        Self {
            footer_license: Self::generate_footer_license(&templates.snippets.footer_license),
        }
    }

    fn generate_footer_license(template: &liquid::Template) -> String {
        let today = format!("{}", Local::today().format("%Y-%m-%d"));
        let globals = liquid::value::Object::from_iter(vec![("year".into(), to_liquid_val(today))]);

        template
            .render(&globals)
            .expect("failed to render footer license template")
    }
}

/// Maintain structs and data to be shared among rendering functions
struct Context {
    blocks: TemplateBlockStrings,
    rendered: RenderedTemplates,
    templates: Templates,
}
impl Context {
    fn new() -> Self {
        let parser = liquid::ParserBuilder::with_liquid()
            .build()
            .expect("failed to build parser");
        let templates = Templates::new(&parser);
        Self {
            rendered: RenderedTemplates::new(&templates),
            templates: templates,
            blocks: TEMPLATE_STRINGS.blocks,
        }
    }
}
```

### Putting it together

Now that we've got our templates strings in a global constant and our actual
templates and blocks all stored in a `Context`, we can update most of our
functions to simply take the `Context` object.

Just as a reminder, the top of our `generate()` function previously looked
like this:

```rust
fn generate() -> Result<(), String> {

    let parser = liquid::ParserBuilder::with_liquid()
        .build()
        .expect("failed to build parser");

    let footer_license_block = generate_footer_license_block(&parser);

    let head_template = &parser
        .parse(&TEMPLATE_STRINGS.snippets.head)
        .expect("couldn't parse head template");
    let post_template = &parser
        .parse(&TEMPLATE_STRINGS.pages.post)
        .expect("couldn't parse post");
    let footer_nav_template = &parser
        .parse(&TEMPLATE_STRINGS.snippets.footer_nav)
        .expect("couldn't parse footer nav template");
    let footer_nav_content_template = &parser
        .parse(&TEMPLATE_STRINGS.snippets.footer_nav_content)
        .expect("couldn't parse footer nav content template");

    generate_about(&parser, &head_template, &footer_license_block);
    generate_not_found(&parser, &head_template, &footer_license_block);

    ...

}
```

And now we've got:

```rust
fn generate() -> Result<(), String> {
    let context = Context::new();
    generate_about(&context);
    generate_not_found(&context);

    ...
}
```

Definitely much cleaner!

## Pulling functions into methods

As I was working on the refactoring above, I noticed that I was winding up
with lots and lots of functions that took either solely or as their first
parameter a `&Context`. That suggests to me that they might be better
organized into methods on the `Context` impl, where they can just use
`&self`.

I'm not going to go through all of the details here, but on the whole this
led to the biggest improvement so far. I also took the opportunity to separate
out rendering of HTML from writing of HTML, so there are now `generate_x`
functions on `Context` which call `render_x` functions, and then write the
result to a file.

In addition, I was able to populate the `pre_rendered` struct on `Context`,
with all of the post summary snippets. This allows both the index and
the posts page rendering methods to access that shared data.

With all of this in place, I added a `.generate_all()` method to the
`Context`, so our `generate()` entrypoint now looks like:

```rust
fn generate() {
    let context = Context::new();
    context.generate_all();
}
```

Phew! With all that done, I think we're finally ready to render tags!

That will be the subject of Part 4 (and almost certainly the final part)
of this series.

[liquid parser]: https://docs.rs/liquid/0.18.2/liquid/struct.Parser.html
[liquid template]: https://docs.rs/liquid/0.18.2/liquid/struct.Template.html

[`DirEntry`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`include_str!`]: https://doc.rust-lang.org/1.7.0/std/macro.include_str!.html
[`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
