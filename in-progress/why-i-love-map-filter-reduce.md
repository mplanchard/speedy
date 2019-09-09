title: I Love Map, Filter, and Reduce
slug: why-i-love-map-filter-reduce
created: 2019-08-18
updated: 2019-08-18
tags: functional programming, map, filter, reduce, python, javascript, rust, f#, kotlin
summary: why I vastly prefer functional array methods to for loops

# Why I Love Map, Filter, and Reduce

I've been writing Python as my primary language at work for the last
five years. I have plenty of issues with Python, but I think one of my
biggest is the apparent dislike of functional programming paradigms
expressed both by the language designers and many Python programmers.
I'll dive into the Python situation in more detail later, but for now,
I'd like to make the case that _all_ programmers should be familiar with
`map()`, `filter()`, and `reduce()`, and none of us should shy away from
using them.

I say this not because I'm a huge fan of functional programming in general,
but because I honestly think that these abstractions are often clearer
and more readable. In addition, because of their ubiquity in virtually
all programming languages, learning to use them well gives you a set of
tools that you can use to easily transfer from one language to the next.

## Ubiquity

Let's say I have some iterable of dictionaries/hash maps/objects (depending
on your language's semantics) representing song information. Perhaps I am
iterating over results from a database, or maybe I've read some data
from a file. Each item looks like this:

```json
{
    "title": "2 + 2 = 5",
    "artist": "Radiohead",
    "personnel": {
        "Thom Yorke": [
            "vocals", "guitar"
        ],
        "Jonny Greenwood": [
            "guitar", "analogue systems", "ondes Martenot"
        ],
        "Ed O'Brien": [
            "guitar", "effects", "backing vocals"
        ],
        "Colin Greenwood": [
            "bass"
        ],
        "Philip Selway": [
            "drums", "percussion"
        ]
    }
}
```

Let's assume my dataset represents an entire library of music of different
artists, and that I want to figure out which artists are represented in
the library. As always, there are a lot of ways to do this, but let's look
at a `map()` based solution. First, Python:

```py
# Assuming the type Song is a class corresponding to the song structure
def get_artists(songs: t.Iterable[Song]) -> t.Set[str]:
    # Map over the songs, returning the artist from each one.
    # Collect that into a set.
    return set(map(lambda song: song.artist, songs))
```

Now, Javascript:

```js
// Assuming the song is an object corresponding to the song structure
const getArtists = (songs) => {
    // Map over the songs, returning the artist from each one.
    // Collect that into a set.
    return new Set(songs.map(song => song.artist))
};
```

And Rust:

```rs
// Assuming the type Song is a struct corresponding to the song structure
fn getArtist<S: IntoIterator<Item=Song>>(songs: S) -> HashSet<String> {
    // Map over the songs, returning the artist from each one.
    // Collect that into a set.
    songs.into_iter().map(|song| song.artist).collect()
}
```

And F#:

```f#
// Assuming the type Song is a type corresponding to the song structure
let getArtists songs =
    // Map over the songs, returning the artist from each one.
    // Collect that into a set.
    Set.ofList (List.map (fun song -> song.artist) songs)
```

And Kotlin:

```kotlin
// Assuming the Song is a data class corresponding to the song structure
fun getArtists(songs: List<Song>): Set<String> {
    // Map over the songs, returning the artist from each one
    val artists = songs.map { it.artist }
    // Collect that into a set
    return artists.toSet()
}
```

Anyway, you get the idea. Not only does this ubiquity make reading code
across languages easier, it means that you can, in almost any language,
reach for `map()` as your tool of choice for applying some function to
a group of things.

Now, let's say we wanted to get all of the songs by a given artist:

```py
def songs_by_artist(songs: t.Iterable[Song], artist: str) -> t.List[Song]:
    # Filter the songs, returning the ones whose artist names match.
    # Collect that into a list.
    return list(
        filter(lambda song: song.artist.lower() == artist.lower(), songs)
    )
```

and JavaScript:

```js
const songsByArtist = (songs, artist) => {
    // Filter the songs, returning the ones whose artist names match
    return songs.filter(
        song => song.artist.toLowerCase() === artist.toLowerCase()
    )
}
```

and Rust:

```rs
fn songs_by_artist(songs: Vec<Song>, artist: String) -> Vec<Song> {
    // Filter the songs, returning the ones whose artist names match
    songs.into_iter().filter(
        |song| song.artist.to_lowercase() == artist.to_lowercase()
    ).collect()  // collect into a Vec
}
```

And so on for F#, Kotlin, Haskell, Swift, and any other language with
support for higher order functions (in C# `map`, `filter`, and `reduce`
are `Select`, `Where`, and `Aggregate`, respectively, and a number of
languages, including Haskell, use `fold` rather than `reduce`, but the
signature and concept is the same).

Finally, `reduce()`, also known as `fold()`, which is generally viewed
to be the most complex of these three functions. As a general rule, you
reach for `reduce()` whenever you need to aggregate data from multiple
items in your collection. For our example, let's say we want to convert
our data into a mapping of artists to their songs, instead of just a list
of songs.

First, python:

```py
def artist_map(songs: t.Iterable[Song]) -> t.Dict[str, t.List[Song]]:
    return reduce(
        # Starting with an empty dictionary (the last parameter to reduce)
        # For each song:
        lambda songs_by_artist, song: {
            # Return a dict with all of the existing values
            **by_artist,
            # And either a new list containing the current song or
            # an existing list recreated with the song added to it
            song.artist: [*by_artist.get(song.artist, []), song]
        },
        songs,
        {},
    )
```

And javascript:

