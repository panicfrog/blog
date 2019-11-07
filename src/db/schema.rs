table! {
    categorys (category_id) {
        category_id -> Unsigned<Integer>,
        name -> Varchar,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
    }
}

table! {
    comments (comment_id) {
        comment_id -> Unsigned<Integer>,
        content -> Varchar,
        post_id -> Unsigned<Integer>,
        sid -> Nullable<Unsigned<Integer>>,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
    }
}

table! {
    posts (post_id) {
        post_id -> Unsigned<Integer>,
        title -> Varchar,
        content -> Text,
        user_id -> Unsigned<Integer>,
        category_id -> Unsigned<Integer>,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
    }
}

table! {
    posts_tags (post_tag_id) {
        post_tag_id -> Unsigned<Integer>,
        post_id -> Unsigned<Integer>,
        tag_id -> Unsigned<Integer>,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
    }
}

table! {
    tags (tag_id) {
        tag_id -> Unsigned<Integer>,
        name -> Varchar,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
    }
}

table! {
    users (user_id) {
        user_id -> Unsigned<Integer>,
        user_name -> Varchar,
        passwd -> Varchar,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
    }
}

joinable!(comments -> posts (post_id));
joinable!(posts -> categorys (category_id));
joinable!(posts -> users (user_id));
joinable!(posts_tags -> posts (post_id));
joinable!(posts_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    categorys,
    comments,
    posts,
    posts_tags,
    tags,
    users,
);
