table! {
    images (id) {
        id -> Nullable<Integer>,
        name -> Text,
        postid -> Integer,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        draft -> Bool,
        time -> Timestamp,
        header -> Nullable<Text>,
    }
}

joinable!(images -> posts (postid));

allow_tables_to_appear_in_same_query!(
    images,
    posts,
);
