use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_address: String,
    pub server_port: u16,
    pub server_request_timeout: u64,
    pub machine_command_timeout: u64,
    pub network_public_ip_v4_services: (String, String),
    pub network_public_ip_v6_services: Option<(String, String)>,

    pub max_connections: u32,
    pub database_url: String,
    pub allowed_origin: String,
    pub max_age: u64,

    pub llm_model: String,
    pub llm_model_architecture: String,
    pub llm_inference_max_token_count: usize,
}

impl Config {
    pub fn init() -> Config {
        let _ = dotenv::load();

        let server_address = env::var("SERVER_ADDRESS")
            .expect("SERVER_ADDRESS must be specified")
            .parse::<String>()
            .unwrap();
        let server_port = env::var("SERVER_PORT")
            .expect("SERVER_PORT must be specified")
            .parse::<u16>()
            .unwrap();

        let max_connections = env::var("MAX_CONNECTIONS")
            .expect("MAX_CONNECTIONS must be specified")
            .parse::<u32>()
            .unwrap();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be specified")
            .parse::<String>()
            .unwrap();
        let allowed_origin = env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN must be specified");
        let max_age = env::var("MAX_AGE")
            .expect("MAX_AGE must be specified")
            .parse::<u64>()
            .unwrap();

        let server_request_timeout = env::var("SERVER_REQUEST_TIMEOUT_IN_SECONDS")
            .expect("SERVER_REQUEST_TIMEOUT_IN_SECONDS must be specified")
            .parse::<u64>()
            .unwrap();
        let machine_command_timeout = env::var("MACHINE_COMMAND_TIMEOUT_IN_SECONDS")
            .expect("MACHINE_COMMAND_TIMEOUT_IN_SECONDS must be specified")
            .parse::<u64>()
            .unwrap();

        let binding = env::var("NETWORK_PUBLIC_IP_V4_SERVICES")
            .expect("NETWORK_PUBLIC_IP_V4_SERVICES must be specified")
            .parse::<String>()
            .unwrap();
        let mut network_public_ip_v4_services_parts = binding.split(',');

        let network_public_ip_v4_services_main = network_public_ip_v4_services_parts
            .next()
            .expect("Main IPV4 service address not found")
            .to_string();
        let network_public_ip_v4_services_fallback = network_public_ip_v4_services_parts
            .next()
            .expect("Fallback IPV4 service address not found")
            .to_string();
        let network_public_ip_v4_services = (
            network_public_ip_v4_services_main,
            network_public_ip_v4_services_fallback,
        );

        let llm_model = env::var("LLM_MODEL")
            .expect("LLM_MODEL must be specified")
            .parse::<String>()
            .unwrap();
        let llm_model_architecture = env::var("LLM_MODEL_ARCHITECTURE")
            .expect("LLM_MODEL_ARCHITECTURE must be specified")
            .parse::<String>()
            .unwrap();

        let llm_inference_max_token_count = env::var("LLM_INFERENCE_MAX_TOKEN_COUNT")
            .expect("LLM_INFERENCE_MAX_TOKEN_COUNT must be specified")
            .parse::<usize>()
            .unwrap();

        Config {
            server_address,
            server_port,
            server_request_timeout,
            machine_command_timeout,
            network_public_ip_v4_services,
            network_public_ip_v6_services: None,
            max_connections,
            database_url,
            allowed_origin,
            max_age,
            //
            llm_model,
            llm_model_architecture,
            llm_inference_max_token_count,
        }
    }
}
