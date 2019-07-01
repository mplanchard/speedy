title: Stacking Calls to pytest's Parametrize
slug: stacking-calls-to-pytest-parametrize
created: 2018-02-17
updated: 2018-02-17
tags: pytest, python, testing, parametrize
summary: how to compose parametrizations for combinatoric testing

# Stacking Calls to pytest’s Parametrize

**NB:** this article was imported from [medium](https://medium.com/@mplanchard/stacking-calls-to-pytests-parametrize-d3cd4acc5611)

Hopefully you’re already familiar with pytest’s fantastic `parametrize()` decorator. If not, there’s a brief summary below. If you are, skip the summary and head straight to the interesting part!

Oh yeah, and remember it’s spelled parametrize (British), not parameterize (American). There has been [some discussion](https://github.com/pytest-dev/pytest/pull/3159) about adding an alias, but until they do, you’ll have to remember the alternative spelling if you’re American!

----

## Parametrize Basics

It allows you to define a series of values to be passed into your test function as parameters. Each member of the series corresponds to a new, isolated call to your test function (and each counts as its own test). Here’s a simple example:

```python
import pytest

@pytest.mark.parametrize('foo', ('a', 'b'))
def test_something(foo):
    print(foo)
```

Run this with pytest `-sv` (`-s` to disable capturing stdout, `-v` for verbose mode) and you should see something like the following:

```raw
collected 2 items

tests/decorators/test_generic.py::test_foo[a] a
PASSED
tests/decorators/test_generic.py::test_foo[b] b
PASSED
```

Notice that a test was generated for each parameter, that the parameters were passed in for the `foo` variable, and that pytest was nice enough to tell you, in the verbose output, which parameters were being passed to which call.

## The Problem

There are plenty of times when you want to pass multiple parametrized arguments to a test function, and sometimes, you want to ensure that each parameter for one argument gets run against each parameter for another. One way to do this is manually:

```python
import pytest

@pytest.mark.parametrize('foo, bar', (
    ('a', True),
    ('a', False),
    ('b', True),
    ('b', False)
))
def test_multi(foo, bar):
    print(foo, bar)
```

This certainly works. Running the above with pytest `-sv` yields:

```raw
collected 4 items

tests/decorators/test_generic.py::test_multi[a-True] a True
PASSED
tests/decorators/test_generic.py::test_multi[a-False] a False
PASSED
tests/decorators/test_generic.py::test_multi[b-True] b True
PASSED
tests/decorators/test_generic.py::test_multi[b-False] b False
PASSED
```

However, this starts to get pretty unwieldy when you’ve got three or more options for any of your parameters. If you’re trying to get every combination of `('a', 'b', 'c')` with `(True, False, None)` for example, you’ll need nine lines of parameter specification, and it’s pretty easy to forget one permutation or another.

Another option is to use `itertools.product()`, which yields the Cartesian product of the passed iterables. In other words, `itertools.product(('a', 'b'), (True, False))` gives me an iterable over `[('a', True), ('a', False), ('b', True), ('b', False)]`. We can use this iterable in `parametrize()` to ensure we call all of our combinations. So, the following is equivalent to our “brute-force” solution above:

```python
import itertools
import pytest

@pytest.mark.parametrize(
    'foo, bar', itertools.product(('a', 'b'), (True, False))
)
def test_multi(foo, bar):
    print(foo, bar)
```

And indeed, this results in the same console output when run with `pytest -sv` as our previous solution. However, it harms readability. Not everyone is going to know immediately what `product()` does, after all.

Luckily, there’s an even easier solution.

## Parametrize Stacking

Like any well behaved decorator, `parametrize()` can be stacked with other decorators. Conveniently, it can be stacked with other calls to `parametrize()`, too! When multiple parametrize decorators are present, the function will be run with each value of each parametrization with each value of every other parametrization. So, we can implement our multiple-parameter test this way:

```python
import pytest

@pytest.mark.parametrize('foo', ('a', 'b'))
@pytest.mark.parametrize('bar', (True, False))
def test_multi(foo, bar):
    print(foo, bar)
```

So much nicer! Here’s the output:

```raw
collected 4 items

tests/decorators/test_generic.py::test_multi[True-a] a True
PASSED
tests/decorators/test_generic.py::test_multi[True-b] b True
PASSED
tests/decorators/test_generic.py::test_multi[False-a] a False
PASSED
tests/decorators/test_generic.py::test_multi[False-b] b False
PASSED
```

So, we’ve managed to replace our extremely long brute-force specification and our confusing call to `itertools.product()` with something that reads more clearly and is easier to reason about. Tangentially, it also makes for easier code reviews when new parameters are added, since each one gets its own line.

----

Hopefully you’ve learned a new, interesting, and useful feature of pytest that helps you to go forth and write better tests! Maybe you think I’m crazy and that you can’t possibly get better than complex calls to `product()`. Either way, hopefully this was an enjoyable read.
