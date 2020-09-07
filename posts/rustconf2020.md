title: RustConf 2020
slug: rustconf-2020
created: 2020-09-07
updated: 2020-09-07
tags: rust, RustConf2020
summary: Rust has been the world's most-loved programming language for five years running. That's a remarkable feat, especially for a language that, particularly back in 2015, was barely even a blip on the list of most-used languages. That fact that so many people who used Rust wanted to keep working with it was one of the main things that made me want to try it out, leading it to become my most-loved programming language too. This was my first year attending RustConf, albeit virtually. Thanks to the conference, for the first time I feel like I really understand what has led to Rust being not only a great language, but a lovable one. I came away from the conference more bullish than ever that Rust adoption will continue to grow in every space where anyone chances to use it. In this post, I'll talk about some of the things that really stood out to me as highlighting what makes Rust special, along with some of the reasons why I think Rust is poised to grow substantially even outside of its originally-intended niche of systems programming.


# RustConf2020: What Makes a Language Great

Rust has been the world's [most-loved programming language for five years running](https://insights.stackoverflow.com/survey/2020#most-loved-dreaded-and-wanted).
That's a remarkable feat, especially for a language that, particularly back in
2015, was barely even a blip on the list of most-used languages. That fact that
so many people who used Rust wanted to keep working with it was one of the main
things that made me want to try it out, leading it to become *my* most-loved
programming language too.

This was my first year attending RustConf, albeit virtually. Thanks to the
conference, for the first time I feel like I really understand what has
led to Rust being not only a great language, but a lovable one. I came away from
the conference more bullish than ever that Rust adoption will continue to grow
in every space where anyone chances to use it.

In this post, I'll talk about some of the things that really stood out to me as
highlighting what makes Rust special, along with some of the reasons why I think
Rust is poised to grow substantially even outside of its originally-intended
niche of systems programming.

