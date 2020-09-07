title: Selected Reads (2020-09-07)
slug: selected-reads-2020-09-07
created: 2020-09-07
updated: 2020-09-07
tags: rust, facebook, academics, jupyter, chess, philosophy, software, classics, psychology, theater
summary: From fancy Rust libraries and applications for working with the command line to deep dives into the role of Greek tragedy in driving psychological catharsis to meditations on the relationship between chess and philosophy, this selection of reads covers a pretty broad range of topics.


# Selected Reads (2020-09-07)

This is a selection of articles, videos libraries, tweets, and sundry that
I've found interesting over the last week or so.


## Rust Commandline Library

-   URL: <https://github.com/rust-shell-script/rust_cmd_lib>
-   Author: <https://github.com/tao-guo>

This library provides a rust macro that allows you to type in shell
scripts exactly as you would on the commandline, up to and including
piping and `$` interpolation of variables and have them run via the usual
Rust subprocess machinery. I'm not sure whether or not I will often reach
for this, given that I don't do a great deal of shelling out from Rust,
and where I need more than a few lines I feel like I'd generally write
a shell script to call from Rust, rather than bothering with installing
a third-party dependency. However, it's a very cool example of the power
of Rust macros!


## Facebook is Quietly Pressuring its Independent Fact-Checkers to Change their Rulings

-   URL: <https://www.fastcompany.com/90538655/facebook-is-quietly-pressuring-its-independent-fact-checkers-to-change-their-rulings>
-   Author: Alex Pasternack

An article detailing some of the cases where what seems like executive
pressure within Facebook has exerted influence over the determinations of
various fact-checking teams. I think the article made a clear case why the
reliance on Facebook's self-policing its own platform's role in spreading
misinformation can't work: the minute that doing the right thing is
misaligned with the bottom line, the right thing is overruled.


## As Above, so Below: Bare Metal Rust Generics

-   URL
    -   Part one: <https://www.ecorax.net/as-above-so-below-1/>
    -   Part two: <https://www.ecorax.net/as-above-so-below-2/>
-   Author: [PabloMansanet](https://github.com/PabloMansanet)

A great two-parter taking a deep dive into writing generic functions in Rust,
presented in a "bare-metal" context of writing a storage driver. It does a
good job of presenting the material in a way that requires very minimal
preexisting knowledge of either Rust or SSD drivers, while giving the reader
an excellent opportunity to learn about both.


## The Corner That State Universities Have Backed Themselves Into

-   URL: <https://www.theatlantic.com/ideas/archive/2020/08/why-state-universities-have-no-other-choice-but-to-reopen/615565/>
-   Author: [Bret Devereux](https://acoup.blog/)
    
    A long-form version of the twitter thread I linked to in the previous
    iteration of this selected reads series, Bret Devereux discusses the
    sad state of university funding and how it's led to the Catch-22 they
    find themselves in with regards to bringing students back on campus
    during the pandemic.


## One Year of NuShell

-   URL: <https://www.nushell.sh/blog/2020/08/23/year_of_nushell.html>
-   Author: [The NuShell Team](https://twitter.com/nu_shell)

This look back at one year since NuShell's official release was a really
interesting read. I haven't used NuShell, but it's great to see that people
are experimenting with ways to improve the shell experience, which has largely
remained unchanged over the past however many years. I have been meaning to
give NuShell a shot since I first saw their public release, and it seems like
this is as a good a time as any.


## I Don't Like Notebooks

-   URL: <https://www.youtube.com/watch?v=7jiPeIFXb6U>
-   Author: [Joel Grus](https://joelgrus.com/)

A coworker linked me to this video as a result of a conversation we were having
about Jupyter notebooks. This is a great talk, and a brave one to give at
Jupyter Con! I find myself largely in the same boat as the author, although I've
never been able to articulate it quite so well. Essentially, I've always really
wanted to get into and like notebooks, but the experience of writing code in a
proper editor has always felt so much better that I've never really been able to
find any use for them. This talk helped encapsulated a lot of the things that
feel like they're lacking.


## Rust 1.46.0

-   URL: <https://blog.rust-lang.org/2020/08/27/Rust-1.46.0.html>

There were a few of exciting things about this version of Rust, but by far the
biggest improvement is the expansion of what's allowed in `const fn` functions.
`const` functions allow you to define parameterized code that can be run at
compile time or at runtime, and this update significantly expands what's allowed
within a `const fn`. I'm currently working on a chess engine as a learning
project, and I've already been able to use these improvements to help me easily
generate constant bitboards representing a variety of positions at compile-time,
which can then be used at runtime to evaluate moves.


## Pure and Applied Chess

-   URL: <https://theelectricagora.com/2020/08/28/pure-and-applied-chess/>
-   Author: [David L. Duffy](https://genepi.qimr.edu.au/Staff/davidD/)

This is a broad, discursive, indulgent essay exploring thoughts about the game
of chess its relationship with philosophy. Pulling in references from
Wittgenstein, Daniel Dennett, and others, this is exactly the sort of
philosophical wankery I most dearly love.


## The door problem

-   URL: <http://www.lizengland.com/blog/2014/04/the-door-problem/>
-   Author: [Liz England](http://www.lizengland.com/blog/)

I really enjoyed this brief exploration of the exploding complexity of even
the simplest detail in a complex system. The author considers the seemingly
trivial example of "let's add doors to our game," and follows up an amazing
array of complications and decisions that arise. The "door problem" is also
used to illustrate the role of different people involved in the production
of games.


## Can Greek Tragedy Get Us through the Pandemic?

-   URL: <https://www.newyorker.com/culture/culture-desk/can-greek-tragedy-get-us-through-the-pandemic>
-   Author: [Elif Batuman](https://www.newyorker.com/contributors/elif-batuman)

One of the best things I read all week (and summarily [shit on](https://news.ycombinator.com/item?id=24380616) by HackerNews
commenters, with the all of their characteristic performative intellectualism),
this article begins with a focus on a theater group call [Theater of War
Productions](https://theaterofwar.com/about), which began some time ago to use the medium of Greek tragedy to
drive catharsis and discussion of taboo subjects among veterans and other
groups that have experienced signifcant past trauma. The group has now adapted
the tragedy of Oedipus to address the COVID pandemic and our associated
collective trauma, performing the play over Zoom to a "crowd" of about
fifteen thousand. The article covers an impressive breadth of content,
ranging from the history of theater in ancient Greece to the development
of the idea of catharsis in Freudian psychoanalysis as a means of processing
repressed trauma to the personal history of the founder of Theater of War,
whose father was a psychologist and who studied classics at University.
Despite the broad range of topics, the author deftly ties them all up into
a neatly reported thesis that perhaps the role of classical tragedy and other
such serious theater may have always been somewhat more pragmatic than
is usually realized, and that it can continue to have the psychologically
healing effect for which it was originally intended on modern audiences who
have dealt with their own traumas.

