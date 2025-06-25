use std::env;
#[derive(Debug, Clone)]
pub struct Settings {
    pub csv_path: String,
    pub database_url: String,
    pub kafka_brokers: String,
}

impl Settings {
    pub fn from_env() -> Self {
        let csv_path = env::var("CSV_PATH")
            .expect("CSV_PATH must be set in .env or .env.test");

        let database_url = env::var("IMPORTER_DATABASE_URL")
            .expect("DATABASE_URL must be set in .env or .env.test");

        let kafka_brokers = env::var("KAFKA_BROKERS")
            .expect("KAFKA_BROKERS must be set in .env or .env.test");

        Settings {
            csv_path,
            database_url,
            kafka_brokers,
        }
    }
}
