```rust
// Update main.rs to integrate LLM proxy
use axum::{Router, routing::post};
use std::sync::Arc;
use std::env;

mod llm_proxy;
mod matrix;

use llm_proxy::{proxy_llm, ProxyParams, AppState};
use matrix::MatrixState;

#[tokio::main]
async fn main() {
    // Required env vars
    env::set_var("RUST_LOG", "info");
    
    let client = reqwest::Client::new();
    let matrix_state = Arc::new(MatrixState::new().await.unwrap());
    let app_state = Arc::new(AppState {
        client,
        matrix: matrix_state,
    });

    let app = Router::new()
        .route("/api/ai-chat", post(proxy_llm))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Abyssal Nexus listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
```

**Required ENV:**
```bash
export ANTHROPIC_API_KEY=sk-ant-...
export OPENAI_API_KEY=sk-proj-...
export XAI_API_KEY=xai-...
```