```js
const artistMap = (songs) => {
    // Starting with an empty object (the last parameter to reduce)
    // For each song:
    return songs.reduce((byArtist, song) => {
        return {
            // Return an object with all existing values
            ...byArtist,
            // And either a new list containing the current song or an
            // existing list recreated with the song added to it
            [song.artist]: [...(byArtist[song.artist] || []), song]
        }
    }, {})
}
```

Rust gets a little more complicated, because of its strong requirements
around reference safety. We also find that in Rust it is easier
to just modify our HashMap as we go, rather than recreating it on each
iteration of `fold()`:

```rs
// Given a reference to a vector of songs, we're going to return a
// HashMap whose keys are references to artist strings and whose values
// are vectors of references to the songs for that artist.
fn artist_map<'a>(songs: &'a Vec<Song>) -> HashMap<&'a String, Vec<&'a Song>> {
    // For each song
    songs
        .into_iter()
        // Starting with a new HashMap, apply a function that takes
        // a mutable hashmap reference as its first argument, and the
        // current song as the second argument
        .fold(HashMap::new(), |mut by_artist, song| {
            by_artist
                // Check for an artist entry in the HashMap
                .entry(&song.artist)
                // If present, add a ref to the song to the Vector
                .and_modify(|songs| songs.push(&song))
                // Otherwise, add a new vector for that artist
                .or_insert(vec![&song]);
            by_artist
        })
}
```

A purely functional (i.e. no mutation) version looks like this:

```rs
fn artist_map<'a>(songs: &'a Vec<Song>) -> HashMap<&'a String, Vec<&'a Song>> {
    // For each song
    songs
        // Starting with a new HashMap, apply a function that takes
        // the hashmap (&HashMap) as its first argument, and the current
        // song (&Song) as the second argument
        .into_iter().fold(HashMap::new(), |by_artist, song| {
            // Look for the artist in the HashMap
            let songs = by_artist.get(&song.artist)
                .map_or(
                    // If not found, make a new vector (Vec<&Song>)
                    vec![song],
                    // If found, take a reference to the artist's songs (&Vec<&Song>)
                    |songs| {
                        songs
                            // Make an iterator over &&Song
                            .into_iter()
                            // Dereference to &Song
                            .map(|s| *s)
                            // Add &Song to the end of the iterator
                            .chain(iter::once(song))
                            // Collect them into a new Vec<&Song>
                            .collect()
                    }
                );
            // So now songs is always a NEW Vec<&Song>
            // Take the HashMap
            by_artist
                // Get an iterator over (&String, Vec<&Song>)
                .into_iter()
                // Add a new item for our current song (remember, this
                // the Vec<&Song> here will either contain JUST our
                // song or all of the songs that were previously in the
                // HashMap, plus ours.)
                .chain(iter::once((&song.artist, songs)))
                // Collect them into a new HashMap
                .collect()
        }
    )
}
```

Just as an aside, you can also do the slightly less functional thing of
modifying the dictionary as you go in JavaScript:

```js
const artistMap = (songs) => {
    return songs.reduce((byArtist, song) => {
        if (byArtist.hasOwnProperty(song.artist)) {
            byArtist[song.artist].push(song);
        } else {
            byArtist[song.artist] = [song];
        }
        return byArtist
    }, {})
}
```

You _cannot_ do this with lambdas in Python, because Python does not allow
multiple expressions in lambdas, and methods to update dictionaries in Python
do not return the updated dictionary. If you wanted to, though, you could get
around this with an accessory function:

```py
def _artist_reducer(by_artist: t.Dict[str, Song], song: Song) -> t.Dict[str, Song]:
    by_artist.setdefault(song.artist, [])
    by_artist[song.artist].append(song)
    return by_artist


def artist_map(songs: t.Iterable[Song]) -> t.Dict[str, t.List[Song]]:
    return reduce(_artist_reducer, songs, {})
```

That being said, the performance costs in Python and JS of constructing
the dict/object on each iteration are almost certainly not going to be
the bottleneck in any real system, so I tend to stick to doing things
the purely functional way unless there's a good reason not to.

## An Aside on Functional Programming in Python

Indeed, the [original plan][guido on reduce] for Python 3 was to drop
_all_ of the functional builtins: `lambda`, `map()`, `filter()`,
and `reduce()`.

Regarding `reduce()` in particular, Guido says:

> This is actually the one I've always hated most, because, apart from a
> few examples involving + or *, almost every time I see a reduce() call
> with a non-trivial function argument, I need to grab pen and paper to
> diagram what's actually being fed into that function before I
> understand what the reduce() is supposed to do. So in my mind,
> the applicability of reduce() is pretty much limited to
> associative operators, and in all other cases it's better to write
> out the accumulation loop explicitly

And to be fair, I've definitely seen (and written) overly complicated
reducers. However, I think that any good code review should point out
when a `reduce()` function gets too complicated, and that that complexity
can be easily managed with comments and named functions.

The main argument against the functional array functions/methods in Python
seems to be that _comprehensions do the job just fine, thank you very much_.
I do think comprehensions are an interesting feature of Python, and that
they can be really convenient. One of the examples given in
[Guido's post][guido on reduce] is the combination of generator
comprehensions with the `any()` and `all()` functions, which can indeed
look very nice:

```py
assert all(x > 0 for x in range(1, 20))
```

However, let's consider working on a nested list:

```py
nested = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
```

You might start out trying something like this:

```py
assert all(
    x > 0 for x in y for y in nested
)
```

But, because `y for y in nested` is just three lists, the `x` variable
is also referring to a given sub-list.

What you wind up having to do is:

```py
assert all(
    y > 0 for x in nested for y in x
)
```

To me, that is much less readable than the equivalent functional version:

```py
from itertools import chain

flatten = chain.from_iterable

assert all(
    map(lambda x: x > 0, flatten(nested))
)
```



[guido on reduce]: https://www.artima.com/weblogs/viewpost.jsp?thread=98196
