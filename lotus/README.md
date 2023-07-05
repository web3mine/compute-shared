# Filecoin Lotus RPC client for rust

## Getting Started

Initialize client by setting in env

"LOTUS_API_HOST_URL"
"LOTUS_HTTP_CONTENT_TYPE"
"LOTUS_API_TOKEN"

```rust
      let client = LotusClient::init(LotusConfig::from_env());
```

or you can pass them as values 

```rust
  let client = LotusClient::init(LotusConfig::new(host_url, content_type, token))
```