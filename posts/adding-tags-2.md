title: Adding Support for Tags: Part 2
slug: adding-support-for-tags-2
created: 2019-07-02
updated: 2019-07-02
tags: rust, blog, programming
summary: actually parsing and using tags

# Adding Tags to my Static Site: Part 2

For Part 1, go [here](/posts/adding-support-for-tags-1.html)

## Getting a mapping of tags to Posts

So, after part one, we have a nice `Metadata` struct for each post,
which contains a `Vec<String>` of tags, which is constructed from the
first few lines of each of our markdown files.

For reference, here is our metadata struct:

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

and the `Post` struct in which we might find one:

```rust
#[derive(Debug)]
struct Post {
    metadata: Metadata,
    content: String,
}
```

We get our `Post` instances out of the markdown files using this relatively
ugly but functional bit of code:

```rust
let mut posts = fs::read_dir("posts")
    .expect("Couldn't read posts dir")
    .map(|r| r.expect(&"couldn't read dir entry"))
    .filter(|de| (&de.file_type().unwrap()).is_file())
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
```

So, at this point, we've got every post we've ever written. For generating
the lists of posts on the index and "Posts" pages, we sort the vector by
create date:

```rust
posts.sort_by(|a, b| a.metadata.created.cmp(&b.metadata.created).reverse());
```

For tags, though, we'll want a map of tags to posts. We should use post
references, so that we don't need to clone the posts in memory.

We can start with a simple, in-place construction of the `HashMap`:

```rust
// generate map of tags to posts
let mut tags_to_posts = HashMap::new();
posts.iter().for_each(|post| {
    post.metadata.tags.iter().for_each(|tag| {
        tags_to_posts
            .entry(tag)
            .and_modify(|post_vec: &mut Vec<&Post>| post_vec.push(post))
            .or_insert(vec![post]);
    });
});
```

A few of things to note in the above implementation:

* the [Entry API](https://doc.rust-lang.org/std/collections/hash_m) in the builtin
  HashMap collection is super cool
* `posts` is `Vec<Post>`. When we call `.iter()`, we get `Iterable<Item: &Post>`,
  i.e. an iterable over references to `Post` instances. We grab the `Metadata`
  instance from each post and call `.iter()` on its `.tags` attribute. Since
  `.tags` is `Vec<String>`, we get `Iterable<Item: &String>`. So, within our
  closures, `post` and `tag` are both references, making the type of our
  map `HashMap<&String, Vec<&Post>>`, which is exactly what we wanted.

Can we extract this into a function? Sure, but we're going to need to
specify lifetimes:

```rust
fn tag_map<'a, T>(posts: T) -> HashMap<&'a String, Vec<&'a Post>>
    where T: IntoIterator<Item = &'a Post>
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
```

Here, we've said, "give me any `posts` that can be turned into an `Iterator`
over references to `Post` instances," and I'll return a `HashMap` of
references to `String` instances pointing to vectors of references to
`Post` instances." Because we have multiple references (and therefore
multiple possible lifetimes), we must specify to the compiler that the
`String` and `Post` references in the `HashMap` should live as long as
the `Post` references on our incoming iterator, which is to say that
it is not safe to drop the `Post`s and continue using the `HashMap`!

We can then update our call site to just look like this:

```rust
let tags_to_posts = tag_map(&posts);
```

One of the nice things about the [`IntoIterator` implementations on `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-IntoIterator)
is that if you have `Vec<T>`, calling `.into_iter()` gives you an `Iterator`
over `T`. Calling `.into_iter()` on a _reference_ to `Vec<T>` gives you
an `Iterator` over references to `T`, which is to say `&T`. Finally,
calling `.into_iter()` on a mutable reference to `Vec<T>` gives you
an `Iterator` over `&mut T`, mutable references to `T`. All of this means
that we can satisfy our trait bound of `IntoIterator<Item = &Post>` by
passing in a _reference_ to our `Vec<Post>`, without any further conversion
required.

## Adding HTML for the tag overview page

Now that we have a tag -> post mapping, we can write some HTML. Our
tags overview will be a combination of several snippets. The page will contain
the usual header and footer. The content will be a listing of tags, with
each associated post linked beneath the tag. We can reuse the simple
`posts-post.html` snippet that we used for listing posts on the posts
overview and index pages. It just contains a single `<li>` element,
with a bolded link whose text is the post title, followed by the post
summary. We can compose this into a tag snippet that looks something like:

```html
<h4>{{ tag }}</h4>
{{ posts }}
```

if we were using the full power of the templating engine, we could do
some looping logic in here, but I've been having fun so far with building
everything up in application code (we'll see how long that lasts).

For now, there's no reason the tag overview can't just be an instance
of our `generic.html` template, which looks like:

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

I think now we have everything we need to build an initial implementation
of the tags page.

