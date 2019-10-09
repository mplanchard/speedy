title: Software Quality at Bestow
slug: software-quality-at-bestow
created: 2019-09-08
updated: 2019-08-08
tags: software quality, craftsmanship, bestow, engineering, series
summary: a look at our approach to building quality software at Bestow, pt 1

# Outline

1. Introduction
1. Goals
    * Driving business value
    * Investing in great people
    * Doing work we can be proud of
2. What is Quality
    * Simplicity
        1. As simple as possible, but no simpler
        1. Simplicity is hard (link to [Rich Hickey talk][simple-made-easy]?)
    * Correctness
        1. Code should do the right thing
        1. The fact that code does the right thing should be verified
        1. It should be easy to see whether the code does the right thing
            * referential transparency
            * pure functions
            * dumb objects
            * do these things belong with simplicity? maybe just link back?
              or maybe put correctness first, and then phrase simplicity as
              a means to that end?
        1. Where possible, correctness should be constrained by types or invariants
    * Resilience
        1. Failure is expected
        2. Error cases are considered
            * e.g. go nil handling
            * python result types (which I finally figured out how to make work)
        3. Active decisions about whether to handle or panic
        4. Invariants are maintained
        5. Easy rollbacks
        6. Feature flags
    * Extensibility
        1. Segregation of public/private methods
        2. Composition over inheritance
        3. SOLID principles (even for non-OOP code, where applicable)
        4. Planning for partial application / currying
3. What Quality is Not
    * Perfection
        1. It's impossible
        2. It takes way too long to try
        3. Resilience + Working Code > Perfection
    * Complication
        1. Avoiding unnecessary abstraction (has Abramov published his talk
           from deconstruct yet?)
        2. Shared mutable state
        3. Spooky action at a distance
        4. Functional logic split across boundaries
4. How we Promote Quality
    * Hiring process
        1. Talk about moving to third party tech screens?
        2. Code and architecture review for (almost?) all levels
        3. If it's not a strong yes it's a no?
    * Code review
        1. No code without tests
        2. Accountability by both reviewer(s) and author(s)
    * Engineering Principles
        1. Prefer immutability
        2. Prefer purity
        3. Prefer composition
        4. Prefer static typing
        5. Bugs to the left
        6. Isolate state ([impure-pure-impure] sandwich?)
        7. Untested code is broken code
    * Continuous Improvement
        1. Campsite coding
        2. Continuous refactoring
        3. Adding types
5. How has Quality Benefited Us
    * Running big new system in shadow environment to validate
    * Drawing boundaries in legacy codebases and improving bit by bit
    * Successful deployment of new systems
    * Successful transition across clouds
6. Conclusion

# Software Quality at Bestow

I'm lucky enough to work at a company and on an engineering team that
has largely internalized one of the most difficult lessons in software
development. To quote the indomitable Samwise Gamgee, or more accurately
to quote Samwise quoting his Old Gaffer, "Short cuts make long delays."
Putting this in software terms, as [Martin Fowler has pointed out][fowler-quality],
with a great deal more experience and insight than I will ever have,
sacrificing software quality for the sake of speed catches up to you a
lot more quickly than you might expect in terms of accumulated technical
debt and reduced overall velocity. But, what exactly is quality, and how
do we create a culture here at Bestow that values it?

This post will be the first in a series diving into how we strive to realize
and maintain a high level of software quality at Bestow. What are the
values that drive us? How do we define quality? What strategies and tactics
do we use to accomplish our goals? Future posts in this series will provide
deep dives into these topics, but here we wish to give a general overview.
We hope that through writing down and publishing our philosophy, we can
both attract talent aligned with our goals and help to introduce new
engineers into the fundamentals that inspire culture they are joining.

## Goals, or Why Quality Matters

When we work to define software quality, we must be careful to always tie
it back to the concrete goals that we serve as an engineering team. Perfection
for perfection's sake is needed when building _Notre-Dame de Paris_, but
perfection takes a great deal of time (_Notre-Dame's_ initial incarnation took
180 years to build), and we must know when spending the extra time to make
something _more_ perfect aligns with our broader goals.

<!--TODO: flesh this out -->

## Defining Quality

Lots of ink has been spilled, both physically and digitally, on defining
software quality. [There][clean-code] [are][philosophy-of-sw-design] [entire][data-intensive-applications]
[books][refactoring] on the [subject][design-patterns] (all of the linked
books have inspired several or many of us at different times in our careers).
With that in mind, our goal here is to specify which of the many aspects
of software quality described elsewhere we view to be most important here.

### Quality Software Is

Below we've selected the top characteristics of what we consider to be
quality software. Importantly, we consider these characteristics to be
applicable at every layer of software design, from the grand architecture
to the humble function, and we strive to consider them at every step of our
process, from product specification through code review.

#### Correct

Perhaps the most important of the attributes of software quality, correctness
is to software the eternal [Brahman] from which all other aspects of creation
arise (or pick your all-encompassing metaphysical concept from your belief
system of choice). That is to say, without correctness, every other measure
of quality is rendered null and void. If the software does not do what it
is intended to do, it is worthless, regardless of how elegantly it is designed,
how easily extensible it is, or how well it responds to unexpected states.

**Correctness is whether the software does what it is intended to do.**

Correctness in software is both a metric of quality _and_ the primary
driving factor for defining other characteristics of quality software.
Quality software is software that is as correct as possible, as consistently
as possible, over its entire lifetime. But software that is purely correct
at any given point in time will not necessarily remain so, and it is that
_over time_ qualifier that gives rise to the need to define other
characteristics of software quality outside of raw correctness.

#### Resilient

resilient

#### Extensible

extensible

#### Simple

> Simplicity is prerequisite for reliability.
>
> &mdash;Dijkstra, [_Truths that Might Hurt_][truths-that-might-hurt]

As Rich Hickey, author of Clojure, so eloquently put it in his famous
["Simple Made Easy" talk][simple-made-easy], simple _does not_ mean easy.
"Easy" means familiar, within our wheelhouse, or readily accessible.
"Simple," on the other hand, in mathematics and related fields
requiring precise definitions, means being composed of a single element. A
simple leaf in botany is one that is not divided. A simple tense is formed
with one word rather than two, e.g. "laughed" vs. "was laughing." It is
this definition of simplicity that we strive to achieve.

**Something is simple when it is composed of a single element or has a single role.**

Of course, if that is simple, than "simplicity" in our software is a measure of
how close any given piece of it comes to being truly "simple." The closer it
is to having one role, the simpler it is. The fewer components it comprises,
the simpler it is.

[brahman]: https://en.wikipedia.org/wiki/Brahman
[clean-code]: https://www.bookpeople.com/book/9780132350884
[data-intensive-applications]: https://www.bookpeople.com/book/9781449373320
[design-patterns]: https://www.bookpeople.com/book/9780201633610
[fowler-quality]: https://martinfowler.com/articles/is-quality-worth-cost.html
[impure-pure-impure]: https://blog.ploeh.dk/2019/08/26/functional-file-system/
[philosophy-of-sw-design]: https://www.abebooks.com/Philosophy-Software-Design-John-Ousterhout-Yaknyam/22867499370/bd
[refactoring]: https://www.bookpeople.com/book/9780134757599
[simple-made-easy]: (https://www.infoq.com/presentations/Simple-Made-Easy/)
[truths-that-might-hurt]: (http://www.cs.utexas.edu/users/EWD/transcriptions/EWD04xx/EWD498.html)
