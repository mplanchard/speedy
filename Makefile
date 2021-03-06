POSTS = $(wildcard posts/*.md)
SRC = $(shell \find src -type f -name *.rs)
TEMPLATES = $(shell \find templates -type f)

# If there is an error while executing a command to build a target,
# delete the built target to ensure that nothing gets corrupted and that
# the target will be rebuilt the next time make is run.
.DELETE_ON_ERROR:

.PHONY: deploy posts run watch

all: run

# Ensure we've built our Rust crate & generated our static site.
# Will only run if generated HTML is out of date
build: static

# Ensure that we've generated our static site.
static: $(SRC) $(POSTS) $(TEMPLATES)
	cargo run generate
	touch static

# Rule to run the server.
# Will regenerate HTML if needed.
run: static
	cargo run run

# Watch for changes and run the server.
# Upon changes, regenerate posts if necessary and then rerun the server.
watch:
	cargo watch --shell "$(MAKE) run"

# Deploy static site to Azure.
# note: deploying requires the `az` cli and the `azcopy` utility.
# `azcopy` can require running a `keyctl new_session` before running `azcopy login`
# on linux.
deploy: static
	azcopy sync './static' 'https://mplanchardspeedyblog.blob.core.windows.net/$$web' --recursive=true
	az cdn endpoint purge --resource-group mplanchardspeedyblog --profile-name speedyblogcdn --name mplanchardspeedyblog --content-paths '/*'