As an aside, thanks to my employer, [Bestow](https://bestow.com/), for sponsoring my registration
through our yearly "Bestow Scholarship" education budget.


## Crafting a Lovable Language

The [Opening Keynote](https://www.youtube.com/watch?v=IwPRu5FhfIQ), presented by members of the Rust core team, focused on
the history and growth of Rust, reflecting on how the language's guiding
principles have shaped the priority-setting and decision-making necessary for
its evolution. To me, it seems clear that it is these principles and a clear
alignment around them that have led to Rust's being the language it is today,
rather than the particular technical excellence of its founders and core team.
Of course, the core team is brilliant, but most languages the get any traction
come from excellent programmers. Even so, not every language is Rust.


### The Value of Values

Growth is hard. Whether we're talking about a company, a framework, or a
programming language, growth often brings with it a whole new host of
problems, one of the most difficult of which tends to be how to manage
priorities in the fact of increased demands. Many projects find in a position
of needing to simultaneously build a framework for decision-making from
scratch while also dealing with problems of scale. Often, this is a recipe
for disaster, and it seems like one of the reasons why Rust's growth has
been relatively smooth has been the presence of an in-baked value system
from the beginning.

So what are Rust's values? They are probably most succinctly expressed
in the language's motto, which is prominently featured on the homepage:

> A language empowering everyone to build reliable and efficient software.

The core team spent a fair amount of time in the keynote diving into this
motto, and in particular the "empowering everyone" part of it. It was
specifically noted that "write good software isn't a value." Software is
written by people, for people, and it's core to the values of Rust to
focus on the empowerment both of software authors (including the authors
of the language itself) and software users. It's clear how reliable and
efficient software benefits users, but what I found particularly interesting
are the priorities that the Rust project has set as a result of wanting
to help software authors:

-   Eliminate things that don't need to be hard
-   Educate people on things that *are* hard
-   Provide access to spaces and power in the project

Let's look at how each of these focuses had led to some of the unique
properties of Rust that set it apart from other languages, and how they
will continue to guide Rust's evolution going forward.

1.  Eliminate Things that Don't Need to be Hard

    Rust has a very strong commitment to backwards compatibility, and this
    has been wonderful in practice (updating an old nightly-only Rust project
    to support standard once the 2018 edition was released was a project of
    less than an hour's work). The thoughtful technical solutions around
    maintaining this compatibility (the edition system, the ability to shop
    features in nightly, etc.) make users' lives easier, while *also* allowing the
    language to introduce major new syntax, rules, and other changes in a
    non-disruptive way. Some examples of this so far have been the reworked
    module system, which became default in the 2018 edition, the \`?\` syntax
    to replace the \`try!\` macro, and improved lifetime elision. All of these
    significantly changed the way the language was written, but they were all
    opt-in, and code using the old patterns continues to compile without complaint.
    
    Another much-loved aspect of Rust is its build system, Cargo. Cargo provides
    not only access to typical \`make\` functionality, but also to the entire
    crates.io ecosystem, making it remarkably easy to share code, particularly
    for a "systems" language. It also provides inbuilt cross-compilation to a
    variety of targets, easy extensibility, a test runner, and a benchmark runner.
    Really, more than almost anything else, Cargo makes it easy when you're
    first getting started with Rust to be able to immediately focus on
    *writing code*.
    
    Currently, the level of growth that Rust has experienced has left the core
    team and other contributors to the language itself a bit stretched, so a major
    focus going forward will be make it easier to get started using Rust to write
    Rust. The solidification of the 3-year release cadence of editions is a part
    of this, designed to reduce stress around getting features in "on a deadline"
    because of a lack of certainty as to when they'll be included if they don't
    make it in. The core team is also looking to improve the RFC process, to open
    up their agenda, and to establish a governance working group. Finally,
    the nascent [Rust Foundation](https://blog.rust-lang.org/2020/08/18/laying-the-foundation-for-rusts-future.html) aims to make it easy for the Rust project to manage
    the contracts and money necessary for hosting documentation, CI, outreach, etc.
    
    For language users, there is continuing investment into [rust-analyzer](https://rust-analyzer.github.io/), a
    state-of-the-art language server for improved IDE support, improvements to
    \`clippy\`, including an autofix feature, and of course continuing improvements
    to language ergonomics, including a continued focus on async.

2.  Educate People on the things that *Are* Hard

    Some things are intractably difficult, and that's okay. Rust has so far and
    will continue to focus on making those hard things easier through education.
    
    One of the examples of this that was particularly striking to me when I was
    getting started with Rust was the documentation: [The Book](https://doc.rust-lang.org/stable/book/), the stanstartedbrary documentation, and the Rustonomicon (a guide to writing unsafe Rust)
    are all included with the language itself and can be easily accessed by
    running \`rustup doc\`. This means that I wasn't dead in the water when I
    wanted to write some Rust on an airplane and had forgotten to pre-download
    the docs. It also shows thoughtfulness and empathy towards the many people
    who are developing software in situations where constant, fast Internet
    access is not a given, a group that is very often a blind spot of modern
    tech.
    
    In addition to the standard docs, every single crate uploaded to crates.io
    automatically has its API documentation built and hosted on doc.rust-lang.org.
    This is wildly fantastic! Sure, you can build your API docs in other languages
    (although Python in particular is often a herculean task involving restructured
    text and obscure Sphinx configs), but then you've got to figure out how to
    host them. For Rust, every third-party library automatically has published
    documentation, accessible in a consistent format.
    
    Another example is of course Rust's famous compiler messages. Much ink has
    been spilled extolling their virtues, so I won't go too far into it, except
    to say that probably 90% of how I learned about Rust's system of ownership
    validation and borrowing came from reading compiler errors. This was, in fact,
    the entire focus of a [later talk in the conference](https://www.youtube.com/watch?v=Z6X7Ada0ugE&list=PL85XCvVPmGQijqvMcMBfYAwExx1eBu1Ei&index=5) by Esteban Kuber, in which
    he expressed the radical idea that the *primary* purpose of a compiler, rather
    than being to compile code, should be to provide a tool to teach the user
    how to write code that can be compiled. Esteban's talk focused on the unique
    nature of the compiler as the one tool that everyone has to use, and that,
    through analysis of what the user *tried* to do, it can potentially serve as
    a personalized tutor, guiding the user towards correcting their misconceptions,
    even providing tailored feedback depending on which other language it seems like
    the user is trying to map onto Rust semantics.
    
    For the future, there are continuing efforts to make error messages even better
    (see Esteban's talk for some examples). In addition, the [Rust Forge](https://forge.rust-lang.org/) aims to make
    it easier for people to start contributing to Rust, while the [rustc-dev-guide](https://rustc-dev-guide.rust-lang.org/index.html)
    hopes to ease the learning curve for working on the compiler.

3.  Provide Access to Spaces and Power in the Project

    Rust is community driven, and it is relatively famous for having one of the more
    welcoming, helpful communities of any programming language. It turns out this
    isn't an accident.
    
    From the beginning, the Rust project had a [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) focusing on empathy
    and inclusion. The founder insisted upon it because he didn't want to have
    to "subject [himself] to the kind of discourse normally surrounding
    language-building communities. In other words: the norms of *other*
    communities were already excluding *me*." The presence and enforcement of
    this code of conduct has helped Rust to build a vibrant community where
    people feel as though they can express themselves freely. The effects of this
    focus could be seen at RustConf 2020, at which over half of the speaker lineup
    (selected blind) comprised trans people.
    
    This focus on providing access can also be seen in the Rust RFC process. All
    new language features are proposed as RFCs. *Anyone* can contribute an RFC,
    and *anyone* can comment on one. This gives the community an amazing amount
    of visibility into and opportunity to influence the language. In addition,
    all Rust governance is performed by teams drawn from the community. The team
    structure allows people who are passionate about a given aspect of the language
    to have the opportunity to have their say and contribute.
    
    While Rust's community is probably among the best of any programming language,
    it's still got room for improvement. The new [RustBridge](https://rustbridge.com/) project aims to teach
    Rust to and provide opportunities for people from underrepresented backgrounds.
    In addition, new teams have been created to focus on ensuring that the growth
    of the language does not come with a loss of its culture of inclusion, as
    online spaces like [/r/rust](https://www.reddit.com/r/rust/) continue to grow. In addition, the core team took
    the time to highlight [Awesome Rust Mentors](https://rustbeginners.github.io/awesome-rust-mentors/), a project started by Jane Lusby
    (author of the [eyre](https://docs.rs/eyre) library and of several recent error-handling RFCs).
    
    Finally, among language contributors, there is a renewed focus around ensuring
    that people aren't being "heroes" and burning out, an acknowledgment that
    mentoring and community building is just as essential as language development,
    and a continued focus on empathy.


### The Politics of Empathy

Given the primacy of people in Rust's values, and in particular the focus on
empathy as a means of understanding people's needs in order to improve the
language, it seems unavoidable that some consideration of politics are
unavoidable. The portion of the opening keynote by Ashley Williams approached
the political question head on, and her section was one of my favorite bits
of the conference, veering definitely philosophical.

She noted that Rust wants to be impactful, and the way it wants to be impactful
is described in its motto, "empowering everyone to build reliable and efficient
software." To empower, she notes, is to give someone power to do something,
and one of the definitions of politics is "the debate or conflict among
individuals or parties hoping to achieve power." As such, in order to properly
fulfill its goal, Rust innately is and must be political. Avoiding this issue
would negate the stated goal. And to begin to address it, one of the most
important question is *who* does Rust want to empower? The slogan says
"everyone" of course, but Ashley points out that focus is a limited resource,
and so Rust must make decisions about what people they focus on when making
decisions. Rust has a remarkable diversity of developers from different
programming languages and paradigms: C and C++ developers; web developers
who come from writing Python, Ruby, and JavaScript; researchers;
data scientists; and more. It cannot be all things to all of these groups
at all times.

One of the goals when focusing on any given audience for Rust is expressed
by the old saw "a rising tide lifts all ships," which Ashley wisely points
out as having a hidden value judgment, namely that a rising tide *should*
lift all ships. At least in the context of the Rust project, they believe that
it should, and that it's important to evaluate whether all ships really
are rising as the tide rises. As such, Rust's framework for choosing who
to focus on looks at who has the greatest need: who has the fewest resources
and the least ability to help themselves? The Rust team believes that
producing things for the people with the greatest need should and does wind up
helping everyone, often in surprising ways.

With that in mind, Rust aims to be a "systems programming" language, built
*especially* for people who haven't done or don't think that they can do
systems programming. A clear result of this has been the steady bubbling
up of Rust in spaces outside of systems programming, since its performance
and correctness guarantees, along with its ergonomics, are appealing to
engineers of all stripes.

As Ashley put it, Rust wants to redistribute the power of systems programming,
and she points out that they need the community to get involved, to contribute,
and to help ensure that Rust will have the capacity it needs to do things
the right way.


## A General Purpose Systems Language: the Future of Rust

The Rust team's focus on core values and empathy gives me confidence that the
language will only get better over time. However, another thing the conference
really highlighted for me was the massive success that Rust has already had
in expanding beyond the realm of systems programming.


### A Faster, Safer Glue Language

Typically, Python is the language of choice for stitching together a bundle
of existing tools (often written in different languages) for the purpose of
analytics or data processing. [Under a Microscope: Exploring Fast and Safe Rust
for Biology](https://www.youtube.com/watch?v=2b8InauuRqw&list=PL85XCvVPmGQijqvMcMBfYAwExx1eBu1Ei&index=4) looks at how Rust can be a great fit for this use case. Often
such processing requires streaming of data between processes, which can
sometimes necessitate data transformation to map something from one program's
output to another's input. In this scenario, it's easy for your glue to
become a bottleneck. Not only can Rust potentially resolve such a bottleneck
by virtue of its own speed, but its ability to tightly integrate with C
and C++ codebases means that sometimes you may not even need the subprocess
boundary, instead just being able to call functions directly from your
"glue" layer and eliminate an entire parsing step.


### Game Development

Long the domain of C++ (and sometimes C#), Rust's gaming ecosystem is getting
to the point where it's a serious option for indie game development.
[Starting a 2D Game with Amethyst](https://www.youtube.com/watch?v=GFi_EdS_s_c&list=PL85XCvVPmGQijqvMcMBfYAwExx1eBu1Ei&index=6) highlights how easy it is to get a basic
2D game going using the amethyst framework. It may take years for a AAA
framework like Unreal to show up in Rust, but in the meantime, I expect
to see the more and more small gaming studios pick Rust up for its
improved ergonomics compared to the traditional options.


### Hardware Drivers

Definitely a "systems programming" thing, but [Controlling Telescope Hardware
with Rust](https://www.youtube.com/watch?v=xlVnp7VOxRE&list=PL85XCvVPmGQijqvMcMBfYAwExx1eBu1Ei&index=7) really helped to highlight how much less intimidating it feels to
interface with some old hardware's C API when you can wrap it in Rust. As
Rust steadily makes its way towards being allowed for Linux kernel development,
I could see it being a pathway for people to move from web development or
another of the realms where Rust has gained some popularity into these
types of low-level projects. Between the improved language ergonomics and
the improved safety, I hope to see Rust help to fill the general lack of
engineers able to and willing to work on things like device drivers for
Linux operating systems.


### Scripts and General Programming

[Rust for Non-Systems Programmers](https://www.youtube.com/watch?v=BBvcK_nXUEg&list=PL85XCvVPmGQijqvMcMBfYAwExx1eBu1Ei&index=9) was a great talk that looked at some tips
and shortcuts that you can take to make Rust development (nearly) as easy
as writing something in a dynamic language like Python. It pointed out, for
example, how much of the error handling in Rust would simply throw an
exception in other languages, so especially for an initial implementation
or a quick script, `.unwrap()` is often just fine. Similarly, it's okay
to `.clone()` things rather than dealing with lifetimes.

In addition to looking at how to make writing Rust more accessible,
the talk looked at some of the things that Rust makes particularly *easy*,
even compared with dynamic languages. This includes best-in-class libraries
for things like argument parsing and serialization/deserialization, as well
as language features like impls with generic methods, enums, pattern
matching, and inline unit testing.


## Highlighted Talks

A couple of other talks I really enjoyed but didn't get the chance to highlight
elsewhere were Jane Lusby's [Error Handling Isn't All About Errors](https://www.youtube.com/watch?v=rAF8mLI0naQ&list=PL85XCvVPmGQijqvMcMBfYAwExx1eBu1Ei&index=2) and
Siân Griffin's closing keynote, [Learning Empathy from Pokémon Blue](https://www.youtube.com/watch?v=RNsEsZbXE-4&list=PL85XCvVPmGQijqvMcMBfYAwExx1eBu1Ei&index=10). The former
was a great deep dive into a differentiation Rust's trait system enables between
errors and error reporters, the differences between how different types of
errors should be handled, and some tips for library and application authors.
The latter is one of the most interesting talks I've ever seen, looking at
the confluence of factors that led to the MissingNo (RareCandy) glitch in
Pokémon Blue. This was a great exploration of the effects of constraints
on development in general, on the particular constraints of GameBoy
development in the 90s, and the amazingly subtle ways in which a variety
of unrelated behavior can interact in unexpected ways to create legendary
glitches.


## Summary

RustConf was a great experience. I came away from it with a significantly
better understanding of the guiding principles behind the language, which
made me even more optimistic than I already was about Rust's future. It
feels good to be working with a project that makes such open and overt
efforts to be inclusive and to ensure that everyone feels like they're
welcome. I believe that focusing on empathy towards the users of the
language and towards the language developers will keep Rust as one of,
if not the most loved language for years to come.

Meanwhile, I was amazed by the huge variety of projects that people are
using Rust for. It's inspiring to see it bringing people into worlds and
types of programming that they've never experienced before. Something
that was highlighted in the opening keynote is that Rust proves that
the common conception that developers of different languages are different
or incompatible in some way is simply false. Rust has become a language where
JavaScript developers and C++ developers can form a common community, a
prospect that might have seemed impossible a few years ago.

As all of my coworkers know&#x2014;at this point my bringing up Rust in
barely related conversations is basically a drinking game&#x2014;I love Rust.
Prior to RustConf, this was due almost entirely to the virtues of the
language itself. However, the conference showed that the language was not
created *ex nihilo*, and perhaps unsurprisingly, the community that
created it is also pretty darn great.

