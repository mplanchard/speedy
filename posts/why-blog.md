title: Why Bother?
slug: why-bother
created: 2019-06-29
updated: 2019-06-29
tags: general
summary: Why bother having a custom blog at all?

# Why Bother (Maintaining a Custom Blog)

So, other than posts that I'm importing from other sources, e.g. my
[medium profile](https://medium.com/@mplanchard), this is my first actual
post written in the context of and entirely for this blog. I think that
introspection of my motivations for doing things is always key to
doing them well, as well as helping me to maintain my motivation for
longer. I am definitely susceptible to starting projects and then letting
them lie, which I'd prefer not to do here.

I'd like to dive in a bit to the negatives of doing all of the work to
make this website a reality, and then explore what I view to be the
positives. Hopefully by the time I finish, I'll better understand how
I got here.

## Why Doing this is a Bad Idea

### Discoverability

I'm going to be honest. I don't know beans about search engine optimization
(SEO). When I first started building websites back in the early aughts,
none of my "clients" (i.e. friends of my parents') knew enough to request
it. Since then, all of my web development has been at companies where
the marketing people were responsible for all of those details.

One of the things that you get for free on a site like medium or [dev.to](https://dev.to)
is inbound traffic through recommendations and good SEO. This is a huge
plus if you care about being discovered.

Luckily, I'm not doing this to be discovered! I'm doing it to have fun,
to play around with technologies, and to have a place apart from any
major company where I can express things just as I like.

### Effort

Looking back at the git commits on the [repo](https://github.com/mplanchard/speedy)
where I've got all this stuff stashed, my first commit was on March 14
of this year (it's currently the end of June). Of course, I wasn't
actively working that whole time, and a lot of what I did was try out
ideas that I thought would be interesting as a way of learning more
Rust, but still. Four months of effort is a lot compared to throwing
some thoughts into a WSYWIG and pressing "publish."

### Complication

The actual process of writing one of these posts is simple. I write
a Markdown file, run `cargo generate`, and then click a button in VSCode
to publish the static files wherever I'm hosting the website (currently
Azure). However, _getting_ to the point where that simplicity is available
took a lot of time and effort. Essentially what I have now is a custom
templating engine written in Rust that generates static HTML files
from the markdown sources. There are obviously a lot of moving pieces
there, and any problems are up to me to fix. It's up to me to make sure
that SSL is configured, that DNS records are correct, and a bunch of
other minor details that are removed from you if you're using someone
to host your blog.

## Why I Love Doing This Anyway

### Customization

There are some things I really care about as a consumer of online content,
and maintaining my own site allows me to ensure that I can provide those
things to anyone reading this site, too. Such concerns include:

* Accessibility through [semantic HTML](https://developer.mozilla.org/en-US/docs/Web/Guide/HTML/HTML5#Semantics)
* Fast load times and minimal JavaScript
* Absolutely no tracking, data collection, or cookies
* A clean and simple aesthetic
* Commitment to open source

By maintaining my own site, I can be sure that 100% of those things are
provided 100% of the time. No one reading my content will be subjected
to popups, cookie notifications, or poorly optimized SPAs that take
upwards of five seconds to load. People using screen readers should
have no trouble consuming the content here.

I also think that the interface on popular blogging platforms, particularly
Medium, is not well-suited to programming-focused content. I wanted to
be able to quickly toss in a code block and have it be properly formatted,
just like on GitHub, e.g.:

```rust
/// This Rust code is properly formatted! And pretty!
fn main() {
    println!("What fools these mortals be!")
}
```

### Fun

This has been and continues to be a really great learning experience. I
don't get to play around with HTML and CSS much at work, and my current
employer runs a Python/JS/Go stack, making my learning Rust an entirely
personal endeavor. This site has been a great opportunity to make something
real and nontrivial.

It's also great knowing that, as I go forward, I can add whatever
functionality I like, experiment with any technologies that interest me,
and be able to do it in my own time.

### Long-term Suitability

One of my issues in the past has been a feeling that I always wanted to
find a better place to put my content. I suspected I would eventually
end up doing something like this, which largely prevented me from
committing completely to any given platform. That is not a problem
here. All of my content is in  markdown files and tracked in GitHub.
I can easily evolve the technical aspects of this site, or even move
all of my content elsewhere. This gives me a sense of freedom, knowing
that anything I write here is as permanent and specific to my own tastes
as is humanly possible.

### Prestige, Pride, &c.

This is the sneaky one. I think many people don't like to admit to
themselves when they're doing things to make themselves look better or
to feel like they're superior. And indeed, I think that if that is the
primary motivation for anything, the effort is probably doomed to failure.
That being said, _ignoring_ it when it is a factor makes it harder to
know when that sneaky motivation is causing you to act in ways you might
not consciously want. So, obviously, with this project, there is some
degree of:

* Pride (I made it, I think it's nice, look at me)
* Status-seeking (it serves as a proxy indicator for my job-related
  abilities)
* Holier-than-thou (clearly, _my_ site does things the right way)

So, the big goal here is to recognize that these feelings are present,
and to never let them be the main drivers of my actions. As long as
I'm more focused on having fun and making something good for goodness'
sake, I think it won't be a problem.

## Wrap

I imagine that this will become the main place that I go to get thoughts
out of my head and "onto paper," as it were. I'll continue to have a good
time exploring new ideas and technologies in order to put those thoughts
out on the web. I hope that anyone who finds their way here finds some
enjoyment from it. And that's all that really matters!
