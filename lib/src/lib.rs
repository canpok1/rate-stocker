pub mod datetime;
pub mod models;
pub mod schema;

use diesel::{mysql::MysqlConnection, prelude::*};
use dotenvy::dotenv;
use models::NewDailyRate;
use std::env;

use crate::models::{DailyRate, NewRateType, RateType};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn select_rate_type(conn: &mut MysqlConnection, pair: &str) -> Option<RateType> {
    use crate::schema::rate_types::dsl::*;

    rate_types
        .filter(type_name.eq(pair))
        .select(RateType::as_select())
        .first(conn)
        .optional()
        .expect("Error loading rate_types")
}

pub fn create_rate_type(conn: &mut MysqlConnection, pair: &str) -> RateType {
    use crate::schema::rate_types;

    let new_rate_type = NewRateType { type_name: pair };

    diesel::insert_into(rate_types::table)
        .values(&new_rate_type)
        .execute(conn)
        .expect("Error saving new rate_types");

    select_rate_type(conn, pair).unwrap()
}

pub fn select_daily_rate(
    conn: &mut MysqlConnection,
    type_id: &u64,
    date: &chrono::NaiveDate,
) -> Option<DailyRate> {
    use crate::schema::daily_rates::dsl::*;

    daily_rates
        .filter(rate_type_id.eq(type_id).and(rate_date.eq(date)))
        .select(DailyRate::as_select())
        .first(conn)
        .optional()
        .expect("Error loading daily_rates")
}

pub fn select_daily_rates(
    conn: &mut MysqlConnection,
    type_id: &u64,
    base_date: &chrono::NaiveDate,
    limit: i64,
) -> Vec<DailyRate> {
    use crate::schema::daily_rates::dsl::*;
    daily_rates
        .filter(rate_type_id.eq(type_id).and(rate_date.le(base_date)))
        .select(DailyRate::as_select())
        .order(rate_date.desc())
        .limit(limit)
        .load(conn)
        .expect("Error loading daily_rates")
}

pub fn create_daily_rate(conn: &mut MysqlConnection, new_daily_rate: &NewDailyRate) -> DailyRate {
    use crate::schema::daily_rates;

    diesel::insert_into(daily_rates::table)
        .values(new_daily_rate)
        .execute(conn)
        .expect("Error saving new daily_rate");

    select_daily_rate(conn, new_daily_rate.rate_type_id, new_daily_rate.rate_date).unwrap()
}

pub fn upsert_daily_rate(conn: &mut MysqlConnection, new_daily_rate: &NewDailyRate) {
    use crate::schema::daily_rates::dsl::*;

    let old = select_daily_rate(conn, new_daily_rate.rate_type_id, new_daily_rate.rate_date);
    match old {
        Some(_old) => {
            diesel::update(new_daily_rate)
                .set((
                    closing_rate.eq(&new_daily_rate.closing_rate),
                    high_rate.eq(&new_daily_rate.high_rate),
                    low_rate.eq(&new_daily_rate.low_rate),
                    volume.eq(&new_daily_rate.volume),
                ))
                .execute(conn)
                .expect("Error saving new rate_types");
        }
        None => {
            create_daily_rate(conn, new_daily_rate);
        }
    };
}
