pub struct Config {
    pub mongo_uri: String,
    pub mongo_db: String,
    pub app_name: String,
    pub app_version: String,
    pub port: String,
}

impl Config {
    pub fn load() -> Self {
        let _ = dotenvy::dotenv();
        Self {
            mongo_uri: env("MONGO_URI", "mongodb://localhost:27017"),
            mongo_db: env("MONGO_DB", "mydatabase"),
            app_name: env("BACKEND_NAME", "axum-mongodb-starter"),
            app_version: env("BACKEND_VERSION", "0.0.1"),
            port: env("PORT", "8000"),
        }
    }
}

fn env(key: &str, fallback: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| fallback.to_string())
}
