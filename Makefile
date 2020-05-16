POST_INPUT := $(wildcard posts/*.md)
POST_OUTPUT := $(wildcard static/posts/*.html)

.PHONY: posts run

all: run

# Phony target `posts`.
# Will only run if generated HTML is out of date
posts: $(POST_OUTPUT)

# Rule to generate HTML.
# Will only run if markdown files have been updated.
$(POST_OUTPUT): $(POST_INPUT)
	cargo run generate

# Rule to run the server.
# Will regenerate HTML if needed.
run: posts
	cargo run run
