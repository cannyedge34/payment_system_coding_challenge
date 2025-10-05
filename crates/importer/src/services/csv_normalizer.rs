use crate::entities::merchants::{DisbursementFrequency, Merchant};
use chrono::NaiveDate;
use csv::StringRecord;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use uuid::Uuid;

pub fn read_and_normalize_merchants_from_csv(path: &str) -> Result<Vec<Merchant>, Box<dyn Error + Send + Sync>> {
    println!("Trying to open CSV at path: {}", path);

    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(buf_reader);

    let mut merchants = Vec::new();

    for result in reader.records() {
        let record = result?;
        merchants.push(parse_merchant_record(&record)?);
    }

    Ok(merchants)
}

fn parse_merchant_record(record: &StringRecord) -> Result<Merchant, Box<dyn Error + Send + Sync>> {
  let frequency = match record[4].to_uppercase().as_str() {
    "DAILY" => DisbursementFrequency::Daily,
    "WEEKLY" => DisbursementFrequency::Weekly,
    other => return Err(format!("Unknown disbursement_frequency: {}", other).into()),
  };

    Ok(Merchant {
        id: Uuid::parse_str(&record[0])?,
        merchant_reference: record[1].to_string(),
        email: record[2].to_string(),
        live_on: NaiveDate::parse_from_str(&record[3], "%Y-%m-%d")?,
        disbursement_frequency: frequency,
        minimum_monthly_fee: (record[5].parse::<f64>()? * 100.0).round() as i32,
    })
}