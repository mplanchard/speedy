title: Creating an RSS (Atom) Feed
slug: creating-an-rss-feed
created: 2019-07-30
updated: 2019-07-30
tags: rust, rss, programming, blog, atom
summary: creating an RSS (Atom) feed from scratch

# Creating an RSS (Atom) Feed

The main way that I keep up with blogs is through RSS. I have been using
the excellent [Reeder](https://reederapp.com/) since whenever Google Reader
shut down, with [Feedly](https://feedly.com/i/welcome) as the backend.
Naturally, I therefore would like for my own blog to have an
RSS feed. This serves two purposes: one, it helps any interested people to
keep up with the blog, and two, if I put it in my feed, it will give me
a second opportunity to proofread things that I post.

This post will follow the process of making an RSS feed from scratch.
I figured that, in the spirit of this generally handcrafted blog experience,
I might as well make it myself rather than going with some third-party
service.

## RSS or Atom

One very quickly learns that there are two competing specifications in the
real simple syndication world. [RSS 2.0][RSS Specification], published in
2003, was the successor to the original RSS specification and was the
dominant player prior to [Atom][Atom Specification], published in 2005 with
what seems to my uneducated eye to be a bit more of a formal specification.

Web searches like "rss vs atom" aren't particularly enlightening as to what
might lead one to pick one or the other feed format, although it does
seem to be a topic that several blog-related companies have written fluff
pieces about in order to improve their SEO, which I will refrain from linking
here. Fluff aside, I did manage to find a [more substantive discussion](https://github.com/jekyll/jekyll-feed/issues/2)
on the [jekyll-feed GitHub repository](https://github.com/jekyll/jekyll-feed).
There's also [this post from the _null program_ blog](https://nullprogram.com/blog/2013/09/23/),
which looks at some of the details about the pain points in the [RSS specification]
as compared to the [Atom specification]. There's also a [question from 2010](https://wordpress.stackexchange.com/questions/2922/should-i-provide-rss-or-atom-feeds)
on the Wordpress StackExchange.

From reading through those links, what I've gathered is mostly that:

* Basically any feed aggregator/reader supports both Atom and RSS feeds
* Publishing podcasts on iTunes requires an RSS feed
* The Atom specification is more well defined and by most accounts easier
  to work with

Given all of that, I think that I'll probably start with Atom, and then
potentially add an RSS feed as well later on.

## Implementing Atom

I started out by reading the [Atom specification], which is actually quite
an easy read. It gives some examples of Atom documents, one of which I
copied over to form the base of my template. From there, I started reading
up on the definitions for the various elements of the specification. There
are top-level metadata attributes that can be specified for a feed, most
of which are fairly straightforward (`author`, `title`, `updated`, etc.).
Feed entries are specified in an `entry` element, which contains information
about the individual entry.

### Unique IDs

One of the components of the Atom specification is that each entry have
a unique ID. This honestly isn't something I planned for when I was
originally coding the site, because none of the content exists in a
database. However, there is an assumption that each blog post's slug
will be unique, since the slugs are used as the path in the URL.

The [atom specification] has this to say about the ID element:

> Its content MUST be an IRI, as defined by [RFC3987].  Note that the
> definition of "IRI" excludes relative references.  Though the IRI
> might use a dereferencable scheme, Atom Processors MUST NOT assume it
> can be dereferenced.
>
> When an Atom Document is relocated, migrated, syndicated,
> republished, exported, or imported, the content of its atom:id
> element MUST NOT change.  Put another way, an atom:id element
> pertains to all instantiations of a particular Atom entry or feed;
> revisions retain the same content in their atom:id elements.  It is
> suggested that the atom:id element be stored along with the
> associated resource.
>
> The content of an atom:id element MUST be created in a way that
> assures uniqueness.

That description, along with their example ID of
`urn:uuid:1225c695-cfb8-4ebb-aaaa-80da344efa6a`, got me going down a bit
of a rabbit hole of trying to figure out how to hash the slug and
represent it as an IRI, but the [w3c validator introduction to atom][w3c atom intro]
makes things a lot less scary:

> Identifies the entry using a universally unique and permanent URI.
> Suggestions on how to make a good id can be found here. Two entries in
> a feed can have the same value for id if they represent the same entry
> at different points in time.

Their example ID is just `<id>http://example.com/blog/1234</id>`. Since
we have unique URIs for our posts, we can just use those. Phew!

### Templates

Atom's a pretty simple format, so we just need two templates, one for the
page and another for entries:

```html
<!-- atom.xml -->

<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">

    <title>Matthew Planchard's Blog</title>
    <link rel="self" href="https://blog.mplanchard.com/atom.xml"/>
    <link href="https://blog.mplanchard.com/"/>
    <updated>{{ updated }}</updated>
    <author>
        <name>Matthew Planchard</name>
    </author>
    <id>https://blog.mplanchard.com/</id>

    {{ entries }}

</feed>
```

```html
<!-- atom-entry.xml -->

<entry>
    <title>{{ title }}</title>
    <link href="{{ link }}"/>
    <id>{{ link }}</id>
    <updated>{{ updated }}</updated>
    <summary>{{ summary }}</summary>
</entry>
```

### The Rust!

With all of the refactoring we did as part of
[adding tag support](/posts/adding-support-for-tags-1.html), this is
pretty easy.

First, we add our page template to our
[`TemplatePageStrings` struct](https://github.com/mplanchard/speedy/blob/ed60e0eb767a60e1371401b0741c2add86d01b08/src/main.rs#L114-L119)
and its [const instance](https://github.com/mplanchard/speedy/blob/ed60e0eb767a60e1371401b0741c2add86d01b08/src/main.rs#L146-L151),
and add a parser to our `PageTemplates` struct:

```rust
struct TemplatePageStrings {
    about: &'static str,
    atom: &'static str,
    generic: &'static str,
    index: &'static str,
    post: &'static str,
}
// ...
const TEMPLATE_STRINGS: TemplateStrings = TemplateStrings {
    // ...
    pages: TemplatePageStrings {
        about: include_str!("../templates/pages/about.html"),
        atom: include_str!("../templates/pages/atom.xml"),
        generic: include_str!("../templates/pages/generic.html"),
        index: include_str!("../templates/pages/index.html"),
        post: include_str!("../templates/pages/post.html"),
    },

    // ...
struct PageTemplates {
    about: liquid::Template,
    atom: liquid::Template,
    generic: liquid::Template,
    index: liquid::Template,
    post: liquid::Template,
}
impl PageTemplates {
    fn new(parser: &liquid::Parser) -> Self {
        let parse = |template_str| parse_template_str(parser, template_str);
        Self {
            about: parse(TEMPLATE_STRINGS.pages.about),
            atom: parse(TEMPLATE_STRINGS.pages.atom),
            generic: parse(TEMPLATE_STRINGS.pages.generic),
            index: parse(TEMPLATE_STRINGS.pages.index),
            post: parse(TEMPLATE_STRINGS.pages.post),
        }
    }
}
```

As a side note, we might eventually do a post on macros to make it easier
to get these templates made :)

From there, we do the same thing for the snippet template in its struct
and instance, although I won't bore you with that.

We need to add two rendering methods to the `Context` struct, one for
atom entries and another for the atom page. The first looks like this:

```rust
impl<'a> Context<'a> {
    // ...
    fn render_atom_entry(&self, post: &Post) -> String {
        let globals = liquid::value::Object::from_iter(vec![
            ("title".into(), to_liquid_val(&post.metadata.title)),
            ("link".into(), to_liquid_val(&post.url)),
            (
                "updated".into(), to_liquid_val(
                    DateTime::<Utc>::from_utc(
                        post.metadata.updated.and_hms(0, 0, 0), Utc
                    ).to_rfc3339())
            ),
            ("summary".into(), to_liquid_val(&post.metadata.summary)),
        ]);
        self.templates
            .snippets
            .atom_entry
            .render(&globals)
            .expect(&format!("failed to reader atom entry for {:?}", post))
    }
    // ...
```

Because I only store the date of the last update, and the atom spec wants
a datetime, I decided just to represent it as the time at midnight UTC,
for entirely arbitrary reasons.

Rendering a _page_ is a little more complicated, at least if we want to
avoid adding multiple iterations through our posts. This is because one
of the things that the spec reuqires is an `<updated>` tag for the feed
as a whole. This is fine, because I store this information, but the posts
are stored in the `Context` struct in order by creation date. So, I need
to iterate over them, get the most recent update date, and also render
them all into `atom-entry` format, ideally all in one loop, and ideally
functionally!

I wound up doing this by bifurcating the result of a `.fold()`:

```rust
    fn render_atom_page(&self) -> String {
        // Get the most recently updated entry and a string of rendered
        // <entry> documents, separated by newlines.
        let (updated, entries) = self.posts.iter().fold(
            (NaiveDate::from_ymd(1, 1, 1), String::new()),
            |(newest_date, entries), post| {
                (
                    if post.metadata.updated > newest_date {
                        post.metadata.updated
                    } else {
                        newest_date
                    },
                    [entries, self.render_atom_entry(post)].join("\n"),
                )
            },
        );
        let globals = liquid::value::Object::from_iter(vec![
            (
                "updated".into(),
                to_liquid_val(Self::updated_datetime_str(&updated)),
            ),
            ("entries".into(), to_liquid_val(entries)),
        ]);
        self.templates
            .pages
            .atom
            .render(&globals)
            .expect("failed to render atom feed")
    }
```

Amazingly, once the compiler was satisfied, this worked on the first try!
I honestly don't think I could have written something like this in Python,
which has been my primary language at work for five years, and gotten it
to work without extensive testing and tweaking.

From there, it's just a matter of adding links to the RSS feed from the
main page!

## Conclusions

The [atom specification] is thorough and easy to follow, although you'll
probably still want some extra resources (like the [W3C atom intro]) for
when you get confused.

The architecture of my site generator is coming along nicely. There's still
a fair bit of what feels like busy work when adding a new template, because
it needs to be added in one way or another in three separate places.
Making this smoother with a macro or something similar might be the focus
of a future piece.

Rust is a great functional language! Sometimes you've got to really think
about how to do something functional, but it's almost always possible.

[Atom Specification]: https://tools.ietf.org/html/rfc4287
[IRI Specification]: https://tools.ietf.org/html/rfc3987
[RSS Specification]: https://cyber.harvard.edu/rss/rss.html
[W3C Atom Intro]: https://validator.w3.org/feed/docs/atom.html
