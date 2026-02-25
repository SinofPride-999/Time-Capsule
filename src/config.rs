#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("Failed to read DATABASE_URL from env. Field must be set");

        let port = std::env::var("PORT")
            .expect("Failed to read PORT from env. Field must be set")
            .parse::<u16>()
            .expect("Failed to read PORT from env. Field must be set");

        Config {
            database_url,
            port,
        }
    }
}
