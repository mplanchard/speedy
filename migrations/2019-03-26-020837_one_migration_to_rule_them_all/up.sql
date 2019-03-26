-- Your SQL goes here

CREATE TABLE IF NOT EXISTS posts (
    id INTEGER NOT NULL PRIMARY KEY,
    title TEXT NOT NULL UNIQUE,
    body TEXT NOT NULL
);


CREATE TABLE IF NOT EXISTS tags (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);


CREATE TABLE IF NOT EXISTS posts_tags (
    id INTEGER NOT NULL PRIMARY KEY,
    post_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (post_id) REFERENCES posts (id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION,
    FOREIGN KEY (tag_id) REFERENCES tags (id)
        ON DELETE NO ACTION
        ON UPDATE NO ACTION
);

CREATE UNIQUE INDEX idx_posts_tags ON posts_tags (post_id, tag_id);
CREATE INDEX idx_posts_tags_post_id ON posts_tags (post_id);
CREATE INDEX idx_posts_tags_tag_id ON posts_tags (tag_id);
