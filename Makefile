POST_INPUT := $(wildcard posts/*.md)
POST_OUTPUT := $(wildcard static/posts/*.html)

.PHONY: posts run

all: run

posts: $(POST_OUTPUT)
$(POST_OUTPUT): $(POST_INPUT)
	cargo run generate


run: $(POST_OUTPUT)
	cargo run run
