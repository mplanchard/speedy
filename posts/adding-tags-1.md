title: Adding Support for Tags: Part 1
slug: adding-support-for-tags-1
created: 2019-06-30
updated: 2019-07-01
tags: rust, blog, programming
summary: refactoring header parsing in preparation for tag support

# Adding Tags to my Static Site: Part 1

For part 2, go [here](/posts/adding-support-for-tags-2.html)

<aside>
N.B. I initially intended to do this all in one go, but the first post
wound up being fairly long, so I'm going to split it out into sections.
This first part is on refactoring tag parsing.
</aside>

A main architectural consideration when creating this site was that I
wanted to support all of the parts of a dynamic website I liked,
specifically tags, post updates, and ordering posts by arbitrary
criteria (generally date).

I played with a lot of ideas, but what I wound up with was:

* A [directory](https://github.com/mplanchard/speedy/tree/master/posts) of markddown
  files, each of which becomes a single blog post
* A predefined header format (colon-separated keys and values, headers
  delineated by newlines) expected at the top of each markdown file

The header allows me to track whatever arbitrary metadata I'd like to track
about a post. It also gives me the ability to go in and add or update things
later (for example, while I originally only had a single date header, I
decided to add created/updated dates).

Tags have always been a part of my header "specification," because it was
always my intention to enable them. I just didn't get around to it in the
initial implementation, because I was in a "just get things working" mode.
There's no real error handling, things are done roughly, there's plenty
of duplication. In fact, nothing in this project is perfect or
perfectly architected, but as I continue to work on it, I am beginning to
refactor things a bit. As such, these posts will be as much about how to
do a given thing as they will be about doing early-stage rewrites of a
project. This is a critical skill in software development, in my
experience, because the first version of something is never the best,
especially when it was done with speed as a primary concern.

Anyway, on to the action!

## Refactoring Header Parsing

Each post's metadata is stored in a simple `struct`:

```rust
#[derive(Debug)]
struct Metadata {
    title: String,
    slug: String,
    created: NaiveDate,
    updated: NaiveDate,
    tags: Vec<String>,
    summary: String,
}
```

The metadata is a part of another `struct` that represents a post during
static site generation:

```rust
#[derive(Debug)]
struct Post {
    metadata: Metadata,
    content: String,
}
```

Currently, the generator is quite naive. It just rolls through the
`posts` directory, reads the content of each one, splits out the header
lines, and then makes a `Post` from the `Metadata` gleaned from the
headers and the remainder of the content.

The function that parses the metadata is currently comically straightforward.
It simply collects an iterable of `&str` into a `Vec`, sorts it, and
then relies on the fact that the items are always going to be in the
same alphabetical order to grab the values out from the lines:

```rust
fn parse_metadata<'a, L: Iterator<Item = &'a str>>(lines: L) -> Metadata {
    let mut lines = lines.collect::<Vec<&str>>();
    lines.sort();
    let created_date_str = line_contents(lines[0]);
    let slug = line_contents(lines[1]);
    let summary = line_contents(lines[2]);
    let tags_str = line_contents(lines[3]);
    let title = line_contents(lines[4]);
    let updated_date_str = line_contents(lines[5]);

    let updated = NaiveDate::parse_from_str(
        &updated_date_str, "%Y-%m-%d"
    ).expect("invalid date");
    let created = NaiveDate::parse_from_str(
        &created_date_str, "%Y-%m-%d"
    ).expect("invalid date");
    let tags = tags_str
        .trim()
        .split(",")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    Metadata {
        created,
        slug,
        summary,
        tags,
        title,
        updated,
    }
}
```

(`line_contents` just splits the line, discards the key, and returns the
trimmed string).

Anyway, this is a prime spot for a refactor, it seems to me! We've got
what is essentially a constructor method, but it's not directly associated
with its `struct`. The question is what we should pass in to our constructor.
Options include:

* The raw content of the markdown
* A single String or slice of the _n_ header lines
* An iterable of header lines
* A vector of header lines

It seems to me the best separation of concerns is for the the metadata
struct to be constructable from either the raw `.md` content _or_ just
the header lines. This allows more flexibility in the call, freeing the
callers from needing to implement logic to split the lines or otherwise
collect them in a way that is easier for the constructor to digest.

With this, we'll probably also want to adjust our process for getting header
content to work by looking for header keys, rather than sorting and getting
things in alphabetical order.

Let's start by storing an [associated constant](https://doc.rust-lang.org/stable/edition-guide/rust-2018/trait-system/associated-constants.html)
with the `Metadata` referencing how many lines we expect the header to be:

```rust
impl Metadata {
    const NUM_HEADER_LNS: u8 = 6;
}
```

We can then delete the global `POST_HEADER_LEN` constant we were using, and
let the compiler guide us in replacing it with the new constant.

Now we want to add a constructor that takes something we can reference as
a `&str`:

```rust
impl Metadata {
    const NUM_HEADER_LNS: u8 = 6;

    fn new<S: AsRef<str>>(header_text: S) -> Self {
    }
}
```

Getting an iterator over our header lines is easy:

```rust
fn new<S: AsRef<str>>(header_text: S) -> Self {
    let lines = header_text.as_ref().lines().take(Self::NUM_HEADER_LNS.into());
}
```

<aside>
N.B. we call <code>.into()</code> on our constant, because
<code>.take()</code> expects <code>u16</code>, and we stored our value as
<code>u8</code>.
</aside>

It's probably not as efficient as just grabbing things based on their
alphabetical order like we did earlier, but we can now take our iterator
and convert it into a `HashMap`:

```rust
fn new<S: AsRef<str>>(header_text: S) -> Self {
    let lines = header_text.as_ref().lines().take(Self::NUM_HEADER_LNS.into());

    let ln_tuples = lines.map(
        |ln| {
            let parts = ln.splitn(2, ":").map(|i| i.trim());
            (
                parts.next().expect(&format!("bad header: {:?}", ln)),
                parts.next().expect(&format!("bad header: {:?}", ln))
            )
        }
    );
    let headers: HashMap<&str, &str> = ln_tuples.collect();
}
```

May as well go ahead and pull everything we've done so far out into
its own function:

```rust
fn new<S: AsRef<str>>(header_text: S) -> Self {
    let headers = Self::header_map(&header_text);
}

// We need the lifetimes to tell the interpreter that our references in
// our HashMap will only live as long as the `header_text`
fn header_map<'a, S: AsRef<str>>(header_text: &'a S) -> HashMap<&'a str, &'a str> {
    let lines = header_text.as_ref().lines().take(Self::NUM_HEADER_LNS.into());
    let ln_tuples = lines.map(
        |ln| {
            let mut parts = ln.splitn(2, ":").map(|i| i.trim());
            (
                parts.next().expect(&format!("bad header: {:?}", ln)),
                parts.next().expect(&format!("bad header: {:?}", ln))
            )
        }
    );
    // Note we don't have to specify the type on `.collect()` here,
    // because the compiler is smart enough to collect it into our
    // specified return type
    ln_tuples.collect()
}
```

The only of our struct attributes that aren't `String`s are the dates
and the tags, so let's handle parsing those individually. First, tags:

```rust
fn tags<S: AsRef<str>>(tags: S) -> Vec<String> {
    tags.as_ref()
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.to_owned())
        .collect()
}
```

Then, dates:

```rust
fn date<S: AsRef<str>>(date: S) -> NaiveDate {
    NaiveDate::parse_from_str(
        date.as_ref(), "%Y-%m-%d"
    ).expect(&format!("invalid date: {:?}", date.as_ref()))
}
```

actually, let's get those magic strings out of there and make them
associated constants, too:

```rust
impl Metadata {
    const NUM_HEADER_LNS: u8 = 6;
    const TAG_DELIMITER: &'static str = ",";
    const DATE_FMT: &'static str = "%Y-%m-%d";

    fn new<S: AsRef<str>>(header_text: S) -> Self { ... }
    fn header_map<'a, S: AsRef<str>>(header_text: &'a S) -> HashMap<&'a str, &'a str> { ... }

    fn tags<S: AsRef<str>>(tags: S) -> Vec<String> {
        tags.as_ref()
            .split(Self::TAG_DELIMITER)
            .map(|s| s.trim())
            .map(|s| s.to_owned())
            .collect()
    }

    fn date<S: AsRef<str>>(date: S) -> NaiveDate {
        NaiveDate::parse_from_str(
            date.as_ref(), Self::DATE_FMT
        ).expect(&format!("invalid date: {:?}", date.as_ref()))
    }
}
```

A generic function to grab a header out of our map:

```rust
fn header_value<'a>(headers: &'a HashMap<&str, &str>, key: &str) -> &'a str {
    headers.get(key).expect(&format!("No {:?} header", key))
}
```

Now we can finally flesh out or `new()` method:

```rust
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
```

All of this allows us to replace this, in our `generate()` function:

```rust
let md_txt =
    fs::read_to_string(md.path()).expect(&format!("couldn't read md: {:?}", md));
let metadata = parse_metadata(md_txt.lines().take(Metadata::NUM_HEADER_LNS.into()));
```

with this:

```rust
let md_txt =
    fs::read_to_string(md.path()).expect(&format!("couldn't read md: {:?}", md));
let metadata = Metadata::new(&md_txt);
```

And, `cargo run generate` still runs quite quickly.

Next time, I'm going to work on automatically adding a page with a list of
posts for each tag, a `tags.html` page with a listing of all tags, and the
ability for any tag reference to easily point to the tag's individual
page. Maybe that won't all be in one part, but we'll see.
