-- This file should undo anything in `up.sql`


DROP TABLE IF EXiSTS posts;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS posts_tags;

DROP INDEX IF EXISTS idx_posts_tags
DROP INDEX IF EXISTS idx_posts_tags_post_id
DROP INDEX IF EXISTS idx_posts_tags_tag_id
