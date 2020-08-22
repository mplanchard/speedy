title: Selected Reads (2020-08-22)
slug: selected-reads-2020-08-22
created: 2020-08-22
updated: 2020-08-22
tags: google,haskell,software,politics,rust,RustConf2020,covid,teams,academics
summary: It&rsquo;s been a wild ride this week, with RustConf and the Democratic National Convention generating a significant amount of interesting content to digest, along with the usual steady trickle of interesting reads.


# Selected Reads (2020-08-22)

This is a selection of articles, videos, tweets, and sundry that I&rsquo;ve found
interesting over the last week or so.


## [Dear Google Cloud: Your Deprecation Policy is Killing You](https://medium.com/@steve.yegge/dear-google-cloud-your-deprecation-policy-is-killing-you-ee7525dc05dc)

****Author:**** Steve Yegge

A great post (rant, really) by Steve Yegge on Google Cloud&rsquo;s deprecation policy,
how it relates to google&rsquo;s internal culture, and how it affects their customers.
Largely the inspiration for my recent post on [software breakage within teams](https://blog.mplanchard.com/posts/why-are-we-so-eager-to-break-things.html).


## [Types as axioms, or: playing god with static types](https://lexi-lambda.github.io/blog/2020/08/13/types-as-axioms-or-playing-god-with-static-types/)

****Author:**** Alexis King

A new post from one of my favorite blogs. The same author&rsquo;s [*Parse, Don&rsquo;t Validate*](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
is probably the post on software engineering that I recommend the most often.
This one goes into types as axioms, encouraging us to think of types as
positive-space definitions of what our desired program behavior, rather than
as negative-space restrictions. This one was discussed in our work slack, and
one of my colleagues posted [this video](https://www.youtube.com/watch?v=uuTkuy9D5lY&amp;feature=share) of philosopher [Slavoj Žižek](https://en.wikipedia.org/wiki/Slavoj_%C5%BDi%C5%BEek) discussing
the intricacies of negation, which was both really interesting and a good
companion for the topics discussed in the blog post.


## [Baking those Potatoes with Microservices and Vendors](http://rachelbythebay.com/w/2020/08/17/potato/)

****Author:**** Rachel by the Bay

I liked this post setting up &ldquo;hot potato&rdquo; and &ldquo;cold potato&rdquo; routing as a
metaphor for team dynamics, particularly in a microservices context, where
&ldquo;hot potato&rdquo; refers to those people or teams who immediately take ownership
and dive into a problem, while &ldquo;cold potato&rdquo; refers to those who tend to
ignore or pass ownership on to other teams.


## [Laying the Foundation for Rust's future](https://blog.rust-lang.org/2020/08/18/laying-the-foundation-for-rusts-future.html)

****Author:**** Rust Core Team

Like most of us, I was upset by the news that Mozilla had to lay off about
a quarter of its workforce. I have been a Firefox user since 2004, other
than a brief flirtation with Chrome when it was new, and I appreciate all
the work Mozilla does to promote an open web. Rust was created at Mozilla,
and while I knew that Rust&rsquo;s governance was very intentionally community-driven
and not dependent on Mozilla, I was very glad to see that the core team
is creating a foundation so that they can manage the financial and
contractual requirements the language requires.


## RustConf 2020

I&rsquo;m going to be writing an entire post on RustConf, but here are a few quick
links to some of my favorite talks. All of the talks were great, but these
are the ones I think I will be most frequently referring people to.


### [Opening Keynote](https://www.youtube.com/watch?v=IwPRu5FhfIQ)

****Presenter:**** Rust Core Team

I really dug the opening keynote of this year&rsquo;s conference, which emphasized
the main values of the Rust project (people, empathy) and talked about
how it&rsquo;s been Rust&rsquo;s position from the beginning that software is inherently
political, and to create a language and a community in which *everyone*
is empowered to build software, you must face the politics head-on, which
indeed the Rust project has consistently done. This presentation highlighted
how the focus of the language team on user community and empathy has helped
to make Rust the successful and much-loved language that it is.


### [Error Handling Isn't All About Errors](https://www.youtube.com/watch?v=rAF8mLI0naQ)

****Presenter:**** Jane Lusby (author of [eyre](https://docs.rs/eyre/0.6.0/eyre/))

A relatively deep dive into how Rust&rsquo;s trait system and approach to error
handling enables a couple of points of differentiation not often found in other
languages: first, that there is a clear difference in idiomatic handling
of recoverable vs. unrecoverable errors, and second, that the Error trait
enables a differentiation between errors and error reporters.


### [Rust for Non-Systems Programmers](https://www.youtube.com/watch?v=BBvcK_nXUEg)

****Presenter:**** Rebecca Turner

I was really glad to see this talk at the conference. It&rsquo;s an exploration
of writing a simple Rust program from the perspective of someone coming
from a dynamic language like Python, and it presents some techniques that
you can use to speed up your initial forays into the language, especially
when writing something for your own edification (don&rsquo;t be afraid to unwrap,
don&rsquo;t be afraid to clone, etc.). This will go straight into my list of
resources to send people who are getting into Rust for the first time.


### [Learning Empathy from Pokémon Blue](https://www.youtube.com/watch?v=RNsEsZbXE-4)

****Presenter:**** Siân Griffin

The closing keynote, this talk was not particularly rust-specific, but was
without a doubt one of the more interesting talks I&rsquo;ve ever seen. It takes
a deep dive into the MissingNo rare candy glitch from Pokémon Blue, with
Rust examples of what Siân was able to learn about how the original game
worked. This was an amazing exploration both into the constraints of the
programming required for this generation of games and the subtle
ways that small, seemingly innocuous bugs can work together to
create entirely unexpected effects. A must-watch!


## [University Funding and COVID preparedness](https://twitter.com/bretdevereaux/status/1295909929228873728?s=21)

****Author:**** Bret Devereux

A twitter thread from the author of one of my absolute favorite [blogs](https://acoup.blog/)
(an ancient military historian who often writes about fantasy and science
fiction, his six-post series on the [tactics and strategy at the Siege of Gondor](https://acoup.blog/2019/05/10/collections-the-siege-of-gondor/)
is absolute gold, as is his exploration of [what Sparta was *actually* like](https://acoup.blog/2019/08/16/collections-this-isnt-sparta-part-i-spartan-school/)).
Bret Devereux here talks about *why* it would&rsquo;ve been financial suicide for
colleges to not attempt reopening, given the last 20 years or so of cuts to
public funding for higher education, increased reliance on adjunct teaching
staff, flat professor salaries, and ballooning administrative costs. A clear
and cogent call for change in the way we fund our colleges, I found myself
nodding along vigorously, as his points all matched exactly what I saw in
my graduate program.


## [Bill Gates Interview on the United States' COVID Situation](https://arstechnica.com/?p=1697546)

This interview posted to Ars Technica is a really interesting one. I found particularly
interesting Gates&rsquo; stance on political donations, and his assessment of the
current state of COVID testing in the United States.


## Politics

I&rsquo;m not generally interested in making this blog super political, but it
certainly does seem to me that the current president is hell-bent on following
what is a distressingly tyrannical playbook. As such, I was very glad to see
that the main speeches made this week at the Democratic National Convention
were without exception excellent.

Even if you&rsquo;ve already decided that you&rsquo;ll vote for literally any human being
with a pulse other than Trump like I had done, I think that these speeches are
worth watching, if only to feel that warm fuzzy feeling again of watching
leaders who are capable of stringing together coherent sentences.


### [Michelle Obama DNC Speech](https://twitter.com/michelleobama/status/1295552611026780160?s=21)

The former first lady makes a moving exhortation that this is one of the most
important elections of our lifetimes, and that even if you normally detest
politics, there has never been a more important time to exercise your right
to vote.


### [Barack Obama DNC Speech](https://www.axios.com/obama-dnc-speech-d7326f30-dab1-4c1c-af78-ff1dc5f28f8d.html)

I had plenty of issues with particular things Obama did or didn&rsquo;t do while he
was president, but damn if he isn&rsquo;t one of the best orators in modern
history. I won&rsquo;t lie, I&rsquo;ve been hoping for the duration of the Trump presidency
for Obama to take the gloves off and tell us what he really thinks, and this
is I think one of the first times I&rsquo;ve heard him be openly critical of the
sitting president. Definitely worth a watch.


### [Biden Nomination Speech](https://www.c-span.org/video/?c4901075/joe-biden-accepts-democratic-party-nomination-president)

Seeing that even Fox News anchors were admitting that this was an undeniably
strong speech, I checked it out. It&rsquo;s a wonderful speech. Strong, emotional,
well measured, and hitting all of the salient points, it&rsquo;s moved me from being
primarily an anti-Trump voter to an actually a little excited about Biden
voter.

