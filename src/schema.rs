// @generated automatically by Diesel CLI.

diesel::table! {
    medias (id) {
        id -> Integer,
        #[max_length = 255]
        file_name -> Varchar,
        #[max_length = 255]
        url -> Varchar,
        #[max_length = 255]
        path -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        body -> Text,
        published -> Bool,
        user_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        roles -> Longtext,
    }
}

diesel::joinable!(posts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    medias,
    posts,
    users,
);
