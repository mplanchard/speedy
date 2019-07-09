title: Adding Support for Tags: Part 4
slug: adding-support-for-tags-4
created: 2019-07-08
updated: 2019-07-08
tags: rust, blog, programming
summary: actually generating tag content

# Adding Tags to my Static Site: Part 4

* Part 1 [here](/posts/adding-support-for-tags-1.html)
* Part 2 [here](/posts/adding-support-for-tags-2.html)
* Part 3 [here](/posts/adding-support-for-tags-3.html)

Alright! This is the end. In part one we did some cleanup and refactoring
of the way we were parsing headers from our markdown blog posts. In part
two we wrote some functionality allowing us to collect a [`HashMap`] of
references to our `Post` objects, keyed by their tags. In part three,
we went hog wild on the refactoring, creating a global constant with all
of our template strings and creating a single `Context` object that we
can use for all of our template rendering.

While the refactoring in part three was definitely not necessary to
accomplish our goals in regards to tags, it will definitely make this
part of things easier.

So, on we go.

## HTML

Just as a reminder, in part two we created the following snippet, with
the intention of using it to render a tag name with its associated
posts:

```html
<h4>{{ tag }}</h4>
{{ posts }}
```

Let's update it a little bit to support anchor links:

```html
<h4 class="tag-header">
    <a id="{{ tag }}" href="tags.html#{{ tag }}">{{ tag }}</a>
</h4>
{{ posts }}
```

We can use our existing post template to fill out `{{ posts }}`:

```html
<li>
    <a href="/posts/{{ slug }}.html">{{ title }}</a>
    &mdash; {{ summary }}
</li>
```

We can then stick a newline-joined series of the rendered tag templates
directly into our `generic.html` generic template:

```html
<!DOCTYPE html>
<html>

{{ head }}

<body>
    <header>
        {{ header }}
    </header>
    <main>
        {{ content }}
    </main>
    <footer>
        {{ footer-license }}
    </footer>
</body>

</html>
```

Since we have methods for easily rendering a generic page by just passing
its title and its content, it should be easy.

## Rust Prep

After the refactoring in part three, we have a collection of all of our
`Post` instances available on the `Context` instance to use at any time.
We also have all of our posts pre-rendered using the post template shown
above, stored on our `Context.pre_rendered.post_summaries` attribute.

Unfortunately, those summaries are just stored as a list of strings, so
there's no way to grab them via our tag-to-post mapping. It seems like
it might be a better idea to render our post summaries and put them
directly on the `Post` objects with which they're associated. This should
allow us to avoid extra rendering and keep things better encapsulated.
The transition is easy. Currently, post summaries are created like:

```rust
struct PreRenderedTemplates {
    footer_license: String,
    // Post list items, with title, summary & link
    post_summaries: Vec<String>,
}

    ...

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
```

There's no good reason we can't pull the map out into its own method,
call it for each `Post` we construct, and be good to go. Let's see how
that looks:

```rust
#[derive(Debug)]
struct Post {
    content: String,
    metadata: Metadata,
    rendered_summary: String,
}
impl Post {
    fn new(post_summary_template: &liquid::Template, metadata: Metadata, content: String) -> Self {
        Self {
            content,
            rendered_summary: Self::render_summary(&post_summary_template, &metadata),
            metadata,
        }
    }

    fn render_summary(template: &liquid::Template, metadata: &Metadata) -> String {
        let globals = liquid::value::Object::from_iter(vec![
            ("slug".into(), to_liquid_val(&metadata.slug)),
            ("title".into(), to_liquid_val(&metadata.title)),
            ("summary".into(), to_liquid_val(&metadata.summary)),
        ]);
        template
            .render(&globals)
            .expect(&format!("couldn't render post summary: {:?}", metadata))
    }
}
```

now we've got to update our `Post` construction elsewhere to use our new
`new()` method. Since we're only actually creating posts once on our
initial creation of the `Context` instance, this is easy. Now, when we
instantiate our `Context` instance:

