title: Why Are We So Eager to Break Things?
slug: why-are-we-so-eager-to-break-things
created: 2020-08-20
updated: 2020-08-20
tags: software, teams, engineering, resilience
summary: An exploration of some of the ways we intentionally or unintentionally make extra work and churn for our colleagues, how we justify it to ourselves, and how we can avoid it.

# Why Are We So Eager to Break Things?

I read a great article by Steve Yegge this week, [*Dear Google, Your
Deprecation Policy Is Killing You*](https://medium.com/@steve.yegge/dear-google-cloud-your-deprecation-policy-is-killing-you-ee7525dc05dc), full of well-appreciated snark and
profanity. The gist of it is that Google takes a much too cavalier
approach to backwards compatibility in its services. While the article
goes into detail about what it specifically about Google that might cause
them to be culturally biased towards breaking changes, the problem is far
more wide-reaching than Google. It&rsquo;s not limited to a particular platform or
ecosystem. It&rsquo;s not even limited to programming interfaces. It&rsquo;s symptomatic
of a general mindset that pervades the industry, creating a significant
amount of required learning and labor that effectively serves as
a constant tax on every engineer, sapping hours and hours of their time
to update things that were *already working*.

I think this feeling of freedom to break things is especially problematic
*within* engineering teams, where because our decisions are not directly
breaking anything for customers, we don't consider them to be important.
But, if I'm working on a codebase with 10 other people, and I introduce
a change that costs each of them two hours, I've just cost my company
somewhere on the order of $1,000 in lost time alone. This is of course
compounded by decisions that leech more and more time as they become
entrenched, and does not account for qualitative things like developer
satisfaction.

## Breakage, Breakage Everywhere

What are some of the ways we might, knowingly or unknowingly, cost our
fellow engineers their time and sanity?  What leads to our generally blasé
approach to breaking things?

### My Library, My Rules

People maintaining open-source libraries are not beholden to anyone, and are
free to make breaking changes whenever they like. They may or may not adhere
to &ldquo;proper&rdquo; semantic versioning. They may decide that breaking changes are worth
the hassle for the sake of cleanliness, intuitiveness, or any number of other
subjective measures.

Let me be clear: open source maintainers, working in their spare time for free
have *absolutely no obligation* to do things in any particular way. I am
not being critical here of project maintainers who break backwards
compatibility in their projects. I have done this in my own open-source
projects, and I certainly wouldn&rsquo;t appreciate some rando on the Internet
telling me how I should manage my project.

That being said, free and open-source libraries come with no compatibility
guarantees, no support, and no guarantee of timely resolution. When engineers
at Foo Corp choose to use a library rather than writing software themselves,
they are implicitly agreeing to deal with any associated churn due to that
library breaking its backwards compatibility, introducing regressions, failing
to fix bugs in a timely manner, etc.

Every dependency, and every dependency of that dependency, recursively, becomes
a part of your project just as surely as if you had written the code yourself.
Engineers not taking the time to fully evaluate dependencies&#x2014;to honestly
consider whether the risk of relying on unpaid volunteers&rsquo; code is worth the
risk&#x2014;creates a huge amount of churn within organizations.

Not to beat a dead horse, but this is more of a problem in some ecosystems than
others. Many of us have started a brand spanking new JavaScript project with
up-to-date dependencies and then watched in horror as, six months down the road,
security vulnerabilities pop up in dependencies of dependencies, with no
reasonable update path that satisfies all of our explicit dependencies&rsquo;
requirements. I *do not* think that this is a fundamental problem with
JavaScript or NPM. I think it&rsquo;s the result of a culture that fails to emphasize
the fact that third-party dependencies are not a free pass to get new
functionality without writing it yourself. The culture perpetuates itself
partly because the Jenga tower of FE dependencies necessary to enable any
modern frontend pipeline is so complex that you&rsquo;re *starting out* with hundreds
of dependencies, no matter what you do.

The thing is, free, open-source software being readily available in an
easy-to-use package manager is an *amazing* thing! We would be foolish not to
take advantage of it. We&rsquo;ve just got to learn as an industry to be more
intentional about what we choose to pull in and when. We should always consider
the package&rsquo;s release history, its commit frequency, its dependency graph,
and the cost of just doing whatever we need ourselves. Do the maintainers
care about backwards compatibility? If we&rsquo;re not checking, we have no right
to complain.

### This Will Make Your Life Better

More than once I have had a new employee join a project, look at the way things
were being compiled/built/deployed/linted, and *immediately* start pushing some
New Tool that will make everything SO MUCH BETTER. *How* could we possibly be
maintaining this project without it? Our lives would be *so much easier* with
this one cool tool!

Sometimes, they were right! More often, they&rsquo;ve introduced some new hiccup (at
best) or significant impediment (at worst) in the development workflow, causing
errors on everyone&rsquo;s machines that they have to go ask in chat how to fix,
potentially causing us to have to go back and do *more* work later to rip out
whatever fancy new tool they added.

The thing is, new tools can be awesome. I&rsquo;ve introduced lots of new tools to
various projects that I&rsquo;ve worked on. But if you&rsquo;re introducing a new tool
without first taking the time to understand the existing developer workflow,
without adequate documentation about how to start using it, or without
considering all of the ways in which the tool might fail and accounting for
them, you are *actively robbing* your coworkers of time. Time that could
probably be better spent working on things that make the company money.

Similarly to the discussion on OSS dependencies from package managers, the key
is intentionality. Take the time to understand how you&rsquo;re changing
your coworkers&rsquo; lives, and try to make them better! Communication is
also critical. If you&rsquo;ve carefully decided that disruptive change is for
the better, take the time to ensure you&rsquo;ve communicated the change to the
best of your ability, with written documentation, conversations, and
presentations if needed.

### We Have to Do it the Right Way (TM)

Sometimes a coworker is an Evangelist. What they&rsquo;re an Evangelist for may vary,
but the result tends to be the same: changes introduced for the sake of doing
things the Proper Way, The Right Way, the Ideologically Pure Way. Whether that
Way is blessed by the founder of a language, some blog guru, or one of the
FAANG companies, you&rsquo;ll find changes introduced to one codebase (best case
scenario) or across the company (worst case scenario) with minimal technical
justification, shoddy understanding of the underlying reasons why you might
want to make such a change, and a bunch of frustrating arguments that
essentially boil down to &ldquo;we should do this because it&rsquo;s what people do.&rdquo;

As is almost certainly obvious from the tone of the above paragraph, I strongly
disagree with changes for evangelism&rsquo;s sake. I could write a whole post just
about that. Here, though, I want to focus on the fact that these changes, like
the changes with developer tooling, almost invariably result in breakage to
lots of people&rsquo;s workflows. To the Evangelist, the suffering resulting from
this breakage is noble, a necessary sacrifice for the sake of aligning with
The Vision. They tend to provide minimal considerations for making the path
forward easier for people: they will be made Better by taking the time to work
through it, to learn about the One True Way. The Evangelist also tends to feel
like The Way is obvious, further exacerbating their tendency to not provide
documentation or assistance with any transitions. If you don&rsquo;t get it, it&rsquo;s
a personal failing on your part.

You may have seen this with a decree that all endpoints must be fully
ReSTful, that a service be rewritten in the &ldquo;better&rdquo; language, that a
framework be used because it&rsquo;s the community favorite, or that a container
orchestration tool is absolutely necessary if we ever intend to &ldquo;scale&rdquo;.
Often these decrees will require a massive amount of work. Sometimes, it&rsquo;s
worth it. Oftentimes, it isn&rsquo;t.

Again, we need to apply some intentionality to the decisions we make. We should
introspect ourselves to make sure *we&rsquo;re* not being the Evangelist. We should
insist that our technical decisions be made based on real, demonstrable,
ideally measurable criteria. And we should watch out that we&rsquo;re not getting
carried away with someone else&rsquo;s evangelism. It&rsquo;s easy for a company to spend
months and hundreds of man hours on bringing things in conformance with The One
True Path, and it&rsquo;s easy for one senior evangelist to continually force the
company through such gauntlets of purification. It&rsquo;s hard to be the curmudgeon
fighting the new, the exciting, the Ideologically Pure.

## Avoiding Unnecessary Breakage

We&rsquo;ve covered a few different cases where we can, if we&rsquo;re not being careful,
create lots of extra work for ourselves or our colleagues. What&rsquo;s interesting
about all of these cases is that the remedy is largely the same: take some time
and be intentional about introducing new things. Whether it&rsquo;s a third-party
dependency, a fancy tool to manage local programming language environments,
or a new philosophy and paradigm, it is critical that we *first* fully
understand the context into which we&rsquo;re introducing the change. Sometimes, you
might find that that third-party dependency can be replaced with 50 lines of
code. Sometimes, when you spend some time with something, you might find
it&rsquo;s nowhere near as bad as you expected. Sometimes, it&rsquo;s bad, but the
change you&rsquo;d like to introduce is going to cause so many issues that
the cure is worse than the disease. Sometimes the philosophy you&rsquo;d like to
apply just doesn&rsquo;t fit.

And of course, sometimes it&rsquo;s worth it. Sometimes the benefits outweigh the
costs. That is totally okay, and in these cases, there&rsquo;s nothing wrong with
breaking a few eggs to make an omelet.

However, if you&rsquo;re not taking the time to consider whether it is worth breaking
the eggs, you may soon find that your colleagues are much less receptive to your
ideas, that your codebase is bogged down in rewrite for rewrites&rsquo; sake, and that
everyone is constantly complaining about their tooling. And that&rsquo;s not a place
any of us want to be.
