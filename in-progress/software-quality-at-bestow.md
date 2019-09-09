title: Software Quality at Bestow
slug: software-quality-at-bestow
created: 2019-09-08
updated: 2019-08-08
tags: software quality, craftsmanship, bestow, engineering
summary: a look at our approach to building quality software at Bestow

# Outline

1. Introduction
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
    * Extendability
        1. Segregation of public/private methods
        2. Composition over inheritance
        3. SOLID principles (even for non-OOP code, where applicable)
        4. Planning for partial application / currying
    * Continuous Improvement
        1. Campsite coding
        2. Continuous refactoring
        3. Adding types
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
5. How has Quality Benefited Us
    * Running big new system in shadow environment to validate
    * Drawing boundaries in legacy codebases and improving bit by bit
    * Successful deployment of new systems
    * Successful transition across clouds
6. Conclusion

# Introduction: Software Quality at Bestow

I'm lucky enough to work at a company and on an engineering team that
has largely internalized one of the most difficult lessons in software
development: "Short cuts make long delays," to quote the indomitable
Samwise Gamgee, or more accurately to quote Samwise quoting the Old Gaffer.
Which is to say that, like [Martin Fowler has pointed out][fowler-quality]
with a great deal more experience and insight than I will ever have, that
sacrificing software quality for the sake of speed catches up to you a
lot more quickly than you might expect in terms of accumulated technical
debt and reduced overall velocity. But, what exactly is quality, and how
do we create a culture here at Bestow that values it?

[fowler-quality]: https://martinfowler.com/articles/is-quality-worth-cost.html
[impure-pure-impure]: https://blog.ploeh.dk/2019/08/26/functional-file-system/
[simple-made-easy]: (https://www.infoq.com/presentations/Simple-Made-Easy/)