```rust
impl Context {
    fn new() -> Self {
        let parser = liquid::ParserBuilder::with_liquid()
            .build()
            .expect("failed to build parser");
        let templates = Templates::new(&parser);
        let posts = Self::collect_posts(&templates.snippets.posts_post);
        Self {
            blocks: TEMPLATE_STRINGS.blocks,
            pre_rendered: PreRenderedTemplates::new(&templates, &posts),
            posts,
            templates,
        }
    }
```

we have a list of posts on `posts` that also includes the pre-rendered
summary text. We can update our renderers for the index and posts pages
to use this new location. As is often the case in Rust, we can use the
compiler to help us refactor. In this case, if we remove the reference
from our `PreRendered` struct where we were previously storing rendered
post summaries, it will be obvious where we need to update calls to
use the new location.

We're immediately directed to our `render_index_page` method on `Context`:

```rust
fn render_index_page(&self) -> String {
    let index_content_globals = liquid::value::Object::from_iter(vec![(
        "posts".into(),
        to_liquid_val(
            self.pre_rendered
                .post_summaries  // compiler error here
                .iter()
                .take(IDX_NUM_RECENT_POSTS.into())
                .map(|p| p.as_str())
                .collect::<Vec<&str>>()
                .join("\n"),
        ),
    )]);

    ...

```

fixing this is straightforward:

```rust
fn render_index_page(&self) -> String {
    let index_content_globals = liquid::value::Object::from_iter(vec![(
        "posts".into(),
        to_liquid_val(
            self.posts
                .iter()
                .take(IDX_NUM_RECENT_POSTS.into())
                .map(|p| p.rendered_summary.as_str())
                .collect::<Vec<&str>>()
                .join("\n"),
        ),
    )]);

    ...

```

### An Aside on Self-Referential Structs

The next thing I wanted to do was call our `tags_map()` function from
[part two] as part of the setup of our `Context` object, that way our map
could be used any time we need it without reconstructing it. I tried
this:

```rust
struct Context<'a> {
    blocks: TemplateBlockStrings,
    posts: Vec<Post>,
    pre_rendered: PreRenderedTemplates,
    tag_map: HashMap<&'a String, Vec<&'a Post>>,
    templates: Templates,
}
impl<'a> Context<'a> {
    fn new() -> Self {
        let parser = liquid::ParserBuilder::with_liquid()
            .build()
            .expect("failed to build parser");
        let templates = Templates::new(&parser);
        let posts = Self::collect_posts(&templates.snippets.posts_post);
        Self {
            blocks: TEMPLATE_STRINGS.blocks,
            pre_rendered: PreRenderedTemplates::new(&templates, &posts),
            posts,
            tag_map: Self::tag_map(&posts),
            templates,
        }
    }
```

but oh boy was the compiler unhappy:

```rust
error[E0515]: cannot return value referencing local variable `posts`
   --> src/main.rs:259:9
    |
257 |           let tag_map = Self::tag_map(&posts);
    |                                       ------ `posts` is borrowed here
258 |           let pre_rendered = PreRenderedTemplates::new(&templates, &posts);
259 | /         Self {
260 | |             blocks: TEMPLATE_STRINGS.blocks,
261 | |             pre_rendered,
262 | |             tag_map: tag_map,
263 | |             posts: posts,
264 | |             templates,
265 | |         }
    | |_________^ returns a value referencing data owned by the current function

error[E0505]: cannot move out of `posts` because it is borrowed
   --> src/main.rs:263:20
    |
250 |   impl<'a> Context<'a> {
    |        -- lifetime `'a` defined here
...
257 |           let tag_map = Self::tag_map(&posts);
    |                                       ------ borrow of `posts` occurs here
258 |           let pre_rendered = PreRenderedTemplates::new(&templates, &posts);
259 | /         Self {
260 | |             blocks: TEMPLATE_STRINGS.blocks,
261 | |             pre_rendered,
262 | |             tag_map: tag_map,
263 | |             posts: posts,
    | |                    ^^^^^ move out of `posts` occurs here
264 | |             templates,
265 | |         }
    | |_________- returning this value requires that `posts` is borrowed for `'a`

