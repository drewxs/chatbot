# Chatbot

An AI chatbot built using Leptos, Actix, and Rustformers with Llama 2 (OSS variant of GPT).

## Requirements

- Nightly Rust toolchain
- `wasm32-unknown-unknown` target
- Trunk and cargo-leptos tools

```sh
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
cargo install trunk cargo-leptos
```

## Setup

### Model

A downloaded model in GGML format supported by [Rustformers/llm](https://github.com/rustformers/llm).

Find one [here](https://huggingface.co/models?search=ggml).

Copy env template:

```
cp .env.example .env
```

Update `MODEL_PATH` with the full path to the downloaded model.

## Development

Launch development server:

```sh
cargo leptos watch
```
