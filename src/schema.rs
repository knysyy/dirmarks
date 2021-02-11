use diesel::table;

table! {
    bookmarks (id) {
        id -> Integer,
        key -> Text,
        path -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    histories (id) {
        id -> Integer,
        path -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    bookmarks,
    histories,
);