error: aborting due to 2 previous errors
```

It's essentially telling me that I can't have a reference to `posts`,
because `posts` is getting moved out of the `new()` function, and no
matter how I messed around with the lifetimes, I could not get tell
the compiler that I wanted the `tag_map` to live for as long as the
constructed instance. This led me to a [really great Stack Overflow question](https://stackoverflow.com/questions/32300132/why-cant-i-store-a-value-and-a-reference-to-that-value-in-the-same-struct).
The top answer there does an excellent job of describing why this is a
problem, even though from a naïve it seems like it shouldn't be.

Basically, it's important to understand that lifetime isn't necessarily
referring to the lifetime of the thing the reference points to. It's instead
referring to _how long a reference to a given location in memory will be valid._
If we think about it this way, it makes more sense. The **return value** of
the function is going to be literally moved to another location in memory
when it's called and assigned to a value. At that point, if it contained
an attribute that referenced a location in memory associated with the struct
_during its construction_, that reference will no longer be valid.

I highly recommend reading the top answer on that Stack Overflow question,
as it explains the whole thing in much more detail and with examples.

Ultimately, this is a great example of Rust's borrow checker saving me again
from code that I would have thought to have been perfectly valid!

So, how do we resolve the issue? I decided to make a "core" struct that
would hold all of the actual values for the `Context`, which the `Context`
could then just store references to. So, now we have `ContextData`, into
which I've pulled the `collect_posts` logic:

```rust
struct ContextData {
    posts: Vec<Post>,
    pre_rendered: PreRenderedTemplates,
    templates: Templates,
}
impl ContextData {
    fn new() -> Self {
        let parser = liquid::ParserBuilder::with_liquid()
            .build()
            .expect("failed to build parser");
        let templates = Templates::new(&parser);
        let posts = Self::collect_posts(&templates.snippets.posts_post);
        let pre_rendered = PreRenderedTemplates::new(&templates);
        ContextData { posts, pre_rendered, templates }
    }

    fn collect_posts(post_summary_template: &liquid::Template) -> Vec<Post> { ... }

    ...
}
```

and we've updated `Context` to hold lots of references. Note that we had to
specify that the lifetime of the passed `ContextData` reference must be
the same as the lifetime of the references on our struct.

```rust
/// Maintain structs and data to be shared among rendering functions
struct Context<'a> {
    blocks: TemplateBlockStrings,
    posts: &'a Vec<Post>,
    pre_rendered: &'a PreRenderedTemplates,
    tag_map: HashMap<&'a String, Vec<&'a Post>>,
    templates: &'a Templates,
}
impl<'a> Context<'a> {
    fn new(data: &'a ContextData) -> Self {
        let tag_map = Self::tag_map(&data.posts);
        Self {
            blocks: TEMPLATE_STRINGS.blocks,
            pre_rendered: &data.pre_rendered,
            tag_map: tag_map,
            posts: &data.posts,
            templates: &data.templates,
        }
    }
    ...
}
```

Then we just update our `generate()` function to create the `ContextData`
instance and pass it into the `Context` struct:

```rust
fn generate() {
    let context_data = ContextData::new();
    let context = Context::new(&context_data);
    context.generate_all();
}
```

## Adding the Tags Page

Since we have our tag map available on our `Context` object now, we can
go ahead and write a few functions to render the content of our tag page:

```rust
fn render_tag_for_tags_page<S: AsRef<str>>(&self, tag: &S) -> String {
    let posts = self.tag_map.get(tag.as_ref()).expect("Tag disappeared?");
    let post_content = posts
        .iter()
        .map(|p| p.rendered_summary.as_str())
        .collect::<Vec<&str>>()
        .join("\n");
    let tag_globals = liquid::value::Object::from_iter(vec![
        ("tag".into(), to_liquid_val(tag)),
        ("posts".into(), to_liquid_val(post_content)),
    ]);
    self.templates
        .snippets
        .tag_posts
        .render(&tag_globals)
        .expect(&format!("Couldn't render tag: {}", tag.as_ref()))
}

