use chrono::NaiveDate;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Merchant {
    pub id: Uuid,
    pub merchant_reference: String,
    pub live_on: NaiveDate,
    pub disbursement_frequency: DisbursementFrequency,
    pub minimum_monthly_fee: i32,
}

#[derive(Deserialize, Serialize, Display)]
#[serde(rename_all = "UPPERCASE")]
pub enum DisbursementFrequency {
    #[strum(to_string = "DAILY")]
    Daily,
    #[strum(to_string = "WEEKLY")]
    Weekly,
}