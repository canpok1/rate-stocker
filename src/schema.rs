// @generated automatically by Diesel CLI.

diesel::table! {
    daily_rates (rate_type_id, rate_date) {
        rate_type_id -> Unsigned<Tinyint>,
        rate_date -> Date,
        closing_rate -> Decimal,
        opening_rate -> Decimal,
        high_rate -> Decimal,
        low_rate -> Decimal,
        volume -> Decimal,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::table! {
    rate_types (id) {
        id -> Unsigned<Tinyint>,
        #[max_length = 255]
        type_name -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::joinable!(daily_rates -> rate_types (rate_type_id));

diesel::allow_tables_to_appear_in_same_query!(
    daily_rates,
    rate_types,
);
