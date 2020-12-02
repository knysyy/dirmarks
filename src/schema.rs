use diesel::table;

table! {
    bookmarks (id) {
        id -> Integer,
        key -> Text,
        path -> Text,
        description -> Nullable<Text>,
    }
}
