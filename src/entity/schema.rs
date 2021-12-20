table! {
    account (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

table! {
    series (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        rating -> Int4,
    }
}

table! {
    session (id, id_account) {
        id -> Int4,
        id_account -> Int4,
    }
}

joinable!(session -> account (id_account));

allow_tables_to_appear_in_same_query!(
    account,
    series,
    session,
);
