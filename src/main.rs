use std::{convert::Infallible, io::Write, path::PathBuf};

use actix_cors::Cors;
use actix_web::http::Error;
use actix_web::web::Json;
use actix_web::{http, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use llm::ModelArchitecture;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::utils::match_model_architecture;

mod config;
mod utils;

#[derive(Debug, Deserialize)]
struct PromptRequest {
    message: String,
}

#[derive(Debug, Serialize)]
struct PromptResponse {
    response: String,
}

fn infer(prompt: String) -> String {
    let config: Config = Config::init();

    let tokenizer_source = llm::TokenizerSource::Embedded;
    let model_path = PathBuf::from(&config.llm_model);
    let prompt = prompt.to_string();
    let now = std::time::Instant::now();
    let model = llm::load_dynamic(
        match_model_architecture(&config.llm_model_architecture),
        &model_path,
        tokenizer_source,
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

    let mut result = String::new();
    let mut inference_session = model.start_session(Default::default());
    let inference_session_result = inference_session.infer::<Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(),
        &llm::InferenceRequest {
            prompt: (&prompt).into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: true,
            maximum_token_count: Some(config.llm_inference_max_token_count),
        },
        // OutputRequest
        &mut Default::default(),
        |r| match r {
            llm::InferenceResponse::PromptToken(token)
            | llm::InferenceResponse::InferredToken(token) => {
                std::io::stdout().flush().unwrap();

                result.push_str(&token);

                Ok(llm::InferenceFeedback::Continue)
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );

    match inference_session_result {
        Ok(_) => result,
        Err(err) => format!("Error: {}", err),
    }
}

pub async fn prompt_handler(body: Json<PromptRequest>) -> impl Responder {
    let result = infer(body.message.clone());

    HttpResponse::Ok().body(result)
}

pub async fn health_handler() -> impl Responder {
    "OK!"
}

#[actix_web::main]
#[cfg(feature = "server")]
async fn main() -> std::io::Result<()> {
    //@todo initialize the model as a shared state for subsequent queries
    let config: Config = Config::init();
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
            .route("/prompt", web::post().to(prompt_handler))
            .route("/health", web::get().to(health_handler))
    })
    .bind_openssl(complete_address, ssl_builder)?
    .run()
    .await
}
