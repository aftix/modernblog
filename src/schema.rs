table! {
    images (id) {
        id -> Nullable<Integer>,
        location -> Text,
        refs -> Integer,
        hash -> Text,
    }
}

table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        draft -> Bool,
        time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    images,
    posts,
);
