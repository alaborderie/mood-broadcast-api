table! {
    auth (id) {
        id -> Int4,
        user_id -> Int4,
        login_timestamp -> Timestamptz,
        login_session -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        picture_url -> Varchar,
    }
}

table! {
    games (id) {
        id -> Int4,
        name -> Varchar,
        logo_url -> Varchar,
    }
}

table! {
    moods (id) {
        id -> Int4,
        user_id -> Int4,
        game_id -> Int4,
        begin_timestamp -> Timestamptz,
        end_timestamp -> Timestamptz,
    }
}

table! {
    friends (id) {
        id -> Int4,
        user_from_id -> Int4,
        user_to_id -> Int4,
        status -> Varchar,
        update_timestamp -> Timestamptz,
    }
}

joinable!(auth -> users (user_id));
joinable!(moods -> users (user_id));
joinable!(moods -> games (game_id));
joinable!(friends -> users (user_from_id));
// joinable!(friends -> users (user_to_id));
// Not working, need to find an alternative.

allow_tables_to_appear_in_same_query!(auth, users, games, moods, friends);
