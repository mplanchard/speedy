title: Selected Reads (2020-09-27)
slug: selected-reads-2020-09-27
created: 2020-09-27
updated: 2020-09-27
tags: selected-reads, rust, politics, engineering, open-source, academics
summary: It&rsquo;s been a minute since I put out a list of selected reads, which means we&rsquo;ve got quite a few today. Lots of Rust, as usual, but also some politics, an article on VSCode, and another Bret Devereux post on academics.


# Selected Reads (2020-09-27)

Here is a selection of articles and other media that I&rsquo;ve found interesting
since the last installment of this series. I&rsquo;ve started providing archive
links in addition to the original URLs, in case in the future the original
sites change URLs, go down, etc. Please prefer the original links when
possible.


## Alien Signals

-   URL: <https://www.robinsloan.com/notes/alien-signals/>
-   Author: [Robin Sloan](https://www.robinsloan.com/about/)
-   Archive URL: <https://web.archive.org/web/20200908002533/https://www.robinsloan.com/notes/alien-signals/>

This is a pleasantly nostalgic look back at his childhood experience
of listening to nighttime radio and waiting for zines, and the feeling
of mystery and suspense those things engendered.


## Column Names as Contracts

-   URL: <https://emilyriederer.netlify.app/post/column-name-contracts/>
-   Author: [Emily Riederer](https://emilyriederer.netlify.app/)
-   Archive URL: <https://web.archive.org/web/20200908000858/https://emilyriederer.netlify.app/post/column-name-contracts/>

This article goes in depth into the utility of standardizing column
names in a data schema, to the point where it&rsquo;s possible to treat the
names as contracts, even building automated tooling around them. In
addition to being interesting in the specific, it makes a good general
point about the value of standardization in naming.


## Peeking Inside a Rust Enum

-   URL: <https://fasterthanli.me/articles/peeking-inside-a-rust-enum>
-   Author: [Amos](https://fasterthanli.me/about)
-   Archive URL: <https://web.archive.org/web/20200924142844/https://fasterthanli.me/articles/peeking-inside-a-rust-enum>

An extremely in-depth article about Rust enums, with deep comparisons
to C enums, introspection of their layout in memory, and some of the
benefits of Rust enums.


## My FOSS Story

-   URL: <https://blog.burntsushi.net/foss/>
-   Author: [Andrew Gallant](https://blog.burntsushi.net/about/)
-   Archive URL: <https://web.archive.org/web/20200924164611/https://blog.burntsushi.net/foss/>

The author of this post is a prolific open source contributor, having
written ripgrep, the most popular Rust regular expression engine, the
most popular Rust CSV parser, the main Golang TOML parser, a property
testing library for Rust, an X window manager in Golang, and so on.
This article chronicles his experience contributing to open source,
maintaining projects, and so on. It&rsquo;s a great, measured look at the
experience from the perspective of a well respected member of the
community. This article was published way back in January, but I think
an update put it back into my RSS feed, and I figured there&rsquo;s no harm
in re-highlighting it.


## The Overwhelming Racism of COVID Coverage

-   URL: <https://medium.com/indica/the-overwhelming-racism-of-covid-coverage-78e37e4ce6e8>
-   Author: [Indi Samarajiva](https://medium.com/@indica)
-   Archive URL: Medium 500s for me with archive.org URLs :(

This post calls out the western media for in large part completely failing
to report on the relative successes with COVID management seen in a
number of non-white, non-wealthy, non-western countries, such as Thailand,
Vietnam, Rwanda, and Ghana, making the rather difficult to refute case
that the media tends to present the success of those countries, when it&rsquo;s
mentioned at all, as being due to luck or genetics, failing to consider
the details of these countries public health responses, which have often
been comprehensive and, apparently, quite effective.


## A Concurrency Cost Hierarchy

-   URL: <https://travisdowns.github.io/blog/2020/07/06/concurrency-costs.html>
-   Author: [Travis Downs](https://travisdowns.github.io/about/)
-   Archive URL: <https://web.archive.org/web/20200923153019/https://travisdowns.github.io/blog/2020/07/06/concurrency-costs.html>

This is a fantastic article going into the depths of relative performance
of various low-level synchronization primitives in situations of low,
medium, and high thread saturation. An absolute must-read and must-bookmark
for anyone needing to choose between atomics, mutexes, or other
synchronization primitives in their code.


## An Introduction to Data-oriented Design with Rust

-   URL: <https://jamesmcm.github.io/blog/2020/07/25/intro-dod/>
-   Author: [James McMurray](https://jamesmcm.github.io/about/#en)
-   Archive URL: <https://web.archive.org/web/20200920023302/https://jamesmcm.github.io/blog/2020/07/25/intro-dod/>

A nice primer on some of the ways that you can optimize your data structures
to enable faster processing, fewer cache misses, etc. This kind of
data-oriented design has the potential to significantly improve the
performance of processing large amounts of data or smaller amounts of
data in hot paths, and seems like a really useful set of skills to be
aware of.


## White House-CDC tensions explode as Trump contradicts its leadership

-   URL: <https://arstechnica.com/science/2020/09/white-house-cdc-tensions-explode-as-trump-contradicts-its-leadership/>
-   Author: [John Timmer](https://arstechnica.com/author/john-timmer/)
-   Archive URL: <https://web.archive.org/web/20200927201104/https://arstechnica.com/science/2020/09/white-house-cdc-tensions-explode-as-trump-contradicts-its-leadership/>

You&rsquo;d think this sort of thing would stop surprising me, but the degree
to which a significant portion of the country seem to be *just fine* with
the White House intercepting basic science and public health messaging
and spinning it to its own ends continue to floor me, especially when it&rsquo;s
as flagrant as this.


## The Era of Visual Studio Code

-   URL: <https://blog.robenkleene.com/2020/09/21/the-era-of-visual-studio-code/>
-   Author: [Roben Kleene](https://blog.robenkleene.com/about/)
-   Archive URL: <https://web.archive.org/web/20200927123546/https://blog.robenkleene.com/2020/09/21/the-era-of-visual-studio-code/>

This article makes the case that VSCode is ushering in a new era of text
editor dominance, brought on by having perfected the extension model and
having the benefit of continuous open source development by Microsoft.
It&rsquo;s a little breathlessly uncritical for my taste, but I do think it&rsquo;s
worth considering whether VSCode may be the IDE/editor outside of vim
and emacs to potentially achieve long-term success.


## The Purpose of College

-   URL: <https://acoup.blog/2020/08/28/fireside-friday-august-28-2020/>
-   Author: [Bret Devereux](https://acoup.blog/)
-   Archive URL: <https://web.archive.org/web/20200828191545/https://acoup.blog/2020/08/28/fireside-friday-august-28-2020/>

This &ldquo;Fireside Friday&rdquo; post by Bret Devereux looks at the value proposition
of college in modern American society, particularly what it is about
a college education that remains a strong signal for employers. Like
everything he writes, this is perceptive, nuanced, and interesting.

