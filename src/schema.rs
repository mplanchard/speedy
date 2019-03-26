table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
    }
}

table! {
    posts_tags (id) {
        id -> Nullable<Integer>,
        post_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    tags (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

joinable!(posts_tags -> posts (post_id));
joinable!(posts_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    posts,
    posts_tags,
    tags,
);