fn render_tags_page_content(&self) -> String {
    let mut tags = self.tag_map.keys().collect::<Vec<&&str>>();
    tags.sort();

    tags.into_iter()
        .map(|t| self.render_tag_for_tags_page(t))
        .collect::<Vec<String>>()
        .join("\n")
}

fn render_tags_page(&self) -> String {
    let tags_page_content = self.render_tags_page_content();
    self.render_generic_page("Tags", &tags_page_content)
}
```

and of course we need to write it to disk:

```rust
fn generate_tags_page(&self) {
    fs::write("static/tags.html", &self.render_tags_page()).expect("failed to write tags file");
}
```

Done!

## Linking from Posts

From here, we can also update the renderer for a given post page to not
just show the tags for the post, but link to our anchors on our tags page!

The template for a Post page looks like this:

```html
<html>

{{ head }}

<body>
    <header>
        {{ header }}
    </header>
    <main>
        {{ content }}
        <section class="post-metadata">
            <p>Last Updated: {{ date }}</p>
            <p>Tags: {{ tags }}</p>
        </section>
    </main>
    <footer>
        {{ footer-nav }}
        {{ footer-license }}
    </footer>
</body>

</html>
```

and we're populating that with these globals, from `render_post_page()`:

```rust
let mut globals_vec = self.generic_globals_vec(&post.metadata.title, &post.content);
globals_vec.extend(vec![
    (
        "date".into(),
        to_liquid_val(format!("{}", post.metadata.updated.format("%Y-%m-%d"))),
    ),
    ("footer-nav".into(), to_liquid_val(footer_nav)),
    ("tags".into(), to_liquid_val(post.metadata.tags.join(", "))),
]);
```

So, we need to replace our tags with a little link snippet, which looks like
this:

```html
<a href="/tags.html#{{ tag }}">{{ tag }}</a>
```

Write a function to render a tag link:

```rust
fn render_tag_link<S: AsRef<str>>(&self, tag: &S) -> String {
    let globals =
        liquid::value::Object::from_iter(vec![("tag".into(), to_liquid_val(&tag.as_ref()))]);
    self.templates
        .snippets
        .tag_link
        .render(&globals)
        .expect(&format!("Couldn't render tag link: {}", tag.as_ref()))
}
```

and call it when creating our globals for a given post:

```rust
globals_vec.extend(vec![
    (
        "date".into(),
        to_liquid_val(format!("{}", post.metadata.updated.format("%Y-%m-%d"))),
    ),
    ("footer-nav".into(), to_liquid_val(footer_nav)),
    (
        "tags".into(),
        to_liquid_val(
            post.metadata.tags
                .iter()
                .map(|t| self.render_tag_link(&t))
                .collect::<Vec<String>>()
                .join(", ")
        )
    ),
]);
```

We can also add in a `Tags` link to our static header content:

```html
<div>
    <a href="/">
        <abbr title="Matthew Planchard">MP</abbr>
    </a>
</div>
<nav class="header-links">
    <ol>
        <li class="header-link">
            <a href="/posts.html">
                Posts
            </a>
        </li>
        <li class="header-link">
            <a href="/tags.html">
                Tags
            </a>
        </li>
        <li class="header-link">
            <a href="/about.html">
                About
            </a>
        </li>
    </ol>
</nav>
```

Generated, checked, and functional!

## Conclusion

Well, that is the end of the #tags saga, for now. This was a rather meandering
path through lots of interesting refactors, into the depths of the Rust
borrow checker, and finally to actually rendering some tag links. The next
major part of the building out of the website saga will be making an
RSS feed (yayyy, XML)!

:)

[part two]: /posts/adding-support-for-tags-2.html
[liquid template]: https://docs.rs/liquid/0.18.2/liquid/struct.Template.html

[`DirEntry`]: https://doc.rust-lang.org/std/fs/struct.DirEntry.html
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`include_str!`]: https://doc.rust-lang.org/1.7.0/std/macro.include_str!.html
[`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
