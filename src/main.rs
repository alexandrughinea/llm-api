use std::{convert::Infallible, io::Write, path::PathBuf};

use actix_cors::Cors;
use actix_web::web::Json;
use actix_web::{http, middleware, web, App, HttpResponse, HttpServer, Responder};
use llm::{InferenceError, Model};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::utils::match_model_architecture;

mod config;
mod utils;

#[derive(Debug, Deserialize)]
struct GenerateRequest {
    prompt: String,
}

#[cfg(feature = "server")]
pub struct AppState {
    pub model: Box<dyn Model>,
    pub config: Config,
}

pub async fn server_info_handler(data: web::Data<AppState>) -> impl Responder {
    let result = format!(
        "Model: {}, Architecture: {}, Token count",
        &data.config.llm_model, &data.config.llm_model_architecture
    );
    HttpResponse::Ok().body(result)
}

fn run_inference_session(
    config: &Config,
    model: &Box<dyn Model>,
    prompt: String,
) -> Result<String, InferenceError> {
    let mut result_tokens = String::new();
    let mut prompt_tokens = String::new();
    let mut inference_session = model.start_session(Default::default());
    let inference_session_result = inference_session.infer::<Infallible>(
        model.as_ref(),
        // Input:
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: (&*prompt).into(),
            parameters: Option::from(&llm::InferenceParameters::default()),
            play_back_previous_tokens: false,
            maximum_token_count: Some(config.llm_inference_max_token_count),
        },
        // Output:
        &mut Default::default(),
        |response| {
            print!("{response}");
            std::io::stdout().flush().unwrap();

            Ok(())
        },
    );

    println!(
        "Prompt: {}\nChar size: {}\nInference result: {}\nInference char size: {}",
        prompt_tokens,
        prompt_tokens.len(),
        result_tokens,
        result_tokens.len()
    );

    match inference_session_result {
        Ok(_) => Ok(result_tokens),
        Err(err) => Err(err),
    }
}

pub async fn generate_handler(
    data: web::Data<AppState>,
    body: Json<GenerateRequest>,
) -> HttpResponse {
    let result = run_inference_session(&data.config, &data.model, body.prompt.clone()).unwrap();

    HttpResponse::Ok().body(result)
}

pub async fn health_handler() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
#[cfg(feature = "server")]
async fn main() -> std::io::Result<()> {
    let config: Config = Config::init();
    let model_path = PathBuf::from(&config.llm_model);
    let now = std::time::Instant::now();
    let model_architecture = match_model_architecture(&config.llm_model_architecture)
        .unwrap_or_else(|| {
            panic!(
                "Failed to find model architecture {} for model: {}.\n",
                config.llm_model_architecture, &config.llm_model
            );
        });

    let model = llm::load_dynamic(
        model_architecture,
        &model_path,
        Default::default(),
        llm::load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| {
        panic!(
            "Failed to load {} model from {:?}: {}",
            config.llm_model, model_path, err
        );
    });

    println!(
        "{} model ({}) has been started!\nElapsed: {}ms",
        config.llm_model,
        config.llm_model_architecture,
        now.elapsed().as_millis()
    );

    println!(
        "Starting server at https://{}:{}.\n",
        config.server_address, config.server_port
    );

    let config: Config = Config::init();
    let app_state = web::Data::new(AppState {
        model,
        config: config.clone(),
    });

    let complete_address = format!("{}:{}", config.server_address, config.server_port);
    let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    ssl_builder
        .set_private_key_file("certs/key.pem", SslFiletype::PEM)
        .unwrap();

    ssl_builder
        .set_certificate_chain_file("certs/cert.pem")
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::Compress::default())
            .wrap(
                Cors::default()
                    .allowed_origin(&config.allowed_origin)
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(config.max_age as usize),
            )
            .route("/", web::get().to(server_info_handler))
            .service(
                web::scope("/api")
                    .route("/generate", web::post().to(generate_handler))
                    .route("/health", web::get().to(health_handler)),
            )
    })
    .bind_openssl(complete_address, ssl_builder)?
    .run()
    .await
}
