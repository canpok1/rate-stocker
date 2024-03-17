use bigdecimal::BigDecimal;
use diesel::prelude::*;

use crate::schema::{daily_rates, rate_types};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name=rate_types)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct RateType {
    pub id: u64,
    pub type_name: String,
}

#[derive(Insertable)]
#[diesel(table_name=rate_types)]
pub struct NewRateType<'a> {
    pub type_name: &'a str,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name=daily_rates)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct DailyRate {
    pub rate_type_id: u64,
    pub rate_date: chrono::NaiveDate,
    pub closing_rate: BigDecimal,
    pub opening_rate: BigDecimal,
    pub high_rate: BigDecimal,
    pub low_rate: BigDecimal,
    pub volume: BigDecimal,
}

#[derive(Insertable, Identifiable)]
#[diesel(table_name=daily_rates)]
#[diesel(primary_key(rate_type_id, rate_date))]
pub struct NewDailyRate<'a> {
    pub rate_type_id: &'a u64,
    pub rate_date: &'a chrono::NaiveDate,
    pub closing_rate: BigDecimal,
    pub opening_rate: BigDecimal,
    pub high_rate: BigDecimal,
    pub low_rate: BigDecimal,
    pub volume: BigDecimal,
}
