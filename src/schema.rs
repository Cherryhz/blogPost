
table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        content -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);