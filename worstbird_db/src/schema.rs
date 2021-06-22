table! {
    bird (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        assetid -> Varchar,
        url -> Varchar,
        width -> Int4,
        height -> Int4,
    }
}

table! {
    worstbird_month (bird_id, month, year) {
        bird_id -> Int4,
        month -> Int4,
        year -> Int4,
        votes -> Int4,
    }
}

table! {
    worstbird_year (bird_id, year) {
        bird_id -> Int4,
        year -> Int4,
        votes -> Int4,
    }
}

joinable!(worstbird_month -> bird (bird_id));
joinable!(worstbird_year -> bird (bird_id));

allow_tables_to_appear_in_same_query!(bird, worstbird_month, worstbird_year,);
