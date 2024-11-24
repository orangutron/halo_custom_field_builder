use dotenv::dotenv;
use std::env;

#[derive(Debug)]
struct Config {
    base_url: String,
    tenant: String,
    token_url: String,
    api_url: String,
    client_id: String,
    client_secret: String,
    source_file_name: String,
}
impl Config{
    fn load_config() -> Config {
        // Load environment variables from .env file
        dotenv().expect("Failed to load .env file");
        // Load configuration from environment variables into variables for normalization
        let base_url = format!("https://{}", env::var("BASE_URL")
            .expect("BASE_URL must be set")
            .trim_end_matches("/")
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_start_matches("www.")
            .to_string());
            // ensures...
            // 1. "https://" is at beginnig of URL
            // 2. "/" is not at end of URL
        let tenant = env::var("TENANT")
            .expect("TENANT must exist in .env file (even if blank)");
        let api_url = format!("{base_url}/api");
        let token_url = match &tenant.len() {
            0 => format!("{base_url}/auth/token"),
            _ => format!("{base_url}/auth/token?tenant={tenant}")
        };
        let client_id = env::var("CLIENT_ID")
            .expect("CLIENT_ID must be set");
        let client_secret = env::var("CLIENT_SECRET")
            .expect("CLIENT_SECRET must be set");
        let source_file_name = env::var("SOURCE_FILE_NAME")
            .expect("SOURCE_FILE_NAME must be set");
        
        
        Config {
            base_url,
            tenant,
            api_url,
            token_url,
            client_id,
            client_secret,
            source_file_name,
            }
    }
}
fn main() {
    let config = Config::load_config();
    println!("{:#?}", config);
}
