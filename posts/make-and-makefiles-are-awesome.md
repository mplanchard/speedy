title: Make and Makefiles are Awesome
slug: make-and-makefiles-are-awesome
created: 2020-05-16
updated: 2020-05-16
tags: make, makefiles, makefile, python, javascript, rust, programming, unix
summary: Make and its associated Makefiles have been around forever, and have gotten a bit of a bad rap due to people encountering and writing arcane, poorly documented examples. However, learning to use Make effectively can provide a significant benefit to modern software projects, enabling intelligent ordering and dependency management for common tasks that would be a nightmare to implement from scratch. In this article we'll cover the basics of Makefiles and how they can be useful in the context of a modern scripting language like JavaScript or Python.

# Make and Makefiles are Awesome

## Contents

- [Make and Makefiles are Awesome](#make-and-makefiles-are-awesome)
  - [Contents](#contents)
  - [Make History](#make-history)
  - [Make Setup](#make-setup)
  - [Makefile Anatomy](#makefile-anatomy)
    - [Makefile Rules](#makefile-rules)
    - [Makefile Special Targets](#makefile-special-targets)
    - [Makefile Functions](#makefile-functions)
    - [Makefile Variables](#makefile-variables)
    - [Makefile Rules Patterns](#makefile-rules-patterns)
      - [Directory Targets](#directory-targets)
      - [Aliases](#aliases)
      - [Empty Targets](#empty-targets)
  - [Make Python](#make-python)
    - [Virtual Environment Management](#virtual-environment-management)
    - [Different Environments, Different Requirements](#different-environments-different-requirements)
    - [Inside and Outside of Docker](#inside-and-outside-of-docker)
  - [Make JavaScript](#make-javascript)
    - [Node Modules](#node-modules)
  - [TypeScript Compiled Files](#typescript-compiled-files)
  - [Make Limitations](#make-limitations)
    - [Passing Arguments to Commands](#passing-arguments-to-commands)
    - [Becoming Arcane](#becoming-arcane)
  - [Other Benefits of Make](#other-benefits-of-make)

## Make History

Make has been around a _long_ time. According to [wikipedia][wikipedia make], Make
first appeared in the very first UNIX in 1976 and was written by [Stuart Feldman] at
Bell Labs. Here's a quote from him on its original inspiration, pulled from the
Wikipedia article above, which sources it as being from _[The Art of Unix Programming]_
by Eric S. Raymond.

> Make originated with a visit from Steve Johnson (author of yacc, etc.), storming into my office, cursing the Fates that had caused him to waste a morning debugging a correct program (bug had been fixed, file hadn't been compiled, cc *.o was therefore unaffected). As I had spent a part of the previous evening coping with the same disaster on a project I was working on, the idea of a tool to solve it came up. It began with an elaborate idea of a dependency analyzer, boiled down to something much simpler, and turned into Make that weekend. Use of tools that were still wet was part of the culture. Makefiles were text files, not magically encoded binaries, because that was the Unix ethos: printable, debuggable, understandable stuff.

This core principle of Make, which is to say making it easier to build software
artifacts (compiled files, releases, etc.) from source code files, remains
unchanged 44 years later.

The Make that you find most often on modern systems is GNU Make, an extension of
the original, created in [late 1987 or early 1988](https://stackoverflow.com/questions/34413436/what-was-the-first-release-date-of-gnu-make).
It added some quality of life improvements to the original, including
conditionals, builtin functions, and the ability to set and update variables.
GNU Make is now by far the most commonly used variant of Make, with most
large and small C projects using it. If you've ever run
`./configure && make && make install` to install some Linux application you
downloaded, you were probably using GNU Make (often via [Automake] and
[Autotools]). It is used to build the Linux Kernel itself, Firefox, Chromium,
and many other large projects.

As is the case with any general purpose tool that wraps a complex task, Make
hides a lot of complexity under the covers, and there are a number of ways to
tweak bits and pieces of that complexity. In addition, it carries a great deal of
backwards-compatible syntactical baggage as a result of being a piece of
software that is _44 years old_. As a result, it is possible to write (and
many of us have encountered) Makefiles that are virtually unintelligible
in their obscurity. This has led to a bit of a movement in modern software
development _away_ from Makefiles, except in the C/C++ world where it is
still going strong, and one will not infrequently see StackOverflow answers,
blogs, and comments about how you should NOT use Make for your modern
project, but instead use one of the many, many projects that have sprung up
to try to replace it, such as [CMake] (which you honestly might want to consider
for complex cross-platform builds of compiled code), [Fabric], [Grunt],
[npm], and others.

Unfortunately, most replacements for Make provide only a task runner,
lacking the sophisticated ability to skip unnecessary build steps that makes
Make so useful. Those that do give you a way to intelligently skip steps
often add a complexity burden approaching that of Make itself. "But," you
might say, "I'm writing Python! I don't run compiled files directly! I don't
need anything other than a task runner." While it might be true that Python's
`.pyc` files are not called directly, I hope I will show in this article that
there are still plenty of artifacts created during the life cycle of a Python
project, and how Make can make working with them much easier (while also
providing a task runner as needed).

## Make Setup

If you'd like to follow along, you'll need `make`. Make adds features with
some regularity, but the basic set of features we'll discuss in this article
should be readily available on most systems. They all work with the version of
`make` that comes on a modern Mac, 3.81, which is from 2006!

```raw
> make --version
GNU Make 3.81
Copyright (C) 2006  Free Software Foundation, Inc.
This is free software; see the source for copying conditions.
There is NO warranty; not even for MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.

This program built for i386-apple-darwin11.3.0
```

Versions of `make` are generally backwards compatible, so you can also use
newer versions. On a Mac, `brew install make` gets me `gmake` (GNU Make) 4.3.
This is the same version currently in the Arch Linux Core repo, while stable
Debian is on 4.2.

If you're on Windows, `make` should already be available in the WSL.

One **massive tip** before we get started. When you're writing your own
Makefiles, you can run them with `make --debug`, which will show you
_why_ Make decided or did not decide to run a given rule. This is extremely
helpful when trying to work out prerequisites.

## Makefile Anatomy

A Makefile is the one requirement for actually using Make. Generally
you'll want a Makefile in the root of your project, so that you can run
`make` commands from there.

According to the [docs](https://www.gnu.org/software/make/manual/html_node/Makefile-Contents.html#Makefile-Contents),
Makefiles may contain "explicit rules, implicit rules, variable definitions,
directives, and comments". We won't discuss implicit rules here, but we'll
touch on all the rest.

### Makefile Rules

**A Note on Copying and Pasting:** Makefiles use tabs rather than spaces. If
you copy and paste an example from this site into your editor it _may_ or
it _may not_ automatically convert the spaces to tabs for you. Most editors
are smart enough to know to use tabs in Makefiles, but you should check to be
sure.

A rule looks like this:

```Makefile
# Comments are preceded with hashes
target: prerequisite...
    command...
```

A rule says, "This target depends on these prerequisites. To create
the target, run this command." The collection of commands for a rule is
called its recipe.

Prerequisites are optional, and the command can span multiple lines.
Prerequisites can be file paths (relative to the makefile) or other targets
defined in the Makefile.

To run the rule, you run `make target`. Make will run the commands if:

- The target does not exist
- Any of the prerequisite(s) are newer than the target (determined by last
  modification time of the files)

If any of the prerequisites are other targets in the Makefile, `make` will
perform the same checks to determine if those need to run before running
the checks for the current rule.

If neither the target nor any prerequisite targets need to be run, `make target`
will say `Nothing to be done for 'target'` and quickly exit.
**This is the core value proposition of Makefiles.**

For example, when working on this blog, a `make static` rule compiles my
markdown posts into HTML if either my static site generator code
has or the markdown posts themselves have changed. Another rule, `make run`,
runs the server. It depends on `static`, and so it will automatically build
the site if needed. Yet another rule, `make watch`, watches the filesystem for
changes and runs `make run` if it encounters any. Since `make watch` calls
`make run`, which depends on `static`, it automatically "knows" when it only
needs to restart the server versus when it needs to also rebuild the site. At
every point in this dependency chain, I can rely on the `static` rule to
either run or not run as needed, meaning other, higher-level rules automatically
become intelligent, only compiling code when necessary.

You can see the Makefile for this blog [here](https://github.com/mplanchard/speedy/blob/master/Makefile).

Also don't forget, when writing rules, you can run with `make [target] --debug`
to see why Make chose whether or not to run a particular rule!

### Makefile Special Targets

Makefiles have a number of [special targets], but there are two that you should
definitely know.

The first is key to using `Make` as a task runner, and might look like:

```Makefile
.PHONY: test

test:
  cargo test
```

`.PHONY` tells Make that we don't actually expect a `test` file to be created
when we run `make test`. This is important, because otherwise if you create
a `test` directory or a file called `test`, make will think that the `test`
target is up-to-date and won't run anything.

The second is actually an implicit special target and is just the first
non-special target in the Makefile. Whatever that first target is will
be called whenever `make` is called with no arguments. Conventionally,
a target called `all` is used for this:

```Makefile
all: test build distribute

test:
  cargo test

build:
  cargo build --release

distribute:
  cargo publish
```

With the above Makefile, running `make` would run the test rule, then
the build rule, then the distribute rule.

### Makefile Functions

Make comes with a bunch of [builtin functions][functions]. The function
call syntax looks like `$(fn_name arg1,arg2,arg3)` (note the lack of spaces
between argument names).

You can use function results as targets or prerequisites, you can use them
to populate commands, and you can assign them to variables.

Two of the most convenient functions are `wildcard`, which allows you to
get a glob pattern list of matching files to assign to a variable, and
`shell`, which allows you to get the result of any arbitrary shell command.

### Makefile Variables

Variables can be assigned at the root of the Makefile or for an individual
rule. Variable assignment for a rule overrides any variable with the same
name defined in the Makefile root.

```Makefile
MY_VAR = initial

one:
  echo $(MY_VAR)  # echoes "initial"

# Define variables for a rule by creating a target-to-variable assignment map
two: MY_VAR = overridden  # they can override existing variables
two: MY_OTHER_VAR = other  # or define new ones
two: prereq
  echo $(MY_VAR)  # echoes "overridden
  echo $(MY_OTHER_VAR)  # echos "other"

three:  # variables overridden in rules are not overridden globally
  echo $(MY_VAR)  # echoes "initial"
```

There are [two basic variable assignment operators][variable flavors] in Makefiles,
`=` and `:=`, with the difference being the way nested variable expansions
in the variable values are handled. Nested variables in variables defined
with `:=` are expanded _when they are defined_, while nested variables
in variables defined with `=` are expanded _when they are used_.

In most cases either will work and you don't have to worry about it, but
`:=` is probably less likely to cause odd behavior in edge cases, so I would
recommend using it unless you need the behavior of `=`.

To illustrate:

```Makefile
# BASE isn't defined, but since we use `=` it won't be expanded until it is
# used, at which point BASE will be defined.
F_NAME_1 = $(BASE).js
# BASE isn't defined, and since we use `:=` we try to expand it immediately,
# and BASE gets inserted as an empty string.
F_NAME_2 := $(BASE).js
BASE = myfile

my_rule:
  echo $(F_NAME_1)  # echoes "myfile.js"
  echo $(F_NAME_2)  # echoes ".js"
```

There are also a number of [automatic variables] that are defined by Make
itself and which can be used throughout your Makefile. One important
automatic variable is `MAKE`, which is the Make executable used to run
the current Makefile. You can use this to recursively invoke Make if needed.
For example, you might want to use [watchexec] to enable automatic test rerunning
even if your framework doesn't support it:

```Makefile
test:
  pytest

watch:
  watchexec --exts py $(MAKE) test
```

### Makefile Rules Patterns

Now that we've covered the basics of functions and variables, we can consider
some more advanced rules.

#### Directory Targets

First, let's talk about directories as targets. You might want this if you
have a rule that takes some files in one directory and outputs files in another.
The first way to do this is to base the rule on the files in the directory,
and this often works just fine:

```Makefile
# Find input files in the foo-in directory.
INPUT_FILES = $(shell find foo-in -type f *.in)
# Ensure foo-out exists and find all output files in the dir
OUTPUT_FILES = $(shell mkdir -p foo-out && find foo-out -type f *.out)

$(OUTPUT_FILES): $(INPUT_FILES)
  # Note you need to use $$ to pass a $ to the shell, since it's a reserved
  # character in makefiles.
  for i in $(INPUT_FILES); do touch "foo-out/$$i.out"; done
```

This is usually sufficient. However, sometimes your output directory is _massive_,
so you don't want to spend the time finding files in it, or there's not really
a direct mapping between your input and output, so you only really care about running
some command when the prereqs change. To enable this, you can make a rule
whose target is the directory. Because adding or updating files in
a directory doesn't update _the directory itself_, I'll often use the following
pattern to ensure that Make can tell that the rule has been run:

```Makefile
build_dir: prereq_one prereq_two
  mkdir -p build_dir  # whatever command would create/update the directory
  touch build_dir  # mark _the directory itself_ as being updated
```

Touching the directory updates its file metadata so that its most recent
modification date will be when the rule was run.

#### Aliases

In some of the rules we've seen, we've got variable target names, or target
names that we might be able to easily remember a Make command for. To help
with that, we can make rules that have other rules' targets as their
prerequisites, creating what are essentially aliases.

As an example, let's consider a node project where I want to run `npm install`
any time `package.json` or `package-lock.json` change:

```Makefile:
.PHONY: install

install: node_modules  # this rule has no recipe, just a prereq.

# This rule defines how to satisfy the prereq for install
node_modules: package.json package-lock.json
  npm install
  touch node_modules  # ensure our target knows it's updated
```

Note that when defining _other_ targets, you SHOULD NOT use the alias
name as a prerequisite, because it will never actually exist and so will always trigger
a rebuild of the associated target. Instead, use the alias' dependency.
In other words, **aliases are for the Make command only, not for prerequisites!**
Nothing that needs to be in `.PHONY` should ever be a prerequisite, unless it's
a prerequisite for something that should always run.

#### Empty Targets

This pattern is mentioned in the [Make docs][empty targets]. Essentially,
you use an empty file as the target, touching it when the Make command is
run. This allows Make to track when the rule was last run, so it will only run
it again if prerequisites change.

Rather than using an empty target directly, I will often use an empty target
in a `.make` directory, combined with an alias, to avoid cluttering my project
directory with empty `test` files and such.

For example, expanding a bit on the example from the Make docs:

```Makefile
# The special .SILENT target tells Make not to print each step in the
# recipe before it runs it. This way, we only get our changed files as
# output.
.SILENT: .make/print
# While .make/print makes a target, print itself does not, so it's phony.
.PHONY: print

# Ensure the .make directory exists
.make:
  mkdir -p .make

# Print out only any dependencies that have changed since the last time
# the rule was run. If no dependencies have changed, do nothing
print: .make/print

# `print` is just an "alias" to this target.
.make/print: .make deps/*
  # $? is an automatic variable representing only changed prerequisites.
  echo $?
  touch .make/print
```

## Make Python

Okay, so we've talked a lot about Make in general, but let's talk about
how it can help us in some modern contexts. First up, a Python project.

One of the biggest pains in Python-land is managing your Python interpreter
and virtual environments. Here's a solution that ensures your venv exists
and makes it accessible to use in other commands:

### Virtual Environment Management

```Makefile
PYTHON := python3.8
# Save the venv activation command as a var for easy use
VENV = source venv/bin/activate;

.PHONY: install lint test

# Running `make` with no arguments will create the venv, install dependencies,
# run linting, and run tests.
all: lint test

# Ensure we have a .make directory for tracking empty targets (see
# Empty Targets above)
.make:
  mkdir -p .make

# Make a venv in a local `./venv` directory if it does not exist.
venv:
  $(PYTHON) -m venv venv
  touch venv

# Install requirements to the venv. If the venv doesn't exist, it will
# be created. Install will run if setup.py or requirements files change
install: .make/install
# Install is an alias to this rule
.make/install: .make  venv setup.py requirements.txt requirements.dev.txt
  $(VENV) pip install -r requirements.txt -r requirements.dev.txt
  $(VENV) python setup.py install -e .

# Other `python` commands now just need to depend on install. For example,
# this target will run tests, but first it will create the venv if needed,
# and install packages if setup.py or requirements.txt or requirements.dev.txt
# have changed.
test: .make/install
  $(VENV) pytest

# Similarly, `make lint` will also create a venv, install or update packages,
# and then run linting, but it will skip the first steps if it can
lint: .make/install
  $(VENV) mypy .
  $(VENV) pylint
  $(VENV) flake8
```

With the above configuration, someone working on your project for the first
time can check it out and _immediately_ run `make test` to verify everything
is working. Make will handle ensuring that all of the prerequisites are
satisfied. In addition, any time you run `make test`, any dependencies
you've added will be automatically installed before tests are run.

### Different Environments, Different Requirements

Let's say you want a different set of dependencies to be installed in CI. Maybe
you don't want to install dev dependencies since they take a bunch of time
and are only needed to do refactoring in VSCode or whatever, so you just
want to install test dependencies to keep CI fast. Assuming you have some
way of detecting that you're running in CI (an environment variable?), we
can do that:

```Makefile
# Env vars are passed into the Makefile, so we can check to see if our CI
# var is defined and then use the appropriate requirements file:
DEV_REQUIREMENTS = $(if $(CI),requirements.ci.txt,requirements.dev.txt)
# if function syntax is $(if condition,then,[else])

# From there you can use the exact same Makefile as above, and just update
# install as follows:

# Install requirements to the venv. If the venv doesn't exist, it will
# be created. Install will run if setup.py or requirements files change
install: .make/install
# Install is an alias to this rule
.make/install: .make  venv setup.py requirements.txt $(DEV_REQUIREMENTS)
  $(VENV) pip install -r requirements.txt -r requirements.dev.txt
  $(VENV) python setup.py install -e .
```

Obviously if your `requirements.ci.txt` doesn't specify the packages you need
for linting, `make lint` will fail, but as long as it contains what you need
for testing, `make test` will run just fine.

### Inside and Outside of Docker

Lots of folks mount their projects directly into Docker for development. This
can be a problem with the above patterns, since packages installed on your
local machine might not necessarily work in the Docker environment (since
the Docker environment may well be an entirely different OS).

Again, we can use some conditional magic to make this work with minimal
modifications:

```Makefile
# See https://stackoverflow.com/questions/20010199/how-to-determine-if-a-process-runs-inside-lxc-docker
# for a discussion on detecting if you're inside a Docker container. Here,
# IN_CONTAINER will be "true" if we are in a container, and the empty string
# otherwise.
IN_CONTAINER := $(shell [ -e /proc/1/cgroup ] && grep -q docker /proc/1/cgroup && echo "true" || echo "")
VENV_DIR := $(if $(IN_CONTAINER),venv-linux,venv)
VENV := source $(VENV_DIR)/bin/activate;

# From there, we just update our venv and install rules like:

# Make a venv in VENV_DIR if it does not exist
$(VENV_DIR):
  $(PYTHON) -m venv venv
  touch venv

# Install requirements to the venv. If the venv doesn't exist, it will
# be created. Install will run if setup.py or requirements files change
install: .make/install
# Install is an alias to this rule
.make/install: .make $(VENV_DIR) setup.py requirements.txt requirements.dev.txt
  $(VENV) pip install -r requirements.txt -r requirements.dev.txt
  $(VENV) python setup.py install -e .
```

## Make JavaScript

Make is also really helpful in working on JavaScript/TypeScript projects.

### Node Modules

As with Python, Make can help to ensure that your local JS environment
is up to date before running tests or running code:

```Makefile

.PHONY: install test

# Run an npm install if we need to update our node_modules directory
install: node_modules

# We need to update our node_modules if package.json or package-lock.json
# have changed.
# We touch node_modules when we're finished b/c just adding or removing
# things from a directory does not change the last-updated timestamp on
# the directory itself. Touch does this for us.
node_modules: package.json package-lock.json
  npm install
  touch node_modules

# This will run our tests
# It will run an npm install only if necessary, i.e. if package.json or
# package-lock.json have changed
test: node_modules
  npx --no-install jest tests
```

At this point just running `make test` will handle setting up the project if needed,
and on future runs, will automatically install any new dependencies before
running tests. You can of course add similar rules for watching files for
tests and anything else you'd like.

## TypeScript Compiled Files

Let's say your TS files are compiled into a `built` directory. You can
make a `build` target that creates them as needed:

```Makefile
BUILT_FILES := $(shell find built -type f -name *.js)

.PHONY: build

# We're assuming we also have the `node_modules` target from above.

# Install or update packages if needed, clean the build directory, and
# compile TypeScript files.
build: $(BUILT_FILES)

# `build` is just an alias for this rule.
$(BUILT_FILES): node_modules src/**/*.ts
  rm -rf built
  tsc

# You can then require your built files to exist for other things like
# running tests or running a local server if needed:
run: $(BUILT_FILES)
  npm run-script serve
```

Running `make build` will compile your TypeScript files into JavaScript files
(but only if any of the TypeScript files have changed), and running
`make run` will start your local server, again compiling your TypeScript
files into JavaScript files first, but only if necessary.

## Make Limitations

### Passing Arguments to Commands

There are of course some limitations to Make. One that you'll always need
to work around when using Make as a task runner is that you cannot directly
pass arguments to Make commands. However, because any variables defined on
the CLI when you _call_ `make` are passed into the Makefile, there is a relatively
elegant workaround:

```Makefile
.PHONY: test

# Run tests for the project
test:
  cargo test $(ARGS)
```

If `ARGS` is not defined, this just calls `cargo test`. Otherwise, it sticks
whatever is in ARGS right there on the call to `cargo test`, so you can do
something like:

```sh
make test ARGS="--test-threads 5 src/lib.rs"
```

And it will run:

```sh
cargo test --test-threads 5 src/lib.rs
```

### Becoming Arcane

There are a lot of things that can make a Makefile start to drift towards
unintelligibility. It requires some discipline, but I hope that you've seen in
this article that, with adequate explanatory comments,
Makefiles are not too hard to read!

If you're on a team using Make, please speak up and ensure that your teammates
are documenting their Makefiles in a way that is as much as possible geared
towards new users and people unfamiliar with Make. When people's first
experience is with a poorly documented, obtuse Makefile, it often turns them
off of the idea of Make altogether.

## Other Benefits of Make

There are a lot of utilities and benefits of Make that we haven't discussed.
This section lists some other niceties that you get for free if you use Make,
to serve as a jumping off point for further reading.

- [Automatic parallelism](https://www.gnu.org/software/make/manual/html_node/Parallel.html)
  with `make -j [N]`, where the optional _N_ is the number of jobs to run in
  parallel. Make automatically retains rule ordering based on prerequisites and
  provides options to [synchronize output](https://www.gnu.org/software/make/manual/html_node/Parallel-Output.html#Parallel-Output)
  to avoid the [parallel output problem](https://twitter.com/davidlohr/status/288786300067270656?s=20)
- [Automatic cleanup](https://www.gnu.org/software/make/manual/html_node/Interrupts.html#Interrupts)
  of partially written files if interrupted (e.g. by `Ctrl-c`)
- [Configurable automatic cleanup](https://www.gnu.org/software/make/manual/html_node/Errors.html#Errors)
  of targets when a rule's commands fail
- [Ability to specify intermediate targets](https://www.gnu.org/software/make/manual/html_node/Chained-Rules.html#Chained-Rules),
  which will be deleted after the build
- [Ability to use any shell for recipes](https://www.gnu.org/software/make/manual/html_node/One-Shell.html),
  including Python, JS, Bash, zsh, etc. (Note: unfortunately this feature was
  released in GNU Make 3.8.2, from 2010, and not in 3.8.1, from 2006, which
  is the version found by default on MacOS. If you want to use this feature on,
  a Mac, you'll need to install a more recent version of GNU Make.)

[Automake]: https://www.gnu.org/software/automake/
[automatic variables]: https://www.gnu.org/software/make/manual/html_node/Automatic-Variables.html#Automatic-Variables
[Autotools]: https://www.gnu.org/software/automake/manual/html_node/Autotools-Introduction.html
[CMake]: https://cmake.org/cmake/help/v3.17/guide/tutorial/index.html
[empty targets]: https://www.gnu.org/software/make/manual/html_node/Empty-Targets.html#Empty-Targets
[gnu make release date]: https://stackoverflow.com/questions/34413436/what-was-the-first-release-date-of-gnu-make
[grunt]: https://gruntjs.com/
[Fabric]: https://www.fabfile.org/
[functions]: https://www.gnu.org/software/make/manual/html_node/Functions.html#Functions
[npm]: https://docs.npmjs.com/
[special targets]: https://www.gnu.org/software/make/manual/html_node/Special-Targets.html#Special-Targets
[sphinx]: https://www.sphinx-doc.org/en/stable/
[Stuart Feldman]: https://en.wikipedia.org/wiki/Stuart_Feldman
[The Art of Unix Programming]: https://www.hpb.com/products/the-art-of-unix-programming-9780131429017
[variable flavors]: https://www.gnu.org/software/make/manual/html_node/Flavors.html#Flavors
[watchexec]: https://github.com/watchexec/watchexec
[wikipedia make]: https://en.wikipedia.org/wiki/Make_(software)
