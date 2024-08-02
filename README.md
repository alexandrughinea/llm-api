# LLM Inference Server

This project implements a secure HTTPS server for running private inference sessions on Large Language Models (LLMs) using Rust and Actix Web.

## Features

- Secure HTTPS server with SSL/TLS support
- CORS-enabled API endpoints
- Configurable LLM model loading and inference
- Health check endpoint
- Server information endpoint
- Generate text based on prompts
- Support for multiple model architectures

## Prerequisites

- Rust programming environment
- OpenSSL library
- SSL certificates (cert.pem and key.pem)

## Configuration

The server is configurable through environment variables or a configuration file. Key configuration options include:

- LLM model path and architecture
- Server address and port
- Allowed origin for CORS
- Maximum token count for inference

Create a file named `.env` in the root directory of your project with the following content:

```env
SERVER_ADDRESS=localhost
SERVER_PORT=8080

SERVER_REQUEST_TIMEOUT_IN_SECONDS=10
MACHINE_COMMAND_TIMEOUT_IN_SECONDS=10

DATABASE_URL=""
MAX_CONNECTIONS=10

ALLOWED_ORIGIN="localhost"
MAX_AGE=4600# 76.67 Minutes = 76 Minutes and 40 Seconds

LLM_MODEL="open_llama_7b-f16.bin"
LLM_MODEL_ARCHITECTURE="llama"
LLM_INFERENCE_MAX_TOKEN_COUNT=400
```

## Model procurement

https://huggingface.co/

## API Endpoints

- `GET /`: Server information
- `POST /api/generate`: Generate text based on a prompt
- `GET /api/health`: Health check endpoint

## Build

To build the project, run:

```bash
cargo build --release --features serve
```


## Run

If everything was configured correctly you should be greeted with something like:

```bash
...
Loaded tensor 288/291
Loading of model complete
Model size = 12853.47 MB / num tensors = 291
open_llama_7b-f16.bin model (llama) has been started!
Elapsed: 96ms
Starting server at https://localhost:8080.
```

Starting the inference session

```bash
curl -k -X POST -H "Content-Type: application/json" -d '{"prompt": "Say hello!"}' https://localhost:8080/api/generate
```

## Performance and Output Considerations

Performance and outputs depend significantly on the model size and hardware capabilities. 
Below are estimated performance metrics for Llama models on various MacBook configurations:

| Model Size | Hardware      | Approx. Inference Time | Memory Usage | Max Tokens/Second |
|------------|---------------|------------------------|--------------|-------------------|
| 7B         | M1 Air (8GB)  | 150-200ms/token        | ~14GB        | ~5-6              |
| 7B         | M1 Pro (16GB) | 120-170ms/token        | ~14GB        | ~6-8              |
| 7B         | M2 Air (16GB) | 100-150ms/token        | ~14GB        | ~7-10             |
| 7B         | M2 Pro (32GB) | 80-130ms/token         | ~14GB        | ~8-12             |
| 13B        | M1 Pro (32GB) | 250-350ms/token        | ~28GB        | ~3-4              |
| 13B        | M2 Pro (32GB) | 200-300ms/token        | ~28GB        | ~3-5              |


Note: These figures are approximate and can vary based on specific configurations, prompt length, optimization techniques, and other factors. Always benchmark on your specific setup for accurate performance metrics.

Key Observations:
1. Larger models (13B, 30B) require more memory and process fewer tokens per second.
2. Newer M-series chips (M2, M3) generally offer better performance for the same model size.
3. Models larger than 7B may not be practical on MacBooks with less than 32GB of RAM.
4. The 30B model is only feasible on high-end configurations with significant RAM.

The quality and coherence of outputs generally improve with larger models, but this comes at the cost of increased computational requirements and slower inference times. Users should balance their specific needs for performance, output quality, and available hardware when selecting a model.

Note: These figures are approximate and can vary based on specific hardware configurations, prompt length, and other factors. Always benchmark on your specific setup for accurate performance metrics.

## Security
This server implements several security measures:

HTTPS: All communication is encrypted using SSL/TLS.
CORS: Cross-Origin Resource Sharing is configured to restrict access to specified origins.
Middleware: The server uses Actix Web's middleware for logging and compression.
SSL/TLS Configuration: The server uses Mozilla's intermediate configuration for SSL/TLS settings.

Always ensure that your SSL certificates are up-to-date and properly configured. Keep your private keys secure and never commit them to version control.

## Model Architectures
This server supports multiple model architectures.
The specific architecture can be configured in the settings. 
Supported architectures are determined by the match_model_architecture function in the utils module.
An exhaustive list would be:

- [x] Llama
- [x] Bloom
- [x] GPT2
- [x] GptJ
- [x] NeoX

## Contributing
Contributions to this project are welcome! Here's how you can contribute:

Fork the repository
Create a new branch for your feature or bug fix
Make your changes and commit them with clear, descriptive messages
Push your changes to your fork
Submit a pull request to the main repository

Please ensure your code adheres to the project's coding standards and include tests for new features or bug fixes.

## Improvements

- [ ] Add a test suite covering server and utilities;
- [ ] Add Docker support;
- [ ] Add inference cancelling support;
- [ ] Add database support for requests history;
- [ ] Add stream support for tokens;

## Disclaimer

**IMPORTANT: READ BEFORE USE**

- This software is provided "as is", without warranty of any kind.
- The authors are not liable for any damages or consequences arising from its use.
- Users are solely responsible for their use of this software and compliance with applicable laws.
- This software is not intended for use in critical systems where failure could cause harm.
- The authors do not support any illegal or unethical use of this software.
- Performance and outputs of language models may be unpredictable.
- Users are responsible for security and should thoroughly test any deployments.

USE OF THIS SOFTWARE INDICATES ACCEPTANCE OF THESE TERMS.