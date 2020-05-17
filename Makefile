POST_INPUT = $(wildcard posts/*.md)
POST_OUTPUT = $(wildcard static/**/*.html)

# If there is an error while executing a command to build a target,
# delete the built target to ensure that nothing gets corrupted and that
# the target will be rebuilt the next time make is run.
.DELETE_ON_ERROR:

.PHONY: posts run

all: run

# Will only run if generated HTML is out of date
posts: $(POST_OUTPUT)

# Rule to generate HTML.
# Will only run if markdown files have been updated.
$(POST_OUTPUT): $(POST_INPUT)
	cargo run generate

# Rule to run the server.
# Will regenerate HTML if needed.
run: $(POST_OUTPUT)
	cargo run run

watch: $(POST_OUTPUT)
	cargo watch --shell "$(MAKE) run"
