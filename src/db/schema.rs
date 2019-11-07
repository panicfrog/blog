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
        topic_id -> Unsigned<Integer>,
        sid -> Nullable<Unsigned<Integer>>,
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
    topics (topic_id) {
        topic_id -> Unsigned<Integer>,
        title -> Varchar,
        content -> Varchar,
        user_id -> Unsigned<Integer>,
        category_id -> Unsigned<Integer>,
        create_time -> Timestamp,
        update_time -> Nullable<Timestamp>,
        delete_time -> Nullable<Timestamp>,
    }
}

table! {
    topics_tags (topic_tag_id) {
        topic_tag_id -> Unsigned<Integer>,
        topic_id -> Unsigned<Integer>,
        tag_id -> Unsigned<Integer>,
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

joinable!(comments -> topics (topic_id));
joinable!(topics -> categorys (category_id));
joinable!(topics -> users (user_id));
joinable!(topics_tags -> tags (tag_id));
joinable!(topics_tags -> topics (topic_id));

allow_tables_to_appear_in_same_query!(
    categorys,
    comments,
    tags,
    topics,
    topics_tags,
    users,
);
