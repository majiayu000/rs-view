diesel::table! {
    accounts (id) {
        id -> Int4,
        exchange -> Varchar,
        symbol -> Varchar,
        pricedot -> Int4,
        amountdot -> Int4,
        info -> Varchar,
        api_key -> Varchar,
        secret_key -> Varchar,
        extra -> Varchar,
    }
}
