### Setup

1.  Install the llm-cli tool globally from the repository.

```sh
    cargo install --git https://github.com/rustformers/llm llm-cli
```

2. Procure LLM models binary files from https://huggingface.co/rustformers

Eg:

```sh
    curl -LO https://huggingface.co/rustformers/open-llama-ggml/resolve/main/open_llama_7b-f16.bin
    curl -LO https://huggingface.co/rustformers/gpt4all-j-ggml/resolve/main/gpt4all-j-q4_0-ggjt.bin
```

3. Test model without a REST server.

```sh
    llm infer -a llama -m open_llama_3b-f16.bin -p "Hello world! Tell me about yourself!"
```
```sh
    llm infer -a gptj  -m gpt4all-j-q4_0-ggjt.bin  -p "Tell me how cool the Rust programming language is:"
```
```sh
    llm infer -a gptj  -m gpt4all-j-q5_1.bin  -p "Tell me how cool the Rust programming language is:"
```

---

### API server

1. Run:

```sh
    cargo run --release
```

2. Test:
```sh
    curl -k -X POST -H "Content-Type: application/json" -d '{"message": "Hello world!"}' https://localhost:8080/prompt   
```

