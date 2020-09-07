title: Extending Rust Builtins: Adding an Iterator Method
slug: extending-rust-builtins-iterator-method
created: 2020-09-07
updated: 2020-09-07
tags: rust
summary: Rust's trait system makes it easy to add functionality to existing types. Here we explore extending Iterators with a new method, so that we can use our new method in Iterator chains just like .filter(), .map(), or .take().


# Extending Rust Builtins: Adding an Iterator Method

[Iterators](https://doc.rust-lang.org/std/iter/trait.Iterator.html) are one of Rust's nicest features, allowing you to easily write
lazily evaluated, declarative code. However, there are occasional times when
I find myself wishing for some iterator method that isn't provided on the
default trait. Unlike many languages, Rust makes it easy to extend builtins
safely. In this post, we'll explore adding a `.sorted_by()` Iterator method,
allowing you to provide a sorting function in your iterator chain.

Note: the solution here is largely for the sake of example. Please read
the caveats in the conclusions before using this extension yourself.


## Defining the Problem

The first step is to consider what we want, which is something equivalent
to the existing [.sort\_by()](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.sort_by) function available for `Vec`, which takes a
closure used to compare the items and uses them to sort the vec in place.
We'd like our new method to be usable in an iterator chain, so we can
do something like:

    type Point = (usize, usize);
    
    fn shift_points_right_and_sort() -> Vec<Point> {
        // A sequence of (x, y) coordinate pairs
        let points = vec![(1, 1), (2, 3), (7, 8), (1, 9)];
        points
            .iter()
            // multiply all x-values by two
            .map(|p| (p.0 * 2, p.1))
            // sort the points by x-value
            .sorted_by(|a, b| a.0.cmp(&b.0))
            // and then collect them into a new vec
            .collect()
    }

So what we need for this is the following:

-   [ ] A method that, given an input iterator, returns a sorted output iterator
-   [ ] An extension to the builtin Iterator that lets us use this anywhere we're
    already using an iterator


## How to Get What We Need

First, let's look at the `.sort_by()` signature for a `Vec<T>`, so we can use it
as the base for our solution:

    pub fn sort_by<F>(&mut self, compare: F) where
        F: FnMut(&T, &T) -> Ordering

The main difference will be that, since we intend to use this in a functional
iterator chain, we will take non-mutable ownership of `self`, rather than
taking a mutable reference to `self`.

We can now work backwards to figure out what we need.

In order to define any new generic method on any existing type, regardless
of whether that type is a builtin, a custom type, or a third-party type,
we'll need a [trait](https://doc.rust-lang.org/book/ch10-02-traits.html) with a [blanket implementation](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods).

Using non-compiling pseudo-code, we know we want something like:

    // We need a trait that extends the Iterator trait and adds a sorted_by() method
    trait SortedByExtension: Iterator {
        fn sorted_by<F>(self, compare: F) -> AnIteratorOverSortedValues
            // Item is an associated type on the Iterator trait, representing
            // the type of value the iterator iterates over. We can reference
            // that type with Self::Item.
            where F: FnMut(&Self::Item, &Self::Item) -> Ordering;
    }
    // We also need a blanket implementation for any iterator
    impl<I> SortedByExtension for I where I: Iterator
    {
        fn sorted_by<F>(self, compare: F) -> AnIteratorOverSortedValues
            where F: FnMut(I::Item, &I::Item) -> Ordering
        {
            // create & return a new iterator over sorted values
        }
    }

So now we just need to figure out how to return an iterator over sorted
values. Note that the Iterator trait is a *trait*, not a concrete type,
so we can't return that. We have a couple of options:

1.  Make our own type that implements Iterator
2.  Use an existing type that implements Iterator

We'll look at both of these options below.


## Solution One: Custom Iterator

Let's look at writing our own customer Iterator first, because this is the more
general-purpose solution. We can create a concrete type that is constructed
from an iterator and, when iterated over, yields sorted values:

    // This struct will keep track of our iteration context for
    // a given item type.
    struct SortedByIterator<T> {
        items: Vec<T>,
    }
    // We can define a convenient constructor
    impl<T> SortedByIterator<T> {
        fn new<I, F>(iter: I, compare: F) -> Self
        where
            I: Iterator<Item=T>,  // I is an iterator over items of type T
            F: FnMut(&T, &T) -> Ordering,  // F is our comparison function
        {
            // Create a vector sorted like we want it
            let mut items: Vec<T> = iter.collect();
            items.sort_by(compare);
            // We reverse it so we can get items off it with a simple pop()
            items.reverse();
            // And return the struct containing the vector
            SortedByIterator { items }
        }
    }
    // Implement Iterator for our struct
    impl<T> Iterator for SortedByIterator<T> {
        // We iterate over items of the same type as contained in our
        // SortedByIterator
        type Item = T;
        // Simply pop items off of the vec to iterate. pop() will return
        // None when we hit the end, signifying the end of iteration
        fn next(&mut self) -> Option<Self::Item> {
            self.items.pop()
        }
    }

We can then use this iterator in our blanket impl:

    trait SortedByExtension: Iterator {
        fn sorted_by<F>(self, compare: F) -> SortedByIterator<Self::Item>
            where
                F: FnMut(&Self::Item, &Self::Item) -> Ordering;
    }
    
    impl<I> SortedByExtension for I where I: Iterator
    {
        fn sorted_by<F>(self, compare: F) -> SortedByIterator<Self::Item>
            where
                F: FnMut(&I::Item, &I::Item) -> Ordering
        {
            // All we need to do for our impl is return our new iterator type
            SortedByIterator::new(self, compare)
        }
    }

And that's it! Now all we need to do to use `.sorted_by()` for any Iterator
is ensure we've imported it with `use whatever::SortedByExtension`.


## Solution Two: Existing Iterator

Note in our pseudocode example that we just wanted `AnIteratorOverSortedValues`.
We can get this without writing our own custom iterator by taking advantage of
a builtin type that implements iterator: [std::vec::IntoIter](https://doc.rust-lang.org/std/vec/struct.IntoIter.html), which is the
iterator type that `Vec` uses to satisfy the [IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) trait. Looking at
the [implementation signature](https://doc.rust-lang.org/std/vec/struct.IntoIter.html#impl-Iterator) of Iterator for `std::vec::IntoIter`, we see that
it does indeed give us `Iterator<T>`, or `AnIteratorOverSortedValues`, provided
we construct it from a vec of sorted values.

So, we can avoid needing to make a custom iterator by rewriting our extension
trait and blanket implementation as follows:

    use std::vec::IntoIter;
    
    trait SortedByExtension: Iterator {
        // Since IntoIter<Self::Item> satisfies our requirements, we
        // can specify it as our output type here.
        fn sorted_by<F>(self, compare: F) -> IntoIter<Self::Item>
            where
                F: FnMut(&Self::Item, &Self::Item) -> Ordering;
    }
    
    // This implementation applies to anything that implements the Iterator trait
    impl<I> SortedByExtension for I where I: Iterator
    {
        fn sorted_by<F>(self, compare: F) -> IntoIter<Self::Item>
            where
                F: FnMut(&I::Item, &I::Item) -> Ordering
        {
            // Here we just need to make a new vector, sort it, and
            // return its IntoIter.
            let mut items: Vec<Self::Item> = self.collect();
            items.sort_by(compare);
            items.into_iter()
        }
    }

Solution two requires less code than solution one, and is slightly more
efficient, due to there not being any need to reverse the iterator. However,
it's important to understand solution one, because not every context in which
you'd want to extend a builtin type can be covered so easily with existing
builtin types and methods. If you don't understand how to generically construct
the type that you need yourself, it'll be hard to figure out whether there's
a builtin that satisfies your needs.


## Conclusions

Either of our solutions satisfy our original requirements:

-   [X] A method that, given an input iterator, returns a sorted output iterator
-   [X] An extension to the builtin Iterator that lets us use this anywhere we're
    already using an iterator

They both illustrate one of Rust's greatest strengths, which is that the trait
system allows a huge amount of flexibility both in terms of defining generic
behavior and of extending existing behavior. Importantly, nothing is mutated
or overridden, and all extensions are opt-in. This system is what allows the
lovely library [colored](https://github.com/mackwic/colored) to provide such an elegant interface for adding color to
console output.

The process of extending builtin types also helps to demonstrate that builtin
types are not special! They are easy to introspect, and extending them works the
same way as extending your own code.

One might wonder why the standard `Iterator` trait doesn't define something like
`.sorted()` or `.sorted_by()`. My guess here is that they would break the
promise of lazy iterators. Since sorting requires collecting the iterator
into a vec or similar structure, it necessarily the inherent laziness
of iterators (while also requiring extra heap allocation). Because of this lack
of laziness, our `.sorted_by()` implementation is fundamentally incompatible
with infinite streams of data. For a proper implementation of our
SortedByExtension, we would probably be better off restricting it to take
and return [ExactSizeIterators](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html), which should generally be non-infinite.

