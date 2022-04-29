table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        draft -> Bool,
        time -> Timestamp,
    }
}
