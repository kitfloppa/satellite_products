// @generated automatically by Diesel CLI.

diesel::table! {
    satellite_data (id) {
        id -> Int4,
        satellite_id -> Int4,
        path -> Varchar,
    }
}

diesel::table! {
    satellites (id) {
        id -> Int4,
        name -> Varchar,
        tle1 -> Nullable<Varchar>,
        tle2 -> Nullable<Varchar>,
    }
}

diesel::joinable!(satellite_data -> satellites (satellite_id));

diesel::allow_tables_to_appear_in_same_query!(
    satellite_data,
    satellites,
);
