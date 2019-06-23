title: First Post
slug: first-post
date: 2019-06-22
tags: about
summary: just an initial post to test the generator

# Welcome to the Blog

This post was created as part of the development process, largely to
test that things were working the way I expected.

The main things I needed to test were:

* markdown rendering into HTML (thus the variety of content types here)
* parsing of header information re: title, date, and tags
* population of introductory `posts` page

I also needed to verify that code formatting is working as intended
when specifying a language, e.g.:

```rust
fn main() {
    println!("Hello, world!");
}
```

The upshot is that, for each markdown file in the `posts` directory,
sibling to the `src` directory, we should generate an individual HTML
file in the `posts` directory using the `post` template, filling in the
HTML generated from the markdown file for the `{{ content }}` block.

We should also be adding each of the posts to the RSS feed.
