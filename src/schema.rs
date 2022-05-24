table! {
    book_review (id) {
        id -> Int4,
        title -> Varchar,
        review -> Nullable<Text>,
        rating -> Float4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        author -> Varchar,
    }
}

table! {
    comment (id) {
        id -> Int4,
        username -> Varchar,
        content -> Text,
        review_id -> Int4,
    }
}

table! {
    one_user (username) {
        username -> Varchar,
        passcode -> Varchar,
    }
}

table! {
    review (id) {
        id -> Int4,
        title -> Varchar,
        #[sql_name = "review"]
        content -> Nullable<Text>,
        rating -> Float4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(comment -> review (review_id));

allow_tables_to_appear_in_same_query!(
    book_review,
    comment,
    one_user,
    review,
);
